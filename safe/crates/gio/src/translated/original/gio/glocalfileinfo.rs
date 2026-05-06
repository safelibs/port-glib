use ::c2rust_bitfields;
extern "C" {
    pub type _GChecksum;
    pub type _GData;
    pub type _GDir;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GSourcePrivate;
    pub type _GVariant;
    pub type _GCancellable;
    pub type _GFile;
    pub type _GFileInfo;
    pub type _GFileAttributeMatcher;
    pub type _GIcon;
    pub type _GWakeup;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_get_home_dir() -> *const gchar;
    fn g_get_user_cache_dir() -> *const gchar;
    fn g_get_user_special_dir(directory: GUserDirectory) -> *const gchar;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn chown(
        __file: *const ::core::ffi::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::core::ffi::c_int;
    fn lchown(
        __file: *const ::core::ffi::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::core::ffi::c_int;
    fn geteuid() -> __uid_t;
    fn symlink(
        __from: *const ::core::ffi::c_char,
        __to: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn readlink(
        __path: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn g_checksum_new(checksum_type: GChecksumType) -> *mut GChecksum;
    fn g_checksum_free(checksum: *mut GChecksum);
    fn g_checksum_update(checksum: *mut GChecksum, data: *const guchar, length: gssize);
    fn g_checksum_get_string(checksum: *mut GChecksum) -> *const gchar;
    fn g_locale_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_filename_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_filename_to_uri(
        filename: *const gchar,
        hostname: *const gchar,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_filename_display_name(filename: *const gchar) -> *mut gchar;
    fn g_filename_display_basename(filename: *const gchar) -> *mut gchar;
    fn g_file_test(filename: *const gchar, test: GFileTest) -> gboolean;
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_build_path(separator: *const gchar, first_element: *const gchar, ...) -> *mut gchar;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_path_get_dirname(file_name: *const gchar) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_replace(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_add(hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_contains(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup_extended(
        hash_table: *mut GHashTable,
        lookup_key: gconstpointer,
        orig_key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_hash_table_iter_init(iter: *mut GHashTableIter, hash_table: *mut GHashTable);
    fn g_hash_table_iter_next(
        iter: *mut GHashTableIter,
        key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_hash_table_iter_remove(iter: *mut GHashTableIter);
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_source_unref(source: *mut GSource);
    fn g_source_attach(source: *mut GSource, context: *mut GMainContext) -> guint;
    fn g_source_set_priority(source: *mut GSource, priority: gint);
    fn g_source_set_callback(
        source: *mut GSource,
        func: GSourceFunc,
        data: gpointer,
        notify: GDestroyNotify,
    );
    fn g_source_set_static_name(source: *mut GSource, name: *const ::core::ffi::c_char);
    fn g_source_get_time(source: *mut GSource) -> gint64;
    fn g_timeout_source_new_seconds(interval: guint) -> *mut GSource;
    fn g_get_monotonic_time() -> gint64;
    fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
    fn g_utf8_validate_len(str: *const gchar, max_len: gsize, end: *mut *const gchar) -> gboolean;
    fn g_ascii_xdigit_value(c: gchar) -> gint;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_strsplit(
        string: *const gchar,
        delimiter: *const gchar,
        max_tokens: gint,
    ) -> *mut *mut gchar;
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
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
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
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn chmod(__file: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn lchmod(__file: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn utimensat(
        __fd: ::core::ffi::c_int,
        __path: *const ::core::ffi::c_char,
        __times: *const timespec,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn statx(
        __dirfd: ::core::ffi::c_int,
        __path: *const ::core::ffi::c_char,
        __flags: ::core::ffi::c_int,
        __mask: ::core::ffi::c_uint,
        __buf: *mut statx,
    ) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn getgrgid_r(
        __gid: __gid_t,
        __resultbuf: *mut group,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut group,
    ) -> ::core::ffi::c_int;
    fn getpwuid_r(
        __uid: __uid_t,
        __resultbuf: *mut passwd,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> ::core::ffi::c_int;
    fn is_selinux_enabled() -> ::core::ffi::c_int;
    fn freecon(con: *mut ::core::ffi::c_char);
    fn getfilecon_raw(
        path: *const ::core::ffi::c_char,
        con: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn lgetfilecon_raw(
        path: *const ::core::ffi::c_char,
        con: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn fgetfilecon_raw(
        fd: ::core::ffi::c_int,
        con: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn setfilecon_raw(
        path: *const ::core::ffi::c_char,
        con: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn setxattr(
        __path: *const ::core::ffi::c_char,
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_void,
        __size: size_t,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn getxattr(
        __path: *const ::core::ffi::c_char,
        __name: *const ::core::ffi::c_char,
        __value: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> ssize_t;
    fn lgetxattr(
        __path: *const ::core::ffi::c_char,
        __name: *const ::core::ffi::c_char,
        __value: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> ssize_t;
    fn fgetxattr(
        __fd: ::core::ffi::c_int,
        __name: *const ::core::ffi::c_char,
        __value: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> ssize_t;
    fn listxattr(
        __path: *const ::core::ffi::c_char,
        __list: *mut ::core::ffi::c_char,
        __size: size_t,
    ) -> ssize_t;
    fn llistxattr(
        __path: *const ::core::ffi::c_char,
        __list: *mut ::core::ffi::c_char,
        __size: size_t,
    ) -> ssize_t;
    fn flistxattr(
        __fd: ::core::ffi::c_int,
        __list: *mut ::core::ffi::c_char,
        __size: size_t,
    ) -> ssize_t;
    fn removexattr(
        __path: *const ::core::ffi::c_char,
        __name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_access(filename: *const gchar, mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn g_unlink(filename: *const gchar) -> ::core::ffi::c_int;
    fn g_close(fd: gint, error: *mut *mut GError) -> gboolean;
    fn g_object_unref(object: gpointer);
    fn g_file_info_new() -> *mut GFileInfo;
    fn g_file_info_get_attribute_status(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
    ) -> GFileAttributeStatus;
    fn g_file_info_set_attribute(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
        type_0: GFileAttributeType,
        value_p: gpointer,
    );
    fn g_file_info_set_attribute_string(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
        attr_value: *const ::core::ffi::c_char,
    );
    fn g_file_info_set_attribute_mask(info: *mut GFileInfo, mask: *mut GFileAttributeMatcher);
    fn g_file_info_unset_attribute_mask(info: *mut GFileInfo);
    fn g_file_info_set_file_type(info: *mut GFileInfo, type_0: GFileType);
    fn g_file_info_set_is_hidden(info: *mut GFileInfo, is_hidden: gboolean);
    fn g_file_info_set_is_symlink(info: *mut GFileInfo, is_symlink: gboolean);
    fn g_file_info_set_name(info: *mut GFileInfo, name: *const ::core::ffi::c_char);
    fn g_file_info_set_display_name(info: *mut GFileInfo, display_name: *const ::core::ffi::c_char);
    fn g_file_info_set_edit_name(info: *mut GFileInfo, edit_name: *const ::core::ffi::c_char);
    fn g_file_info_set_icon(info: *mut GFileInfo, icon: *mut GIcon);
    fn g_file_info_set_symbolic_icon(info: *mut GFileInfo, icon: *mut GIcon);
    fn g_file_info_set_content_type(info: *mut GFileInfo, content_type: *const ::core::ffi::c_char);
    fn g_file_info_set_size(info: *mut GFileInfo, size: goffset);
    fn g_file_info_set_symlink_target(
        info: *mut GFileInfo,
        symlink_target: *const ::core::ffi::c_char,
    );
    fn g_file_attribute_matcher_new(
        attributes: *const ::core::ffi::c_char,
    ) -> *mut GFileAttributeMatcher;
    fn g_file_attribute_matcher_unref(matcher: *mut GFileAttributeMatcher);
    fn g_file_attribute_matcher_enumerate_namespace(
        matcher: *mut GFileAttributeMatcher,
        ns: *const ::core::ffi::c_char,
    ) -> gboolean;
    fn g_file_attribute_matcher_enumerate_next(
        matcher: *mut GFileAttributeMatcher,
    ) -> *const ::core::ffi::c_char;
    fn _g_file_attribute_value_set_from_pointer(
        attr: *mut GFileAttributeValue,
        type_0: GFileAttributeType,
        value_p: gpointer,
        dup: gboolean,
    );
    fn _g_file_info_get_attribute_value(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
    ) -> *mut GFileAttributeValue;
    fn _g_file_attribute_matcher_matches_id(
        matcher: *mut GFileAttributeMatcher,
        id: guint32,
    ) -> gboolean;
    fn _g_file_info_set_attribute_string_by_id(
        info: *mut GFileInfo,
        attribute: guint32,
        attr_value: *const ::core::ffi::c_char,
    );
    fn _g_file_info_set_attribute_byte_string_by_id(
        info: *mut GFileInfo,
        attribute: guint32,
        attr_value: *const ::core::ffi::c_char,
    );
    fn _g_file_info_set_attribute_boolean_by_id(
        info: *mut GFileInfo,
        attribute: guint32,
        attr_value: gboolean,
    );
    fn _g_file_info_set_attribute_uint32_by_id(
        info: *mut GFileInfo,
        attribute: guint32,
        attr_value: guint32,
    );
    fn _g_file_info_set_attribute_uint64_by_id(
        info: *mut GFileInfo,
        attribute: guint32,
        attr_value: guint64,
    );
    fn g_vfs_get_default() -> *mut GVfs;
    fn glib__private__() -> *const GLibPrivateVTable;
    fn _g_local_file_has_trash_dir(dirname: *const ::core::ffi::c_char, dir_dev: dev_t)
        -> gboolean;
    fn _g_local_file_is_lost_found_dir(
        path: *const ::core::ffi::c_char,
        path_dev: dev_t,
    ) -> gboolean;
    fn thumbnail_verify(
        thumbnail_path: *const gchar,
        file_uri: *const gchar,
        file_stat_buf: *const statx,
    ) -> gboolean;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn g_themed_icon_new(iconname: *const ::core::ffi::c_char) -> *mut GIcon;
    fn g_themed_icon_new_with_default_fallbacks(iconname: *const ::core::ffi::c_char)
        -> *mut GIcon;
    fn g_content_type_get_icon(type_0: *const gchar) -> *mut GIcon;
    fn g_content_type_get_symbolic_icon(type_0: *const gchar) -> *mut GIcon;
    fn g_content_type_from_mime_type(mime_type: *const gchar) -> *mut gchar;
    fn g_content_type_guess(
        filename: *const gchar,
        data: *const guchar,
        data_size: gsize,
        result_uncertain: *mut gboolean,
    ) -> *mut gchar;
    fn _g_unix_content_type_get_sniff_len() -> gsize;
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type goffset = gint64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __ino64_t = ::core::ffi::c_ulong;
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
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type GUserDirectory = ::core::ffi::c_uint;
pub const G_USER_N_DIRECTORIES: GUserDirectory = 8;
pub const G_USER_DIRECTORY_VIDEOS: GUserDirectory = 7;
pub const G_USER_DIRECTORY_TEMPLATES: GUserDirectory = 6;
pub const G_USER_DIRECTORY_PUBLIC_SHARE: GUserDirectory = 5;
pub const G_USER_DIRECTORY_PICTURES: GUserDirectory = 4;
pub const G_USER_DIRECTORY_MUSIC: GUserDirectory = 3;
pub const G_USER_DIRECTORY_DOWNLOAD: GUserDirectory = 2;
pub const G_USER_DIRECTORY_DOCUMENTS: GUserDirectory = 1;
pub const G_USER_DIRECTORY_DESKTOP: GUserDirectory = 0;
pub type ino_t = __ino64_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type ssize_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GChecksumType = ::core::ffi::c_uint;
pub const G_CHECKSUM_SHA384: GChecksumType = 4;
pub const G_CHECKSUM_SHA512: GChecksumType = 3;
pub const G_CHECKSUM_SHA256: GChecksumType = 2;
pub const G_CHECKSUM_SHA1: GChecksumType = 1;
pub const G_CHECKSUM_MD5: GChecksumType = 0;
pub type GChecksum = _GChecksum;
pub type GData = _GData;
pub type GDir = _GDir;
pub type GFileTest = ::core::ffi::c_uint;
pub const G_FILE_TEST_EXISTS: GFileTest = 16;
pub const G_FILE_TEST_IS_EXECUTABLE: GFileTest = 8;
pub const G_FILE_TEST_IS_DIR: GFileTest = 4;
pub const G_FILE_TEST_IS_SYMLINK: GFileTest = 2;
pub const G_FILE_TEST_IS_REGULAR: GFileTest = 1;
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
pub type GVariant = _GVariant;
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
pub type __u16 = ::core::ffi::c_ushort;
pub type __s32 = ::core::ffi::c_int;
pub type __u32 = ::core::ffi::c_uint;
pub type __s64 = ::core::ffi::c_longlong;
pub type __u64 = ::core::ffi::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statx_timestamp {
    pub tv_sec: __s64,
    pub tv_nsec: __u32,
    pub __reserved: __s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statx {
    pub stx_mask: __u32,
    pub stx_blksize: __u32,
    pub stx_attributes: __u64,
    pub stx_nlink: __u32,
    pub stx_uid: __u32,
    pub stx_gid: __u32,
    pub stx_mode: __u16,
    pub __spare0: [__u16; 1],
    pub stx_ino: __u64,
    pub stx_size: __u64,
    pub stx_blocks: __u64,
    pub stx_attributes_mask: __u64,
    pub stx_atime: statx_timestamp,
    pub stx_btime: statx_timestamp,
    pub stx_ctime: statx_timestamp,
    pub stx_mtime: statx_timestamp,
    pub stx_rdev_major: __u32,
    pub stx_rdev_minor: __u32,
    pub stx_dev_major: __u32,
    pub stx_dev_minor: __u32,
    pub stx_mnt_id: __u64,
    pub stx_dio_mem_align: __u32,
    pub stx_dio_offset_align: __u32,
    pub __spare3: [__u64; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut ::core::ffi::c_char,
    pub gr_passwd: *mut ::core::ffi::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut ::core::ffi::c_char,
    pub pw_passwd: *mut ::core::ffi::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut ::core::ffi::c_char,
    pub pw_dir: *mut ::core::ffi::c_char,
    pub pw_shell: *mut ::core::ffi::c_char,
}
pub type GStatBuf = stat;
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
pub type GFileAttributeType = ::core::ffi::c_uint;
pub const G_FILE_ATTRIBUTE_TYPE_STRINGV: GFileAttributeType = 9;
pub const G_FILE_ATTRIBUTE_TYPE_OBJECT: GFileAttributeType = 8;
pub const G_FILE_ATTRIBUTE_TYPE_INT64: GFileAttributeType = 7;
pub const G_FILE_ATTRIBUTE_TYPE_UINT64: GFileAttributeType = 6;
pub const G_FILE_ATTRIBUTE_TYPE_INT32: GFileAttributeType = 5;
pub const G_FILE_ATTRIBUTE_TYPE_UINT32: GFileAttributeType = 4;
pub const G_FILE_ATTRIBUTE_TYPE_BOOLEAN: GFileAttributeType = 3;
pub const G_FILE_ATTRIBUTE_TYPE_BYTE_STRING: GFileAttributeType = 2;
pub const G_FILE_ATTRIBUTE_TYPE_STRING: GFileAttributeType = 1;
pub const G_FILE_ATTRIBUTE_TYPE_INVALID: GFileAttributeType = 0;
pub type GFileAttributeInfoFlags = ::core::ffi::c_uint;
pub const G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED: GFileAttributeInfoFlags = 2;
pub const G_FILE_ATTRIBUTE_INFO_COPY_WITH_FILE: GFileAttributeInfoFlags = 1;
pub const G_FILE_ATTRIBUTE_INFO_NONE: GFileAttributeInfoFlags = 0;
pub type GFileAttributeStatus = ::core::ffi::c_uint;
pub const G_FILE_ATTRIBUTE_STATUS_ERROR_SETTING: GFileAttributeStatus = 2;
pub const G_FILE_ATTRIBUTE_STATUS_SET: GFileAttributeStatus = 1;
pub const G_FILE_ATTRIBUTE_STATUS_UNSET: GFileAttributeStatus = 0;
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
pub type GCancellable = _GCancellable;
pub type GFile = _GFile;
pub type GFileInfo = _GFileInfo;
pub type GFileAttributeMatcher = _GFileAttributeMatcher;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileAttributeInfo {
    pub name: *mut ::core::ffi::c_char,
    pub type_0: GFileAttributeType,
    pub flags: GFileAttributeInfoFlags,
}
pub type GFileAttributeInfo = _GFileAttributeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileAttributeInfoList {
    pub infos: *mut GFileAttributeInfo,
    pub n_infos: ::core::ffi::c_int,
}
pub type GFileAttributeInfoList = _GFileAttributeInfoList;
pub type GIcon = _GIcon;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVfs {
    pub parent_instance: GObject,
}
pub type GVfs = _GVfs;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct GFileAttributeValue {
    #[bitfield(name = "type_0", ty = "guint", bits = "0..=7")]
    #[bitfield(name = "status", ty = "guint", bits = "8..=15")]
    pub type_0_status: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub boolean: gboolean,
    pub int32: gint32,
    pub uint32: guint32,
    pub int64: gint64,
    pub uint64: guint64,
    pub string: *mut ::core::ffi::c_char,
    pub obj: *mut GObject,
    pub stringv: *mut *mut ::core::ffi::c_char,
}
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
pub type GWakeup = _GWakeup;
pub type GDataListUpdateAtomicFunc =
    Option<unsafe extern "C" fn(GQuark, *mut gpointer, *mut GDestroyNotify, gpointer) -> gpointer>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GWin32InvalidParameterHandler {
    pub unused_really: ::core::ffi::c_int,
}
pub type GWin32InvalidParameterHandler = _GWin32InvalidParameterHandler;
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
pub type GLocalFileStatField = ::core::ffi::c_uint;
pub const G_LOCAL_FILE_STAT_FIELD_BTIME: GLocalFileStatField = 2048;
pub const G_LOCAL_FILE_STAT_FIELD_BLOCKS: GLocalFileStatField = 1024;
pub const G_LOCAL_FILE_STAT_FIELD_SIZE: GLocalFileStatField = 512;
pub const G_LOCAL_FILE_STAT_FIELD_INO: GLocalFileStatField = 256;
pub const G_LOCAL_FILE_STAT_FIELD_CTIME: GLocalFileStatField = 128;
pub const G_LOCAL_FILE_STAT_FIELD_MTIME: GLocalFileStatField = 64;
pub const G_LOCAL_FILE_STAT_FIELD_ATIME: GLocalFileStatField = 32;
pub const G_LOCAL_FILE_STAT_FIELD_GID: GLocalFileStatField = 16;
pub const G_LOCAL_FILE_STAT_FIELD_UID: GLocalFileStatField = 8;
pub const G_LOCAL_FILE_STAT_FIELD_NLINK: GLocalFileStatField = 4;
pub const G_LOCAL_FILE_STAT_FIELD_MODE: GLocalFileStatField = 2;
pub const G_LOCAL_FILE_STAT_FIELD_TYPE: GLocalFileStatField = 1;
pub type ThumbnailSize = ::core::ffi::c_uint;
pub const THUMBNAIL_SIZE_LAST: ThumbnailSize = 5;
pub const THUMBNAIL_SIZE_XXLARGE: ThumbnailSize = 4;
pub const THUMBNAIL_SIZE_XLARGE: ThumbnailSize = 3;
pub const THUMBNAIL_SIZE_LARGE: ThumbnailSize = 2;
pub const THUMBNAIL_SIZE_NORMAL: ThumbnailSize = 1;
pub const THUMBNAIL_SIZE_AUTO: ThumbnailSize = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UidData {
    pub user_name: *mut ::core::ffi::c_char,
    pub real_name: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HiddenCacheData {
    pub hidden_files: *mut GHashTable,
    pub timestamp_secs: gint64,
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_ISVTX: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const R_OK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const W_OK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const X_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const G_PRIORITY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const G_SOURCE_REMOVE: ::core::ffi::c_int = FALSE;
pub const G_SOURCE_CONTINUE: ::core::ffi::c_int = TRUE;
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
pub const G_USEC_PER_SEC: ::core::ffi::c_int = 1000000 as ::core::ffi::c_int;
pub const STATX_BASIC_STATS: ::core::ffi::c_uint = 0x7ff as ::core::ffi::c_uint;
pub const STATX_ALL: ::core::ffi::c_uint = 0xfff as ::core::ffi::c_uint;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const __O_NOATIME: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const O_NOATIME: ::core::ffi::c_int = __O_NOATIME;
pub const S_ISVTX: ::core::ffi::c_int = __S_ISVTX;
pub const AT_FDCWD: ::core::ffi::c_int = -(100 as ::core::ffi::c_int);
pub const AT_SYMLINK_NOFOLLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const AT_NO_AUTOMOUNT: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const AT_EMPTY_PATH: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const AT_STATX_SYNC_AS_STAT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"standard::symlink-target\0")
};
pub const G_FILE_ATTRIBUTE_TIME_MODIFIED: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"time::modified\0") };
pub const G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"time::modified-usec\0")
};
pub const G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"time::modified-nsec\0")
};
pub const G_FILE_ATTRIBUTE_TIME_ACCESS: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"time::access\0") };
pub const G_FILE_ATTRIBUTE_TIME_ACCESS_USEC: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"time::access-usec\0")
};
pub const G_FILE_ATTRIBUTE_TIME_ACCESS_NSEC: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"time::access-nsec\0")
};
pub const G_FILE_ATTRIBUTE_UNIX_MODE: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"unix::mode\0") };
pub const G_FILE_ATTRIBUTE_UNIX_UID: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"unix::uid\0") };
pub const G_FILE_ATTRIBUTE_UNIX_GID: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"unix::gid\0") };
pub const G_FILE_ATTRIBUTE_SELINUX_CONTEXT: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"selinux::context\0")
};
pub const G_FILE_ATTRIBUTE_ID_STANDARD_IS_HIDDEN: ::core::ffi::c_int =
    1048576 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_STANDARD_IS_BACKUP: ::core::ffi::c_int =
    1048576 as ::core::ffi::c_int + 3 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_STANDARD_DISPLAY_NAME: ::core::ffi::c_int =
    1048576 as ::core::ffi::c_int + 7 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_STANDARD_EDIT_NAME: ::core::ffi::c_int =
    1048576 as ::core::ffi::c_int + 8 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_STANDARD_COPY_NAME: ::core::ffi::c_int =
    1048576 as ::core::ffi::c_int + 9 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_STANDARD_ICON: ::core::ffi::c_int =
    1048576 as ::core::ffi::c_int + 11 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_STANDARD_CONTENT_TYPE: ::core::ffi::c_int =
    1048576 as ::core::ffi::c_int + 12 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_STANDARD_FAST_CONTENT_TYPE: ::core::ffi::c_int =
    1048576 as ::core::ffi::c_int + 13 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_STANDARD_ALLOCATED_SIZE: ::core::ffi::c_int =
    1048576 as ::core::ffi::c_int + 15 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_STANDARD_SYMLINK_TARGET: ::core::ffi::c_int =
    1048576 as ::core::ffi::c_int + 16 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_STANDARD_SYMBOLIC_ICON: ::core::ffi::c_int =
    1048576 as ::core::ffi::c_int + 19 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ETAG_VALUE: ::core::ffi::c_int =
    2097152 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ID_FILE: ::core::ffi::c_int =
    3145728 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ID_FILESYSTEM: ::core::ffi::c_int =
    3145728 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ACCESS_CAN_READ: ::core::ffi::c_int =
    4194304 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ACCESS_CAN_WRITE: ::core::ffi::c_int =
    4194304 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ACCESS_CAN_EXECUTE: ::core::ffi::c_int =
    4194304 as ::core::ffi::c_int + 3 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ACCESS_CAN_DELETE: ::core::ffi::c_int =
    4194304 as ::core::ffi::c_int + 4 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ACCESS_CAN_TRASH: ::core::ffi::c_int =
    4194304 as ::core::ffi::c_int + 5 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_ACCESS_CAN_RENAME: ::core::ffi::c_int =
    4194304 as ::core::ffi::c_int + 6 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_TIME_MODIFIED: ::core::ffi::c_int =
    6291456 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_TIME_MODIFIED_USEC: ::core::ffi::c_int =
    6291456 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_TIME_ACCESS: ::core::ffi::c_int =
    6291456 as ::core::ffi::c_int + 3 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_TIME_ACCESS_USEC: ::core::ffi::c_int =
    6291456 as ::core::ffi::c_int + 4 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_TIME_CHANGED: ::core::ffi::c_int =
    6291456 as ::core::ffi::c_int + 5 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_TIME_CHANGED_USEC: ::core::ffi::c_int =
    6291456 as ::core::ffi::c_int + 6 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_TIME_CREATED: ::core::ffi::c_int =
    6291456 as ::core::ffi::c_int + 7 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_TIME_CREATED_USEC: ::core::ffi::c_int =
    6291456 as ::core::ffi::c_int + 8 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_TIME_MODIFIED_NSEC: ::core::ffi::c_int =
    6291456 as ::core::ffi::c_int + 9 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_TIME_ACCESS_NSEC: ::core::ffi::c_int =
    6291456 as ::core::ffi::c_int + 10 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_TIME_CREATED_NSEC: ::core::ffi::c_int =
    6291456 as ::core::ffi::c_int + 11 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_TIME_CHANGED_NSEC: ::core::ffi::c_int =
    6291456 as ::core::ffi::c_int + 12 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_UNIX_DEVICE: ::core::ffi::c_int =
    7340032 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_UNIX_INODE: ::core::ffi::c_int =
    7340032 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_UNIX_MODE: ::core::ffi::c_int =
    7340032 as ::core::ffi::c_int + 3 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_UNIX_NLINK: ::core::ffi::c_int =
    7340032 as ::core::ffi::c_int + 4 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_UNIX_UID: ::core::ffi::c_int =
    7340032 as ::core::ffi::c_int + 5 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_UNIX_GID: ::core::ffi::c_int =
    7340032 as ::core::ffi::c_int + 6 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_UNIX_RDEV: ::core::ffi::c_int =
    7340032 as ::core::ffi::c_int + 7 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_UNIX_BLOCK_SIZE: ::core::ffi::c_int =
    7340032 as ::core::ffi::c_int + 8 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_UNIX_BLOCKS: ::core::ffi::c_int =
    7340032 as ::core::ffi::c_int + 9 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_UNIX_IS_MOUNTPOINT: ::core::ffi::c_int =
    7340032 as ::core::ffi::c_int + 10 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_OWNER_USER: ::core::ffi::c_int =
    9437184 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_OWNER_USER_REAL: ::core::ffi::c_int =
    9437184 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_OWNER_GROUP: ::core::ffi::c_int =
    9437184 as ::core::ffi::c_int + 3 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 3 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_NORMAL: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 4 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_NORMAL: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 5 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_NORMAL: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 6 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_LARGE: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 7 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_LARGE: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 8 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_LARGE: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 9 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_XLARGE: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 10 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_XLARGE: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 11 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_XLARGE: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 12 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_XXLARGE: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 13 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_XXLARGE: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 14 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_XXLARGE: ::core::ffi::c_int =
    10485760 as ::core::ffi::c_int + 15 as ::core::ffi::c_int;
pub const G_FILE_ATTRIBUTE_ID_SELINUX_CONTEXT: ::core::ffi::c_int =
    14680064 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_gnu_dev_makedev(
    mut __major: ::core::ffi::c_uint,
    mut __minor: ::core::ffi::c_uint,
) -> __dev_t {
    let mut __dev: __dev_t = 0;
    __dev = ((__major & 0xfff as ::core::ffi::c_uint) as __dev_t) << 8 as ::core::ffi::c_int;
    __dev |= ((__major & 0xfffff000 as ::core::ffi::c_uint) as __dev_t) << 32 as ::core::ffi::c_int;
    __dev |= ((__minor & 0xff as ::core::ffi::c_uint) as __dev_t) << 0 as ::core::ffi::c_int;
    __dev |= ((__minor & 0xffffff00 as ::core::ffi::c_uint) as __dev_t) << 12 as ::core::ffi::c_int;
    return __dev;
}
pub const G_LOCAL_FILE_STAT_FIELD_BASIC_STATS: ::core::ffi::c_uint = STATX_BASIC_STATS;
pub const G_LOCAL_FILE_STAT_FIELD_ALL: ::core::ffi::c_uint = STATX_ALL;
#[inline]
unsafe extern "C" fn safe_c2rust_g_local_file_statx(
    mut dirfd: ::core::ffi::c_int,
    mut pathname: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut mask: GLocalFileStatField,
    mut mask_required: GLocalFileStatField,
    mut stat_buf: *mut statx,
) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = 0;
    mask_required = ::core::mem::transmute::<::core::ffi::c_uint, GLocalFileStatField>(
        mask_required as ::core::ffi::c_uint & mask as ::core::ffi::c_uint,
    );
    retval = statx(
        dirfd,
        pathname,
        flags,
        mask as ::core::ffi::c_uint,
        stat_buf,
    );
    if retval == 0 as ::core::ffi::c_int
        && (*stat_buf).stx_mask as ::core::ffi::c_uint & mask_required as ::core::ffi::c_uint
            != mask_required as ::core::ffi::c_uint
    {
        *__errno_location() = ERANGE;
        return -(1 as ::core::ffi::c_int);
    }
    return retval;
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_local_file_fstat(
    mut fd: ::core::ffi::c_int,
    mut mask: GLocalFileStatField,
    mut mask_required: GLocalFileStatField,
    mut stat_buf: *mut statx,
) -> ::core::ffi::c_int {
    return safe_c2rust_g_local_file_statx(
        fd,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        AT_EMPTY_PATH,
        mask,
        mask_required,
        stat_buf,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_local_file_lstat(
    mut path: *const ::core::ffi::c_char,
    mut mask: GLocalFileStatField,
    mut mask_required: GLocalFileStatField,
    mut stat_buf: *mut statx,
) -> ::core::ffi::c_int {
    return safe_c2rust_g_local_file_statx(
        AT_FDCWD,
        path,
        AT_NO_AUTOMOUNT | AT_SYMLINK_NOFOLLOW | AT_STATX_SYNC_AS_STAT,
        mask,
        mask_required,
        stat_buf,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust_g_local_file_stat(
    mut path: *const ::core::ffi::c_char,
    mut mask: GLocalFileStatField,
    mut mask_required: GLocalFileStatField,
    mut stat_buf: *mut statx,
) -> ::core::ffi::c_int {
    return safe_c2rust_g_local_file_statx(
        AT_FDCWD,
        path,
        AT_NO_AUTOMOUNT | AT_STATX_SYNC_AS_STAT,
        mask,
        mask_required,
        stat_buf,
    );
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_has_field(
    mut buf: *const statx,
    mut field: GLocalFileStatField,
) -> gboolean {
    return ((*buf).stx_mask as ::core::ffi::c_uint & field as ::core::ffi::c_uint) as gboolean;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_mode(mut buf: *const statx) -> guint16 {
    return (*buf).stx_mode as guint16;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_nlink(mut buf: *const statx) -> guint32 {
    return (*buf).stx_nlink as guint32;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_dev(mut buf: *const statx) -> dev_t {
    return safe_c2rust_gnu_dev_makedev(
        (*buf).stx_dev_major as ::core::ffi::c_uint,
        (*buf).stx_dev_minor as ::core::ffi::c_uint,
    ) as dev_t;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_ino(mut buf: *const statx) -> guint64 {
    return (*buf).stx_ino as guint64;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_size(mut buf: *const statx) -> guint64 {
    return (*buf).stx_size as guint64;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_uid(mut buf: *const statx) -> guint32 {
    return (*buf).stx_uid as guint32;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_gid(mut buf: *const statx) -> guint32 {
    return (*buf).stx_gid as guint32;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_rdev(mut buf: *const statx) -> dev_t {
    return safe_c2rust_gnu_dev_makedev(
        (*buf).stx_rdev_major as ::core::ffi::c_uint,
        (*buf).stx_rdev_minor as ::core::ffi::c_uint,
    ) as dev_t;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_blksize(mut buf: *const statx) -> guint32 {
    return (*buf).stx_blksize as guint32;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_blocks(mut buf: *const statx) -> guint64 {
    return (*buf).stx_blocks as guint64;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_atime(mut buf: *const statx) -> gint64 {
    return (*buf).stx_atime.tv_sec as gint64;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_ctime(mut buf: *const statx) -> gint64 {
    return (*buf).stx_ctime.tv_sec as gint64;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_mtime(mut buf: *const statx) -> gint64 {
    return (*buf).stx_mtime.tv_sec as gint64;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_atim_nsec(mut buf: *const statx) -> guint32 {
    return (*buf).stx_atime.tv_nsec as guint32;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_ctim_nsec(mut buf: *const statx) -> guint32 {
    return (*buf).stx_ctime.tv_nsec as guint32;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_mtim_nsec(mut buf: *const statx) -> guint32 {
    return (*buf).stx_mtime.tv_nsec as guint32;
}
static mut safe_c2rust_g__uid_cache_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_uid_cache: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut safe_c2rust_g__gid_cache_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_gid_cache: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_info_create_etag(
    mut statbuf: *mut statx,
) -> *mut ::core::ffi::c_char {
    let mut sec: glong = 0;
    let mut usec: glong = 0;
    let mut nsec: glong = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if safe_c2rust__g_stat_has_field(statbuf, G_LOCAL_FILE_STAT_FIELD_MTIME) != 0 {
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
            b"_g_stat_has_field (statbuf, G_LOCAL_FILE_STAT_FIELD_MTIME)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    sec = safe_c2rust__g_stat_mtime(statbuf) as glong;
    usec = safe_c2rust__g_stat_mtim_nsec(statbuf).wrapping_div(1000 as guint32) as glong;
    nsec = safe_c2rust__g_stat_mtim_nsec(statbuf) as glong;
    return g_strdup_printf(
        b"%lu:%lu:%lu\0" as *const u8 as *const gchar,
        sec,
        usec,
        nsec,
    ) as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust__g_local_file_info_create_file_id(
    mut statbuf: *mut statx,
) -> *mut ::core::ffi::c_char {
    let mut ino: guint64 = 0;
    ino = safe_c2rust__g_stat_ino(statbuf);
    return g_strdup_printf(
        b"l%lu:%lu\0" as *const u8 as *const gchar,
        safe_c2rust__g_stat_dev(statbuf),
        ino,
    ) as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust__g_local_file_info_create_fs_id(
    mut statbuf: *mut statx,
) -> *mut ::core::ffi::c_char {
    return g_strdup_printf(
        b"l%lu\0" as *const u8 as *const gchar,
        safe_c2rust__g_stat_dev(statbuf),
    ) as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_read_link(mut full_name: *const gchar) -> *mut gchar {
    let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut size: gsize = 0;
    size = 256 as gsize;
    buffer = g_malloc(size) as *mut gchar;
    loop {
        let mut read_size: gssize = 0;
        read_size = readlink(
            full_name as *const ::core::ffi::c_char,
            buffer as *mut ::core::ffi::c_char,
            size as size_t,
        ) as gssize;
        if read_size < 0 as gssize {
            g_free(buffer as gpointer);
            return ::core::ptr::null_mut::<gchar>();
        }
        if (read_size as gsize) < size {
            *buffer.offset(read_size as isize) = 0 as gchar;
            return buffer;
        }
        size = size.wrapping_mul(2 as gsize);
        buffer = g_realloc(buffer as gpointer, size) as *mut gchar;
    }
}
unsafe extern "C" fn safe_c2rust_get_selinux_context(
    mut path: *const ::core::ffi::c_char,
    mut info: *mut GFileInfo,
    mut attribute_matcher: *mut GFileAttributeMatcher,
    mut follow_symlinks: gboolean,
) {
    let mut context: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_SELINUX_CONTEXT as guint32,
    ) == 0
    {
        return;
    }
    if is_selinux_enabled() != 0 {
        if follow_symlinks != 0 {
            if lgetfilecon_raw(path, &raw mut context) < 0 as ::core::ffi::c_int {
                return;
            }
        } else if getfilecon_raw(path, &raw mut context) < 0 as ::core::ffi::c_int {
            return;
        }
        if !context.is_null() {
            _g_file_info_set_attribute_string_by_id(
                info,
                G_FILE_ATTRIBUTE_ID_SELINUX_CONTEXT as guint32,
                context,
            );
            freecon(context);
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_getxattr(
    mut path: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut value: *mut ::core::ffi::c_void,
    mut size: size_t,
    mut follow_symlinks: gboolean,
) -> gssize {
    if follow_symlinks != 0 {
        return getxattr(path, name, value, size) as gssize;
    } else {
        return lgetxattr(path, name, value, size) as gssize;
    };
}
unsafe extern "C" fn safe_c2rust_g_listxattr(
    mut path: *const ::core::ffi::c_char,
    mut namebuf: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut follow_symlinks: gboolean,
) -> gssize {
    if follow_symlinks != 0 {
        return listxattr(path, namebuf, size) as gssize;
    } else {
        return llistxattr(path, namebuf, size) as gssize;
    };
}
unsafe extern "C" fn safe_c2rust_valid_char(mut c: ::core::ffi::c_char) -> gboolean {
    return (c as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
        && c as ::core::ffi::c_int <= 126 as ::core::ffi::c_int
        && c as ::core::ffi::c_int != '\\' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_name_is_valid(mut str: *const ::core::ffi::c_char) -> gboolean {
    while *str != 0 {
        let fresh8 = str;
        str = str.offset(1);
        if safe_c2rust_valid_char(*fresh8) == 0 {
            return FALSE;
        }
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_hex_escape_buffer(
    mut str: *const ::core::ffi::c_char,
    mut len: size_t,
    mut free_return: *mut gboolean,
) -> *mut ::core::ffi::c_char {
    let mut num_invalid: size_t = 0;
    let mut i: size_t = 0;
    let mut escaped_str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_uchar = 0;
    static mut safe_c2rust_hex_digits: *mut ::core::ffi::c_char = b"0123456789abcdef\0" as *const u8
        as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char;
    num_invalid = 0 as size_t;
    i = 0 as size_t;
    while i < len {
        if safe_c2rust_valid_char(*str.offset(i as isize)) == 0 {
            num_invalid = num_invalid.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    if num_invalid == 0 as size_t {
        *free_return = FALSE as gboolean;
        return str as *mut ::core::ffi::c_char;
    }
    escaped_str = g_malloc(
        (len as gsize)
            .wrapping_add((num_invalid as gsize).wrapping_mul(3 as gsize))
            .wrapping_add(1 as gsize),
    ) as *mut ::core::ffi::c_char;
    p = escaped_str;
    i = 0 as size_t;
    while i < len {
        if safe_c2rust_valid_char(*str.offset(i as isize)) != 0 {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = *str.offset(i as isize);
        } else {
            c = *str.offset(i as isize) as ::core::ffi::c_uchar;
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = '\\' as i32 as ::core::ffi::c_char;
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = 'x' as i32 as ::core::ffi::c_char;
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = *safe_c2rust_hex_digits.offset(
                (c as ::core::ffi::c_int >> 4 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                    as isize,
            );
            let fresh4 = p;
            p = p.offset(1);
            *fresh4 = *safe_c2rust_hex_digits
                .offset((c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as isize);
        }
        i = i.wrapping_add(1);
    }
    *p = 0 as ::core::ffi::c_char;
    *free_return = TRUE as gboolean;
    return escaped_str;
}
unsafe extern "C" fn safe_c2rust_hex_escape_string(
    mut str: *const ::core::ffi::c_char,
    mut free_return: *mut gboolean,
) -> *mut ::core::ffi::c_char {
    return safe_c2rust_hex_escape_buffer(str, strlen(str), free_return);
}
unsafe extern "C" fn safe_c2rust_hex_unescape_string(
    mut str: *const ::core::ffi::c_char,
    mut out_len: *mut ::core::ffi::c_int,
    mut free_return: *mut gboolean,
) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    let mut unescaped_str: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_uchar = 0;
    let mut len: ::core::ffi::c_int = 0;
    len = strlen(str) as ::core::ffi::c_int;
    if strchr(str, '\\' as i32).is_null() {
        if !out_len.is_null() {
            *out_len = len;
        }
        *free_return = FALSE as gboolean;
        return str as *mut ::core::ffi::c_char;
    }
    unescaped_str = g_malloc((len + 1 as ::core::ffi::c_int) as gsize) as *mut ::core::ffi::c_char;
    p = unescaped_str;
    i = 0 as ::core::ffi::c_int;
    while i < len {
        if *str.offset(i as isize) as ::core::ffi::c_int == '\\' as i32
            && *str.offset((i + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 'x' as i32
            && len - i >= 4 as ::core::ffi::c_int
        {
            c = (g_ascii_xdigit_value(*str.offset((i + 2 as ::core::ffi::c_int) as isize) as gchar)
                << 4 as ::core::ffi::c_int
                | g_ascii_xdigit_value(*str.offset((i + 3 as ::core::ffi::c_int) as isize) as gchar))
                as ::core::ffi::c_uchar;
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = c as ::core::ffi::c_char;
            i += 3 as ::core::ffi::c_int;
        } else {
            let fresh6 = p;
            p = p.offset(1);
            *fresh6 = *str.offset(i as isize);
        }
        i += 1;
    }
    if !out_len.is_null() {
        *out_len = p.offset_from(unescaped_str) as ::core::ffi::c_long as ::core::ffi::c_int;
    }
    let fresh7 = p;
    p = p.offset(1);
    *fresh7 = 0 as ::core::ffi::c_char;
    *free_return = TRUE as gboolean;
    return unescaped_str;
}
unsafe extern "C" fn safe_c2rust_escape_xattr(
    mut info: *mut GFileInfo,
    mut gio_attr: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
    mut len: size_t,
) {
    let mut escaped_val: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut free_escaped_val: gboolean = 0;
    escaped_val = safe_c2rust_hex_escape_buffer(value, len, &raw mut free_escaped_val);
    g_file_info_set_attribute_string(info, gio_attr, escaped_val);
    if free_escaped_val != 0 {
        g_free(escaped_val as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_get_one_xattr(
    mut path: *const ::core::ffi::c_char,
    mut info: *mut GFileInfo,
    mut gio_attr: *const ::core::ffi::c_char,
    mut xattr: *const ::core::ffi::c_char,
    mut follow_symlinks: gboolean,
) {
    let mut value: [::core::ffi::c_char; 64] = [0; 64];
    let mut value_p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: gssize = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    len = safe_c2rust_g_getxattr(
        path,
        xattr,
        &raw mut value as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t).wrapping_sub(1 as size_t),
        follow_symlinks,
    );
    errsv = *__errno_location();
    value_p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if len >= 0 as gssize {
        value_p = &raw mut value as *mut ::core::ffi::c_char;
    } else if len == -(1 as ::core::ffi::c_int) as gssize && errsv == ERANGE {
        len = safe_c2rust_g_getxattr(path, xattr, NULL_0, 0 as size_t, follow_symlinks);
        if len < 0 as gssize {
            return;
        }
        value_p = g_malloc((len + 1 as gssize) as gsize) as *mut ::core::ffi::c_char;
        len = safe_c2rust_g_getxattr(
            path,
            xattr,
            value_p as *mut ::core::ffi::c_void,
            len as size_t,
            follow_symlinks,
        );
        if len < 0 as gssize {
            g_free(value_p as gpointer);
            return;
        }
    } else {
        return;
    }
    *value_p.offset(len as isize) = 0 as ::core::ffi::c_char;
    safe_c2rust_escape_xattr(info, gio_attr, value_p, len as size_t);
    if value_p != &raw mut value as *mut ::core::ffi::c_char {
        g_free(value_p as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_get_xattrs(
    mut path: *const ::core::ffi::c_char,
    mut user: gboolean,
    mut info: *mut GFileInfo,
    mut matcher: *mut GFileAttributeMatcher,
    mut follow_symlinks: gboolean,
) {
    let mut all: gboolean = 0;
    let mut list_size: gsize = 0;
    let mut list_res_size: gssize = 0;
    let mut len: size_t = 0;
    let mut list: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut attr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut attr2: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if user != 0 {
        all = g_file_attribute_matcher_enumerate_namespace(
            matcher,
            b"xattr\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        all = g_file_attribute_matcher_enumerate_namespace(
            matcher,
            b"xattr-sys\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if all != 0 {
        let mut errsv: ::core::ffi::c_int = 0;
        list_res_size = safe_c2rust_g_listxattr(
            path,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as size_t,
            follow_symlinks,
        );
        if list_res_size == -(1 as ::core::ffi::c_int) as gssize || list_res_size == 0 as gssize {
            return;
        }
        list_size = list_res_size as gsize;
        list = g_malloc(list_size) as *mut ::core::ffi::c_char;
        loop {
            list_res_size =
                safe_c2rust_g_listxattr(path, list, list_size as size_t, follow_symlinks);
            errsv = *__errno_location();
            if !(list_res_size == -(1 as ::core::ffi::c_int) as gssize && errsv == ERANGE) {
                break;
            }
            list_size = list_size.wrapping_mul(2 as gsize);
            list = g_realloc(list as gpointer, list_size) as *mut ::core::ffi::c_char;
        }
        if list_res_size == -(1 as ::core::ffi::c_int) as gssize {
            g_free(list as gpointer);
            return;
        }
        attr = list;
        while list_res_size > 0 as gssize {
            if user != 0
                && (if 0 != 0 {
                    ({
                        let __str: *const ::core::ffi::c_char = attr;
                        let __prefix: *const ::core::ffi::c_char =
                            b"user.\0" as *const u8 as *const ::core::ffi::c_char;
                        let mut __result: gboolean = FALSE;
                        if ({
                            let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                            if __str.is_null() || __prefix.is_null() {
                                _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_11
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            __result =
                                g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                        } else {
                            let __str_len: size_t = strlen(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize),
                            ) as size_t;
                            let __prefix_len: size_t = strlen(
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                            ) as size_t;
                            if __str_len >= __prefix_len {
                                __result = (memcmp(
                                    __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                        as *const ::core::ffi::c_void,
                                    __prefix
                                        .offset(__prefix.is_null() as ::core::ffi::c_int as isize)
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
                    g_str_has_prefix(
                        attr as *const gchar,
                        b"user.\0" as *const u8 as *const gchar,
                    )
                }) != 0
                || user == 0
                    && (if 0 != 0 {
                        ({
                            let __str: *const ::core::ffi::c_char = attr;
                            let __prefix: *const ::core::ffi::c_char =
                                b"user.\0" as *const u8 as *const ::core::ffi::c_char;
                            let mut __result: gboolean = FALSE;
                            if ({
                                let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                                if __str.is_null() || __prefix.is_null() {
                                    _g_boolean_var_12 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_12 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_12
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                __result = g_str_has_prefix(
                                    __str as *const gchar,
                                    __prefix as *const gchar,
                                );
                            } else {
                                let __str_len: size_t = strlen(
                                    __str.offset(__str.is_null() as ::core::ffi::c_int as isize),
                                ) as size_t;
                                let __prefix_len: size_t = strlen(
                                    __prefix
                                        .offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                                )
                                    as size_t;
                                if __str_len >= __prefix_len {
                                    __result = (memcmp(
                                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                            as *const ::core::ffi::c_void,
                                        __prefix.offset(
                                            __prefix.is_null() as ::core::ffi::c_int as isize
                                        )
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
                        g_str_has_prefix(
                            attr as *const gchar,
                            b"user.\0" as *const u8 as *const gchar,
                        )
                    }) == 0
            {
                let mut escaped_attr: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut gio_attr: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut free_escaped_attr: gboolean = 0;
                if user != 0 {
                    escaped_attr = safe_c2rust_hex_escape_string(
                        attr.offset(5 as ::core::ffi::c_int as isize),
                        &raw mut free_escaped_attr,
                    );
                    gio_attr = g_strconcat(
                        b"xattr::\0" as *const u8 as *const gchar,
                        escaped_attr,
                        NULL_0,
                    ) as *mut ::core::ffi::c_char;
                } else {
                    escaped_attr = safe_c2rust_hex_escape_string(attr, &raw mut free_escaped_attr);
                    gio_attr = g_strconcat(
                        b"xattr-sys::\0" as *const u8 as *const gchar,
                        escaped_attr,
                        NULL_0,
                    ) as *mut ::core::ffi::c_char;
                }
                if free_escaped_attr != 0 {
                    g_free(escaped_attr as gpointer);
                }
                safe_c2rust_get_one_xattr(path, info, gio_attr, attr, follow_symlinks);
                g_free(gio_attr as gpointer);
            }
            len = strlen(attr).wrapping_add(1 as size_t);
            attr = attr.offset(len as isize);
            list_res_size = (list_res_size as size_t).wrapping_sub(len) as gssize as gssize;
        }
        g_free(list as gpointer);
    } else {
        loop {
            attr = g_file_attribute_matcher_enumerate_next(matcher);
            if attr.is_null() {
                break;
            }
            let mut unescaped_attribute: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut a: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut free_unescaped_attribute: gboolean = 0;
            attr2 = strchr(attr, ':' as i32);
            if !attr2.is_null() {
                attr2 = attr2.offset(2 as ::core::ffi::c_int as isize);
                unescaped_attribute = safe_c2rust_hex_unescape_string(
                    attr2,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    &raw mut free_unescaped_attribute,
                );
                if user != 0 {
                    a = g_strconcat(
                        b"user.\0" as *const u8 as *const gchar,
                        unescaped_attribute,
                        NULL_0,
                    ) as *mut ::core::ffi::c_char;
                } else {
                    a = unescaped_attribute;
                }
                safe_c2rust_get_one_xattr(path, info, attr, a, follow_symlinks);
                if user != 0 {
                    g_free(a as gpointer);
                }
                if free_unescaped_attribute != 0 {
                    g_free(unescaped_attribute as gpointer);
                }
            }
        }
    };
}
unsafe extern "C" fn safe_c2rust_get_one_xattr_from_fd(
    mut fd: ::core::ffi::c_int,
    mut info: *mut GFileInfo,
    mut gio_attr: *const ::core::ffi::c_char,
    mut xattr: *const ::core::ffi::c_char,
) {
    let mut value: [::core::ffi::c_char; 64] = [0; 64];
    let mut value_p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: gssize = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    len = fgetxattr(
        fd,
        xattr,
        &raw mut value as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t).wrapping_sub(1 as size_t),
    ) as gssize;
    errsv = *__errno_location();
    value_p = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if len >= 0 as gssize {
        value_p = &raw mut value as *mut ::core::ffi::c_char;
    } else if len == -(1 as ::core::ffi::c_int) as gssize && errsv == ERANGE {
        len = fgetxattr(fd, xattr, NULL_0, 0 as size_t) as gssize;
        if len < 0 as gssize {
            return;
        }
        value_p = g_malloc((len + 1 as gssize) as gsize) as *mut ::core::ffi::c_char;
        len = fgetxattr(
            fd,
            xattr,
            value_p as *mut ::core::ffi::c_void,
            len as size_t,
        ) as gssize;
        if len < 0 as gssize {
            g_free(value_p as gpointer);
            return;
        }
    } else {
        return;
    }
    *value_p.offset(len as isize) = 0 as ::core::ffi::c_char;
    safe_c2rust_escape_xattr(info, gio_attr, value_p, len as size_t);
    if value_p != &raw mut value as *mut ::core::ffi::c_char {
        g_free(value_p as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_get_xattrs_from_fd(
    mut fd: ::core::ffi::c_int,
    mut user: gboolean,
    mut info: *mut GFileInfo,
    mut matcher: *mut GFileAttributeMatcher,
) {
    let mut all: gboolean = 0;
    let mut list_size: gsize = 0;
    let mut list_res_size: gssize = 0;
    let mut len: size_t = 0;
    let mut list: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut attr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut attr2: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if user != 0 {
        all = g_file_attribute_matcher_enumerate_namespace(
            matcher,
            b"xattr\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        all = g_file_attribute_matcher_enumerate_namespace(
            matcher,
            b"xattr-sys\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if all != 0 {
        let mut errsv: ::core::ffi::c_int = 0;
        list_res_size = flistxattr(
            fd,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as size_t,
        ) as gssize;
        if list_res_size == -(1 as ::core::ffi::c_int) as gssize || list_res_size == 0 as gssize {
            return;
        }
        list_size = list_res_size as gsize;
        list = g_malloc(list_size) as *mut ::core::ffi::c_char;
        loop {
            list_res_size = flistxattr(fd, list, list_size as size_t) as gssize;
            errsv = *__errno_location();
            if !(list_res_size == -(1 as ::core::ffi::c_int) as gssize && errsv == ERANGE) {
                break;
            }
            list_size = list_size.wrapping_mul(2 as gsize);
            list = g_realloc(list as gpointer, list_size) as *mut ::core::ffi::c_char;
        }
        if list_res_size == -(1 as ::core::ffi::c_int) as gssize {
            g_free(list as gpointer);
            return;
        }
        attr = list;
        while list_res_size > 0 as gssize {
            if user != 0
                && (if 0 != 0 {
                    ({
                        let __str: *const ::core::ffi::c_char = attr;
                        let __prefix: *const ::core::ffi::c_char =
                            b"user.\0" as *const u8 as *const ::core::ffi::c_char;
                        let mut __result: gboolean = FALSE;
                        if ({
                            let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                            if __str.is_null() || __prefix.is_null() {
                                _g_boolean_var_13 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_13 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_13
                        }) as ::core::ffi::c_long
                            != 0
                        {
                            __result =
                                g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                        } else {
                            let __str_len: size_t = strlen(
                                __str.offset(__str.is_null() as ::core::ffi::c_int as isize),
                            ) as size_t;
                            let __prefix_len: size_t = strlen(
                                __prefix.offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                            ) as size_t;
                            if __str_len >= __prefix_len {
                                __result = (memcmp(
                                    __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                        as *const ::core::ffi::c_void,
                                    __prefix
                                        .offset(__prefix.is_null() as ::core::ffi::c_int as isize)
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
                    g_str_has_prefix(
                        attr as *const gchar,
                        b"user.\0" as *const u8 as *const gchar,
                    )
                }) != 0
                || user == 0
                    && (if 0 != 0 {
                        ({
                            let __str: *const ::core::ffi::c_char = attr;
                            let __prefix: *const ::core::ffi::c_char =
                                b"user.\0" as *const u8 as *const ::core::ffi::c_char;
                            let mut __result: gboolean = FALSE;
                            if ({
                                let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                                if __str.is_null() || __prefix.is_null() {
                                    _g_boolean_var_14 = 1 as ::core::ffi::c_int;
                                } else {
                                    _g_boolean_var_14 = 0 as ::core::ffi::c_int;
                                }
                                _g_boolean_var_14
                            }) as ::core::ffi::c_long
                                != 0
                            {
                                __result = g_str_has_prefix(
                                    __str as *const gchar,
                                    __prefix as *const gchar,
                                );
                            } else {
                                let __str_len: size_t = strlen(
                                    __str.offset(__str.is_null() as ::core::ffi::c_int as isize),
                                ) as size_t;
                                let __prefix_len: size_t = strlen(
                                    __prefix
                                        .offset(__prefix.is_null() as ::core::ffi::c_int as isize),
                                )
                                    as size_t;
                                if __str_len >= __prefix_len {
                                    __result = (memcmp(
                                        __str.offset(__str.is_null() as ::core::ffi::c_int as isize)
                                            as *const ::core::ffi::c_void,
                                        __prefix.offset(
                                            __prefix.is_null() as ::core::ffi::c_int as isize
                                        )
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
                        g_str_has_prefix(
                            attr as *const gchar,
                            b"user.\0" as *const u8 as *const gchar,
                        )
                    }) == 0
            {
                let mut escaped_attr: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut gio_attr: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut free_escaped_attr: gboolean = 0;
                if user != 0 {
                    escaped_attr = safe_c2rust_hex_escape_string(
                        attr.offset(5 as ::core::ffi::c_int as isize),
                        &raw mut free_escaped_attr,
                    );
                    gio_attr = g_strconcat(
                        b"xattr::\0" as *const u8 as *const gchar,
                        escaped_attr,
                        NULL_0,
                    ) as *mut ::core::ffi::c_char;
                } else {
                    escaped_attr = safe_c2rust_hex_escape_string(attr, &raw mut free_escaped_attr);
                    gio_attr = g_strconcat(
                        b"xattr-sys::\0" as *const u8 as *const gchar,
                        escaped_attr,
                        NULL_0,
                    ) as *mut ::core::ffi::c_char;
                }
                if free_escaped_attr != 0 {
                    g_free(escaped_attr as gpointer);
                }
                safe_c2rust_get_one_xattr_from_fd(fd, info, gio_attr, attr);
                g_free(gio_attr as gpointer);
            }
            len = strlen(attr).wrapping_add(1 as size_t);
            attr = attr.offset(len as isize);
            list_res_size = (list_res_size as size_t).wrapping_sub(len) as gssize as gssize;
        }
        g_free(list as gpointer);
    } else {
        loop {
            attr = g_file_attribute_matcher_enumerate_next(matcher);
            if attr.is_null() {
                break;
            }
            let mut unescaped_attribute: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut a: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut free_unescaped_attribute: gboolean = 0;
            attr2 = strchr(attr, ':' as i32);
            if !attr2.is_null() {
                attr2 = attr2.offset(1);
                unescaped_attribute = safe_c2rust_hex_unescape_string(
                    attr2,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    &raw mut free_unescaped_attribute,
                );
                if user != 0 {
                    a = g_strconcat(
                        b"user.\0" as *const u8 as *const gchar,
                        unescaped_attribute,
                        NULL_0,
                    ) as *mut ::core::ffi::c_char;
                } else {
                    a = unescaped_attribute;
                }
                safe_c2rust_get_one_xattr_from_fd(fd, info, attr, a);
                if user != 0 {
                    g_free(a as gpointer);
                }
                if free_unescaped_attribute != 0 {
                    g_free(unescaped_attribute as gpointer);
                }
            }
        }
    };
}
unsafe extern "C" fn safe_c2rust_set_xattr(
    mut filename: *mut ::core::ffi::c_char,
    mut escaped_attribute: *const ::core::ffi::c_char,
    mut attr_value: *const GFileAttributeValue,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut attribute: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut free_attribute: gboolean = 0;
    let mut free_value: gboolean = 0;
    let mut val_len: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut errsv: ::core::ffi::c_int = 0;
    let mut is_user: gboolean = 0;
    let mut a: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if attr_value.is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"Attribute value must be non-NULL\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    if (*attr_value).type_0() as ::core::ffi::c_int
        != G_FILE_ATTRIBUTE_TYPE_STRING as ::core::ffi::c_int
        && (*attr_value).type_0() as ::core::ffi::c_int
            != G_FILE_ATTRIBUTE_TYPE_INVALID as ::core::ffi::c_int
    {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Invalid attribute type (string or invalid expected)\0" as *const u8
                    as *const gchar,
            ),
        );
        return FALSE;
    }
    if safe_c2rust_name_is_valid(escaped_attribute) == 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid extended attribute name\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = escaped_attribute;
            let __prefix: *const ::core::ffi::c_char =
                b"xattr::\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_15 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_15 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_15
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
            escaped_attribute as *const gchar,
            b"xattr::\0" as *const u8 as *const gchar,
        )
    } != 0
    {
        escaped_attribute = escaped_attribute
            .offset(strlen(b"xattr::\0" as *const u8 as *const ::core::ffi::c_char) as isize);
        is_user = TRUE as gboolean;
    } else {
        if !(({
            let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
            if if 0 != 0 {
                ({
                    let __str: *const ::core::ffi::c_char = escaped_attribute;
                    let __prefix: *const ::core::ffi::c_char =
                        b"xattr-sys::\0" as *const u8 as *const ::core::ffi::c_char;
                    let mut __result: gboolean = 0 as gboolean;
                    if ({
                        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
                        if __str.is_null() || __prefix.is_null() {
                            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_16
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
                g_str_has_prefix(
                    escaped_attribute as *const gchar,
                    b"xattr-sys::\0" as *const u8 as *const gchar,
                )
            } != 0
            {
                _g_boolean_var_17 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_17 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_17
        }) as ::core::ffi::c_long
            != 0)
        {
            g_warn_message(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfileinfo.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                790 as ::core::ffi::c_int,
                G_STRFUNC,
                b"g_str_has_prefix (escaped_attribute, \"xattr-sys::\")\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        escaped_attribute = escaped_attribute
            .offset(strlen(b"xattr-sys::\0" as *const u8 as *const ::core::ffi::c_char) as isize);
        is_user = FALSE as gboolean;
    }
    attribute = safe_c2rust_hex_unescape_string(
        escaped_attribute,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut free_attribute,
    );
    if is_user != 0 {
        a = g_strconcat(b"user.\0" as *const u8 as *const gchar, attribute, NULL_0)
            as *mut ::core::ffi::c_char;
    } else {
        a = attribute;
    }
    if (*attr_value).type_0() as ::core::ffi::c_int
        == G_FILE_ATTRIBUTE_TYPE_STRING as ::core::ffi::c_int
    {
        value = safe_c2rust_hex_unescape_string(
            (*attr_value).u.string,
            &raw mut val_len,
            &raw mut free_value,
        );
        res = setxattr(
            filename,
            a,
            value as *const ::core::ffi::c_void,
            val_len as size_t,
            0 as ::core::ffi::c_int,
        );
    } else {
        value = ::core::ptr::null_mut::<::core::ffi::c_char>();
        val_len = 0 as ::core::ffi::c_int;
        free_value = FALSE as gboolean;
        res = removexattr(filename, a);
    }
    errsv = *__errno_location();
    if is_user != 0 {
        g_free(a as gpointer);
    }
    if free_attribute != 0 {
        g_free(attribute as gpointer);
    }
    if free_value != 0 {
        g_free(value as gpointer);
    }
    if res == -(1 as ::core::ffi::c_int) {
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(
                b"Error setting extended attribute \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                    as *const gchar,
            ),
            escaped_attribute,
            g_strerror(errsv as gint),
        );
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_info_get_parent_info(
    mut dir: *const ::core::ffi::c_char,
    mut attribute_matcher: *mut GFileAttributeMatcher,
    mut parent_info: *mut GLocalParentFileInfo,
) {
    let mut statbuf: GStatBuf = stat {
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
    let mut res: ::core::ffi::c_int = 0;
    (*parent_info).extra_data = NULL_0 as gpointer;
    (*parent_info).free_extra_data = None;
    (*parent_info).writable = FALSE as gboolean;
    (*parent_info).is_sticky = FALSE as gboolean;
    (*parent_info).has_trash_dir = FALSE as gboolean;
    (*parent_info).device = 0 as dev_t;
    (*parent_info).inode = 0 as ino_t;
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_ACCESS_CAN_RENAME as guint32,
    ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_ACCESS_CAN_DELETE as guint32,
        ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_ACCESS_CAN_TRASH as guint32,
        ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_UNIX_IS_MOUNTPOINT as guint32,
        ) != 0
    {
        (*parent_info).writable = (g_access(dir as *const gchar, W_OK) == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as gboolean;
        res = stat(dir, &raw mut statbuf);
        if res == 0 as ::core::ffi::c_int {
            (*parent_info).is_sticky = (statbuf.st_mode & S_ISVTX as __mode_t != 0 as __mode_t)
                as ::core::ffi::c_int as gboolean;
            (*parent_info).owner = statbuf.st_uid as ::core::ffi::c_int;
            (*parent_info).device = statbuf.st_dev as dev_t;
            (*parent_info).inode = statbuf.st_ino as ino_t;
            if (*parent_info).writable != 0
                && _g_file_attribute_matcher_matches_id(
                    attribute_matcher,
                    G_FILE_ATTRIBUTE_ID_ACCESS_CAN_TRASH as guint32,
                ) != 0
            {
                (*parent_info).has_trash_dir =
                    _g_local_file_has_trash_dir(dir, statbuf.st_dev as dev_t);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_info_free_parent_info(
    mut parent_info: *mut GLocalParentFileInfo,
) {
    if !(*parent_info).extra_data.is_null() && (*parent_info).free_extra_data.is_some() {
        (*parent_info)
            .free_extra_data
            .expect("non-null function pointer")((*parent_info).extra_data);
    }
}
unsafe extern "C" fn safe_c2rust_get_access_rights(
    mut attribute_matcher: *mut GFileAttributeMatcher,
    mut info: *mut GFileInfo,
    mut path: *const gchar,
    mut statbuf: *mut statx,
    mut parent_info: *mut GLocalParentFileInfo,
) {
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_ACCESS_CAN_READ as guint32,
    ) != 0
    {
        _g_file_info_set_attribute_boolean_by_id(
            info,
            G_FILE_ATTRIBUTE_ID_ACCESS_CAN_READ as guint32,
            (g_access(path, R_OK) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_ACCESS_CAN_WRITE as guint32,
    ) != 0
    {
        _g_file_info_set_attribute_boolean_by_id(
            info,
            G_FILE_ATTRIBUTE_ID_ACCESS_CAN_WRITE as guint32,
            (g_access(path, W_OK) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_ACCESS_CAN_EXECUTE as guint32,
    ) != 0
    {
        _g_file_info_set_attribute_boolean_by_id(
            info,
            G_FILE_ATTRIBUTE_ID_ACCESS_CAN_EXECUTE as guint32,
            (g_access(path, X_OK) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
    }
    if !parent_info.is_null() {
        let mut writable: gboolean = 0;
        writable = FALSE as gboolean;
        if (*parent_info).writable != 0 {
            if (*parent_info).is_sticky != 0 {
                let mut uid: uid_t = geteuid() as uid_t;
                if uid == safe_c2rust__g_stat_uid(statbuf)
                    || uid == (*parent_info).owner as uid_t
                    || uid == 0 as uid_t
                {
                    writable = TRUE as gboolean;
                }
            } else {
                writable = TRUE as gboolean;
            }
        }
        if _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_ACCESS_CAN_RENAME as guint32,
        ) != 0
        {
            _g_file_info_set_attribute_boolean_by_id(
                info,
                G_FILE_ATTRIBUTE_ID_ACCESS_CAN_RENAME as guint32,
                writable,
            );
        }
        if _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_ACCESS_CAN_DELETE as guint32,
        ) != 0
        {
            _g_file_info_set_attribute_boolean_by_id(
                info,
                G_FILE_ATTRIBUTE_ID_ACCESS_CAN_DELETE as guint32,
                writable,
            );
        }
        if _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_ACCESS_CAN_TRASH as guint32,
        ) != 0
        {
            _g_file_info_set_attribute_boolean_by_id(
                info,
                G_FILE_ATTRIBUTE_ID_ACCESS_CAN_TRASH as guint32,
                (writable != 0 && (*parent_info).has_trash_dir != 0) as ::core::ffi::c_int,
            );
        }
    }
}
unsafe extern "C" fn safe_c2rust_set_info_from_stat(
    mut info: *mut GFileInfo,
    mut statbuf: *mut statx,
    mut attribute_matcher: *mut GFileAttributeMatcher,
) {
    let mut file_type: GFileType = G_FILE_TYPE_UNKNOWN;
    file_type = G_FILE_TYPE_UNKNOWN;
    if safe_c2rust__g_stat_mode(statbuf) as ::core::ffi::c_int & __S_IFMT
        == 0o100000 as ::core::ffi::c_int
    {
        file_type = G_FILE_TYPE_REGULAR;
    } else if safe_c2rust__g_stat_mode(statbuf) as ::core::ffi::c_int & __S_IFMT
        == 0o40000 as ::core::ffi::c_int
    {
        file_type = G_FILE_TYPE_DIRECTORY;
    } else if safe_c2rust__g_stat_mode(statbuf) as ::core::ffi::c_int & __S_IFMT
        == 0o20000 as ::core::ffi::c_int
        || safe_c2rust__g_stat_mode(statbuf) as ::core::ffi::c_int & __S_IFMT
            == 0o60000 as ::core::ffi::c_int
        || safe_c2rust__g_stat_mode(statbuf) as ::core::ffi::c_int & __S_IFMT
            == 0o10000 as ::core::ffi::c_int
        || safe_c2rust__g_stat_mode(statbuf) as ::core::ffi::c_int & __S_IFMT
            == 0o140000 as ::core::ffi::c_int
    {
        file_type = G_FILE_TYPE_SPECIAL;
    } else if safe_c2rust__g_stat_mode(statbuf) as ::core::ffi::c_int & __S_IFMT
        == 0o120000 as ::core::ffi::c_int
    {
        file_type = G_FILE_TYPE_SYMBOLIC_LINK;
    }
    g_file_info_set_file_type(info, file_type);
    g_file_info_set_size(info, safe_c2rust__g_stat_size(statbuf) as goffset);
    _g_file_info_set_attribute_uint32_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_UNIX_DEVICE as guint32,
        safe_c2rust__g_stat_dev(statbuf) as guint32,
    );
    _g_file_info_set_attribute_uint32_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_UNIX_NLINK as guint32,
        safe_c2rust__g_stat_nlink(statbuf),
    );
    _g_file_info_set_attribute_uint64_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_UNIX_INODE as guint32,
        safe_c2rust__g_stat_ino(statbuf),
    );
    _g_file_info_set_attribute_uint32_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_UNIX_UID as guint32,
        safe_c2rust__g_stat_uid(statbuf),
    );
    _g_file_info_set_attribute_uint32_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_UNIX_GID as guint32,
        safe_c2rust__g_stat_gid(statbuf),
    );
    _g_file_info_set_attribute_uint32_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_UNIX_RDEV as guint32,
        safe_c2rust__g_stat_rdev(statbuf) as guint32,
    );
    _g_file_info_set_attribute_uint32_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_UNIX_MODE as guint32,
        safe_c2rust__g_stat_mode(statbuf) as guint32,
    );
    _g_file_info_set_attribute_uint32_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_UNIX_BLOCK_SIZE as guint32,
        safe_c2rust__g_stat_blksize(statbuf),
    );
    _g_file_info_set_attribute_uint64_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_UNIX_BLOCKS as guint32,
        safe_c2rust__g_stat_blocks(statbuf),
    );
    _g_file_info_set_attribute_uint64_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_STANDARD_ALLOCATED_SIZE as guint32,
        safe_c2rust__g_stat_blocks(statbuf).wrapping_mul(512 as guint64),
    );
    _g_file_info_set_attribute_uint64_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_TIME_MODIFIED as guint32,
        safe_c2rust__g_stat_mtime(statbuf) as guint64,
    );
    _g_file_info_set_attribute_uint32_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_TIME_MODIFIED_USEC as guint32,
        safe_c2rust__g_stat_mtim_nsec(statbuf).wrapping_div(1000 as guint32),
    );
    _g_file_info_set_attribute_uint32_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_TIME_MODIFIED_NSEC as guint32,
        safe_c2rust__g_stat_mtim_nsec(statbuf),
    );
    if safe_c2rust__g_stat_has_field(statbuf, G_LOCAL_FILE_STAT_FIELD_ATIME) != 0 {
        _g_file_info_set_attribute_uint64_by_id(
            info,
            G_FILE_ATTRIBUTE_ID_TIME_ACCESS as guint32,
            safe_c2rust__g_stat_atime(statbuf) as guint64,
        );
        _g_file_info_set_attribute_uint32_by_id(
            info,
            G_FILE_ATTRIBUTE_ID_TIME_ACCESS_USEC as guint32,
            safe_c2rust__g_stat_atim_nsec(statbuf).wrapping_div(1000 as guint32),
        );
        _g_file_info_set_attribute_uint32_by_id(
            info,
            G_FILE_ATTRIBUTE_ID_TIME_ACCESS_NSEC as guint32,
            safe_c2rust__g_stat_atim_nsec(statbuf),
        );
    }
    _g_file_info_set_attribute_uint64_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_TIME_CHANGED as guint32,
        safe_c2rust__g_stat_ctime(statbuf) as guint64,
    );
    _g_file_info_set_attribute_uint32_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_TIME_CHANGED_USEC as guint32,
        safe_c2rust__g_stat_ctim_nsec(statbuf).wrapping_div(1000 as guint32),
    );
    _g_file_info_set_attribute_uint32_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_TIME_CHANGED_NSEC as guint32,
        safe_c2rust__g_stat_ctim_nsec(statbuf),
    );
    if safe_c2rust__g_stat_has_field(statbuf, G_LOCAL_FILE_STAT_FIELD_BTIME) != 0 {
        _g_file_info_set_attribute_uint64_by_id(
            info,
            G_FILE_ATTRIBUTE_ID_TIME_CREATED as guint32,
            (*statbuf).stx_btime.tv_sec as guint64,
        );
        _g_file_info_set_attribute_uint32_by_id(
            info,
            G_FILE_ATTRIBUTE_ID_TIME_CREATED_USEC as guint32,
            ((*statbuf).stx_btime.tv_nsec as guint32).wrapping_div(1000 as guint32),
        );
        _g_file_info_set_attribute_uint32_by_id(
            info,
            G_FILE_ATTRIBUTE_ID_TIME_CREATED_NSEC as guint32,
            (*statbuf).stx_btime.tv_nsec as guint32,
        );
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_ETAG_VALUE as guint32,
    ) != 0
    {
        let mut etag: *mut ::core::ffi::c_char =
            safe_c2rust__g_local_file_info_create_etag(statbuf);
        _g_file_info_set_attribute_string_by_id(
            info,
            G_FILE_ATTRIBUTE_ID_ETAG_VALUE as guint32,
            etag,
        );
        g_free(etag as gpointer);
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_ID_FILE as guint32,
    ) != 0
    {
        let mut id: *mut ::core::ffi::c_char =
            safe_c2rust__g_local_file_info_create_file_id(statbuf);
        _g_file_info_set_attribute_string_by_id(info, G_FILE_ATTRIBUTE_ID_ID_FILE as guint32, id);
        g_free(id as gpointer);
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_ID_FILESYSTEM as guint32,
    ) != 0
    {
        let mut id_0: *mut ::core::ffi::c_char =
            safe_c2rust__g_local_file_info_create_fs_id(statbuf);
        _g_file_info_set_attribute_string_by_id(
            info,
            G_FILE_ATTRIBUTE_ID_ID_FILESYSTEM as guint32,
            id_0,
        );
        g_free(id_0 as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_make_valid_utf8(
    mut name: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut string: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut remainder: *const gchar = ::core::ptr::null::<gchar>();
    let mut invalid: *const gchar = ::core::ptr::null::<gchar>();
    let mut remaining_bytes: gsize = 0;
    let mut valid_bytes: gsize = 0;
    string = ::core::ptr::null_mut::<GString>();
    remainder = name as *const gchar;
    remaining_bytes = strlen(name) as gsize;
    while remaining_bytes != 0 as gsize {
        if g_utf8_validate_len(remainder, remaining_bytes, &raw mut invalid) != 0 {
            break;
        }
        valid_bytes = invalid.offset_from(remainder) as ::core::ffi::c_long as gsize;
        if string.is_null() {
            string = g_string_sized_new(remaining_bytes);
        }
        safe_c2rust_g_string_append_len_inline(
            string,
            remainder as *const ::core::ffi::c_char,
            valid_bytes as gssize,
        );
        if 0 != 0 {
            ({
                let __val: *const ::core::ffi::c_char =
                    b"\xEF\xBF\xBD\0" as *const u8 as *const ::core::ffi::c_char;
                safe_c2rust_g_string_append_len_inline(
                    string,
                    __val,
                    if ({
                        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
                        if !__val.is_null() {
                            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
                        } else {
                            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
                        }
                        _g_boolean_var_18
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
                b"\xEF\xBF\xBD\0" as *const u8 as *const ::core::ffi::c_char,
                -(1 as ::core::ffi::c_int) as gssize,
            );
        };
        remaining_bytes = remaining_bytes.wrapping_sub(valid_bytes.wrapping_add(1 as gsize));
        remainder = invalid.offset(1 as ::core::ffi::c_int as isize);
    }
    if string.is_null() {
        return safe_c2rust_g_strdup_inline(name);
    }
    if 0 != 0 {
        ({
            let __val: *const ::core::ffi::c_char = remainder as *const ::core::ffi::c_char;
            safe_c2rust_g_string_append_len_inline(
                string,
                __val,
                if ({
                    let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
                    if !__val.is_null() {
                        _g_boolean_var_19 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_19 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_19
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
            string,
            remainder as *const ::core::ffi::c_char,
            -(1 as ::core::ffi::c_int) as gssize,
        );
    };
    if !(({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if g_utf8_validate(
            (*string).str_0,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<*const gchar>(),
        ) != 0
        {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0)
    {
        g_warn_message(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfileinfo.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            1132 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_utf8_validate (string->str, -1, NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return if 0 != 0 {
        if 0 as ::core::ffi::c_int != 0 {
            g_string_free(string, 0 as gboolean) as *mut ::core::ffi::c_char
        } else {
            g_string_free_and_steal(string) as *mut ::core::ffi::c_char
        }
    } else {
        g_string_free(string, 0 as gboolean) as *mut ::core::ffi::c_char
    };
}
unsafe extern "C" fn safe_c2rust_convert_pwd_string_to_utf8(
    mut pwd_str: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut utf8_string: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if g_utf8_validate(
        pwd_str,
        -(1 as ::core::ffi::c_int) as gssize,
        ::core::ptr::null_mut::<*const gchar>(),
    ) == 0
    {
        utf8_string = g_locale_to_utf8(
            pwd_str,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        ) as *mut ::core::ffi::c_char;
        if utf8_string.is_null() {
            utf8_string = safe_c2rust_make_valid_utf8(pwd_str);
        }
    } else {
        utf8_string = safe_c2rust_g_strdup_inline(pwd_str);
    }
    return utf8_string;
}
unsafe extern "C" fn safe_c2rust_uid_data_free(mut data: *mut UidData) {
    g_free((*data).user_name as gpointer);
    g_free((*data).real_name as gpointer);
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_lookup_uid_data(mut uid: uid_t) -> *mut UidData {
    let mut data: *mut UidData = ::core::ptr::null_mut::<UidData>();
    let mut buffer: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut pwbuf: passwd = passwd {
        pw_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pw_passwd: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pw_uid: 0,
        pw_gid: 0,
        pw_gecos: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pw_dir: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pw_shell: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut pwbufp: *mut passwd = ::core::ptr::null_mut::<passwd>();
    let mut gecos: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut comma: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if safe_c2rust_uid_cache.is_null() {
        safe_c2rust_uid_cache = g_hash_table_new_full(
            None,
            None,
            None,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut UidData) -> ()>,
                GDestroyNotify,
            >(Some(
                safe_c2rust_uid_data_free as unsafe extern "C" fn(*mut UidData) -> (),
            )),
        );
    }
    data = g_hash_table_lookup(
        safe_c2rust_uid_cache,
        uid as glong as gpointer as gconstpointer,
    ) as *mut UidData;
    if !data.is_null() {
        return data;
    }
    data = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<UidData>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut UidData;
    getpwuid_r(
        uid as __uid_t,
        &raw mut pwbuf,
        &raw mut buffer as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
        &raw mut pwbufp,
    );
    if !pwbufp.is_null() {
        if !(*pwbufp).pw_name.is_null()
            && *(*pwbufp).pw_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            (*data).user_name = safe_c2rust_convert_pwd_string_to_utf8((*pwbufp).pw_name);
        }
        gecos = (*pwbufp).pw_gecos;
        if !gecos.is_null() {
            comma = strchr(gecos, ',' as i32);
            if !comma.is_null() {
                *comma = 0 as ::core::ffi::c_char;
            }
            (*data).real_name = safe_c2rust_convert_pwd_string_to_utf8(gecos);
        }
    }
    if (*data).real_name.is_null() {
        if !(*data).user_name.is_null() {
            (*data).real_name = safe_c2rust_g_strdup_inline((*data).user_name);
        } else {
            (*data).real_name = g_strdup_printf(
                b"user #%d\0" as *const u8 as *const gchar,
                uid as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_char;
        }
    }
    if (*data).user_name.is_null() {
        (*data).user_name = g_strdup_printf(
            b"%d\0" as *const u8 as *const gchar,
            uid as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char;
    }
    g_hash_table_replace(
        safe_c2rust_uid_cache,
        uid as glong as gpointer,
        data as gpointer,
    );
    return data;
}
unsafe extern "C" fn safe_c2rust_get_username_from_uid(mut uid: uid_t) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut data: *mut UidData = ::core::ptr::null_mut::<UidData>();
    g_mutex_lock(&raw mut safe_c2rust_g__uid_cache_lock);
    data = safe_c2rust_lookup_uid_data(uid);
    res = safe_c2rust_g_strdup_inline((*data).user_name);
    g_mutex_unlock(&raw mut safe_c2rust_g__uid_cache_lock);
    return res;
}
unsafe extern "C" fn safe_c2rust_get_realname_from_uid(mut uid: uid_t) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut data: *mut UidData = ::core::ptr::null_mut::<UidData>();
    g_mutex_lock(&raw mut safe_c2rust_g__uid_cache_lock);
    data = safe_c2rust_lookup_uid_data(uid);
    res = safe_c2rust_g_strdup_inline((*data).real_name);
    g_mutex_unlock(&raw mut safe_c2rust_g__uid_cache_lock);
    return res;
}
unsafe extern "C" fn safe_c2rust_lookup_gid_name(mut gid: gid_t) -> *mut ::core::ffi::c_char {
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut buffer: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut gbuf: group = group {
        gr_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        gr_passwd: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        gr_gid: 0,
        gr_mem: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    };
    let mut gbufp: *mut group = ::core::ptr::null_mut::<group>();
    if safe_c2rust_gid_cache.is_null() {
        safe_c2rust_gid_cache = g_hash_table_new_full(
            None,
            None,
            None,
            ::core::mem::transmute::<Option<unsafe extern "C" fn(gpointer) -> ()>, GDestroyNotify>(
                Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            ),
        );
    }
    name = g_hash_table_lookup(
        safe_c2rust_gid_cache,
        gid as glong as gpointer as gconstpointer,
    ) as *mut ::core::ffi::c_char;
    if !name.is_null() {
        return name;
    }
    getgrgid_r(
        gid as __gid_t,
        &raw mut gbuf,
        &raw mut buffer as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
        &raw mut gbufp,
    );
    if !gbufp.is_null()
        && !(*gbufp).gr_name.is_null()
        && *(*gbufp).gr_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        name = safe_c2rust_convert_pwd_string_to_utf8((*gbufp).gr_name);
    } else {
        name = g_strdup_printf(
            b"%d\0" as *const u8 as *const gchar,
            gid as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char;
    }
    g_hash_table_replace(
        safe_c2rust_gid_cache,
        gid as glong as gpointer,
        name as gpointer,
    );
    return name;
}
unsafe extern "C" fn safe_c2rust_get_groupname_from_gid(
    mut gid: gid_t,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    g_mutex_lock(&raw mut safe_c2rust_g__gid_cache_lock);
    name = safe_c2rust_lookup_gid_name(gid);
    res = safe_c2rust_g_strdup_inline(name);
    g_mutex_unlock(&raw mut safe_c2rust_g__gid_cache_lock);
    return res;
}
unsafe extern "C" fn safe_c2rust_get_content_type(
    mut basename: *const ::core::ffi::c_char,
    mut path: *const ::core::ffi::c_char,
    mut statbuf: *mut statx,
    mut is_symlink: gboolean,
    mut symlink_broken: gboolean,
    mut flags: GFileQueryInfoFlags,
    mut fast: gboolean,
) -> *mut ::core::ffi::c_char {
    if is_symlink != 0
        && (symlink_broken != 0
            || flags as ::core::ffi::c_uint
                & G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0)
    {
        return g_content_type_from_mime_type(b"inode/symlink\0" as *const u8 as *const gchar)
            as *mut ::core::ffi::c_char;
    } else if !statbuf.is_null()
        && safe_c2rust__g_stat_mode(statbuf) as ::core::ffi::c_int & __S_IFMT
            == 0o40000 as ::core::ffi::c_int
    {
        return g_content_type_from_mime_type(b"inode/directory\0" as *const u8 as *const gchar)
            as *mut ::core::ffi::c_char;
    } else if !statbuf.is_null()
        && safe_c2rust__g_stat_mode(statbuf) as ::core::ffi::c_int & __S_IFMT
            == 0o20000 as ::core::ffi::c_int
    {
        return g_content_type_from_mime_type(b"inode/chardevice\0" as *const u8 as *const gchar)
            as *mut ::core::ffi::c_char;
    } else if !statbuf.is_null()
        && safe_c2rust__g_stat_mode(statbuf) as ::core::ffi::c_int & __S_IFMT
            == 0o60000 as ::core::ffi::c_int
    {
        return g_content_type_from_mime_type(b"inode/blockdevice\0" as *const u8 as *const gchar)
            as *mut ::core::ffi::c_char;
    } else if !statbuf.is_null()
        && safe_c2rust__g_stat_mode(statbuf) as ::core::ffi::c_int & __S_IFMT
            == 0o10000 as ::core::ffi::c_int
    {
        return g_content_type_from_mime_type(b"inode/fifo\0" as *const u8 as *const gchar)
            as *mut ::core::ffi::c_char;
    } else if !statbuf.is_null()
        && safe_c2rust__g_stat_mode(statbuf) as ::core::ffi::c_int & __S_IFMT
            == 0o100000 as ::core::ffi::c_int
        && safe_c2rust__g_stat_size(statbuf) == 0 as guint64
    {
        return g_content_type_from_mime_type(
            b"application/x-zerosize\0" as *const u8 as *const gchar,
        ) as *mut ::core::ffi::c_char;
    } else if !statbuf.is_null()
        && safe_c2rust__g_stat_mode(statbuf) as ::core::ffi::c_int & __S_IFMT
            == 0o140000 as ::core::ffi::c_int
    {
        return g_content_type_from_mime_type(b"inode/socket\0" as *const u8 as *const gchar)
            as *mut ::core::ffi::c_char;
    } else {
        let mut content_type: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut result_uncertain: gboolean = 0;
        content_type = g_content_type_guess(
            basename as *const gchar,
            ::core::ptr::null::<guchar>(),
            0 as gsize,
            &raw mut result_uncertain,
        ) as *mut ::core::ffi::c_char;
        if fast == 0 && result_uncertain != 0 && !path.is_null() {
            let mut sniff_buffer: [guchar; 16384] = [0; 16384];
            let mut sniff_length: gsize = 0;
            let mut errsv: ::core::ffi::c_int = 0;
            let mut fd: ::core::ffi::c_int = 0;
            sniff_length = _g_unix_content_type_get_sniff_len();
            if sniff_length == 0 as gsize
                || sniff_length as usize > ::core::mem::size_of::<[guchar; 16384]>() as usize
            {
                sniff_length = ::core::mem::size_of::<[guchar; 16384]>() as usize as gsize;
            }
            fd = open(
                path,
                O_RDONLY | O_NOATIME | O_CLOEXEC,
                0 as ::core::ffi::c_int,
            );
            errsv = *__errno_location();
            if fd < 0 as ::core::ffi::c_int && errsv == EPERM {
                fd = open(path, O_RDONLY | O_CLOEXEC, 0 as ::core::ffi::c_int);
            }
            if fd != -(1 as ::core::ffi::c_int) {
                let mut res: gssize = 0;
                res = read(
                    fd,
                    &raw mut sniff_buffer as *mut guchar as *mut ::core::ffi::c_void,
                    sniff_length as size_t,
                ) as gssize;
                g_close(fd as gint, ::core::ptr::null_mut::<*mut GError>());
                if res >= 0 as gssize {
                    g_free(content_type as gpointer);
                    content_type = g_content_type_guess(
                        basename as *const gchar,
                        &raw mut sniff_buffer as *mut guchar,
                        res as gsize,
                        ::core::ptr::null_mut::<gboolean>(),
                    ) as *mut ::core::ffi::c_char;
                }
            }
        }
        return content_type;
    };
}
unsafe extern "C" fn safe_c2rust_get_thumbnail_dirname_from_size(
    mut size: ThumbnailSize,
) -> *const ::core::ffi::c_char {
    match size as ::core::ffi::c_uint {
        0 => return ::core::ptr::null::<::core::ffi::c_char>(),
        1 => return b"normal\0" as *const u8 as *const ::core::ffi::c_char,
        2 => return b"large\0" as *const u8 as *const ::core::ffi::c_char,
        3 => return b"x-large\0" as *const u8 as *const ::core::ffi::c_char,
        4 => return b"xx-large\0" as *const u8 as *const ::core::ffi::c_char,
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfileinfo.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1420 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
unsafe extern "C" fn safe_c2rust_get_thumbnail_attributes(
    mut path: *const ::core::ffi::c_char,
    mut info: *mut GFileInfo,
    mut stat_buf: *const statx,
    mut size: ThumbnailSize,
) {
    let mut checksum: *mut GChecksum = ::core::ptr::null_mut::<GChecksum>();
    let mut dirname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut uri: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut basename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut failed_attr_id: guint32 = 0;
    let mut is_valid_attr_id: guint32 = 0;
    let mut path_attr_id: guint32 = 0;
    match size as ::core::ffi::c_uint {
        0 => {
            failed_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED as guint32;
            is_valid_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID as guint32;
            path_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH as guint32;
        }
        1 => {
            failed_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_NORMAL as guint32;
            is_valid_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_NORMAL as guint32;
            path_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_NORMAL as guint32;
        }
        2 => {
            failed_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_LARGE as guint32;
            is_valid_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_LARGE as guint32;
            path_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_LARGE as guint32;
        }
        3 => {
            failed_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_XLARGE as guint32;
            is_valid_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_XLARGE as guint32;
            path_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_XLARGE as guint32;
        }
        4 => {
            failed_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_XXLARGE as guint32;
            is_valid_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_XXLARGE as guint32;
            path_attr_id = G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_XXLARGE as guint32;
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfileinfo.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1470 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    dirname = safe_c2rust_get_thumbnail_dirname_from_size(size);
    uri = g_filename_to_uri(
        path as *const gchar,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    checksum = g_checksum_new(G_CHECKSUM_MD5);
    g_checksum_update(checksum, uri as *const guchar, strlen(uri) as gssize);
    basename = g_strconcat(
        g_checksum_get_string(checksum),
        b".png\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    ) as *mut ::core::ffi::c_char;
    g_checksum_free(checksum);
    if !dirname.is_null() {
        filename = g_build_filename(
            g_get_user_cache_dir(),
            b"thumbnails\0" as *const u8 as *const ::core::ffi::c_char,
            dirname,
            basename,
            NULL_0,
        ) as *mut ::core::ffi::c_char;
        if g_file_test(filename, G_FILE_TEST_IS_REGULAR) == 0 {
            let mut _pp: *mut *mut ::core::ffi::c_char = &raw mut filename;
            let mut _ptr: *mut ::core::ffi::c_char = *_pp;
            *_pp = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !_ptr.is_null() {
                g_free(_ptr as gpointer);
            }
        }
    } else {
        let mut i: gssize = 0;
        i = (THUMBNAIL_SIZE_LAST as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as gssize;
        while i >= 0 as gssize {
            filename = g_build_filename(
                g_get_user_cache_dir(),
                b"thumbnails\0" as *const u8 as *const ::core::ffi::c_char,
                safe_c2rust_get_thumbnail_dirname_from_size(i as ThumbnailSize),
                basename,
                NULL_0,
            ) as *mut ::core::ffi::c_char;
            if g_file_test(filename, G_FILE_TEST_IS_REGULAR) != 0 {
                break;
            }
            let mut _pp_0: *mut *mut ::core::ffi::c_char = &raw mut filename;
            let mut _ptr_0: *mut ::core::ffi::c_char = *_pp_0;
            *_pp_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if !_ptr_0.is_null() {
                g_free(_ptr_0 as gpointer);
            }
            i -= 1;
        }
    }
    if !filename.is_null() {
        _g_file_info_set_attribute_byte_string_by_id(info, path_attr_id, filename);
        _g_file_info_set_attribute_boolean_by_id(
            info,
            is_valid_attr_id,
            thumbnail_verify(filename, uri, stat_buf),
        );
    } else {
        filename = g_build_filename(
            g_get_user_cache_dir(),
            b"thumbnails\0" as *const u8 as *const ::core::ffi::c_char,
            b"fail\0" as *const u8 as *const ::core::ffi::c_char,
            b"gnome-thumbnail-factory\0" as *const u8 as *const ::core::ffi::c_char,
            basename,
            NULL_0,
        ) as *mut ::core::ffi::c_char;
        if g_file_test(filename, G_FILE_TEST_IS_REGULAR) != 0 {
            _g_file_info_set_attribute_boolean_by_id(info, failed_attr_id, TRUE);
            _g_file_info_set_attribute_boolean_by_id(
                info,
                is_valid_attr_id,
                thumbnail_verify(filename, uri, stat_buf),
            );
        }
    }
    g_free(basename as gpointer);
    g_free(filename as gpointer);
    g_free(uri as gpointer);
}
static mut safe_c2rust_g__hidden_cache_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_hidden_cache: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut safe_c2rust_hidden_cache_source: *mut GSource =
    ::core::ptr::null::<GSource>() as *mut GSource;
static mut safe_c2rust_hidden_cache_ttl_secs: guint = 5 as guint;
static mut safe_c2rust_hidden_cache_ttl_jitter_secs: guint = 2 as guint;
unsafe extern "C" fn safe_c2rust_remove_from_hidden_cache(mut user_data: gpointer) -> gboolean {
    let mut data: *mut HiddenCacheData = ::core::ptr::null_mut::<HiddenCacheData>();
    let mut iter: GHashTableIter = _GHashTableIter {
        dummy1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy3: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        dummy4: 0,
        dummy5: 0,
        dummy6: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut retval: gboolean = 0;
    let mut timestamp_secs: gint64 = 0;
    g_mutex_lock(&raw mut safe_c2rust_g__hidden_cache_lock);
    timestamp_secs = g_source_get_time(safe_c2rust_hidden_cache_source) / G_USEC_PER_SEC as gint64;
    g_hash_table_iter_init(&raw mut iter, safe_c2rust_hidden_cache);
    while g_hash_table_iter_next(
        &raw mut iter,
        ::core::ptr::null_mut::<gpointer>(),
        &raw mut data as *mut gpointer,
    ) != 0
    {
        if timestamp_secs > (*data).timestamp_secs + safe_c2rust_hidden_cache_ttl_secs as gint64 {
            g_hash_table_iter_remove(&raw mut iter);
        }
    }
    if g_hash_table_size(safe_c2rust_hidden_cache) == 0 as guint {
        let mut _pp: *mut *mut GSource = &raw mut safe_c2rust_hidden_cache_source;
        let mut _ptr: *mut GSource = *_pp;
        *_pp = ::core::ptr::null_mut::<GSource>();
        if !_ptr.is_null() {
            g_source_unref(_ptr as *mut GSource);
        }
        retval = G_SOURCE_REMOVE as gboolean;
    } else {
        retval = G_SOURCE_CONTINUE as gboolean;
    }
    g_mutex_unlock(&raw mut safe_c2rust_g__hidden_cache_lock);
    return retval;
}
unsafe extern "C" fn safe_c2rust_read_hidden_file(mut dirname: *const gchar) -> *mut GHashTable {
    let mut contents: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut filename: *mut gchar = ::core::ptr::null_mut::<gchar>();
    filename = g_build_path(
        b"/\0" as *const u8 as *const gchar,
        dirname,
        b".hidden\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    g_file_get_contents(
        filename,
        &raw mut contents,
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<*mut GError>(),
    );
    g_free(filename as gpointer);
    if !contents.is_null() {
        let mut table: *mut GHashTable = ::core::ptr::null_mut::<GHashTable>();
        let mut lines: *mut *mut gchar = ::core::ptr::null_mut::<*mut gchar>();
        let mut i: gint = 0;
        table = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            None,
        );
        lines = g_strsplit(contents, b"\n\0" as *const u8 as *const gchar, 0 as gint);
        g_free(contents as gpointer);
        i = 0 as ::core::ffi::c_int as gint;
        while !(*lines.offset(i as isize)).is_null() {
            g_hash_table_add(table, *lines.offset(i as isize) as gpointer);
            i += 1;
        }
        g_free(lines as gpointer);
        return table;
    } else {
        return ::core::ptr::null_mut::<GHashTable>();
    };
}
unsafe extern "C" fn safe_c2rust_free_hidden_file_data(mut user_data: gpointer) {
    let mut data: *mut HiddenCacheData = user_data as *mut HiddenCacheData;
    let mut _pp: *mut *mut GHashTable = &raw mut (*data).hidden_files;
    let mut _ptr: *mut GHashTable = *_pp;
    *_pp = ::core::ptr::null_mut::<GHashTable>();
    if !_ptr.is_null() {
        g_hash_table_unref(_ptr as *mut GHashTable);
    }
    g_free(data as gpointer);
}
unsafe extern "C" fn safe_c2rust_file_is_hidden(
    mut path: *const gchar,
    mut basename: *const gchar,
) -> gboolean {
    let mut data: *mut HiddenCacheData = ::core::ptr::null_mut::<HiddenCacheData>();
    let mut result: gboolean = 0;
    let mut dirname: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut table: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    dirname = g_path_get_dirname(path);
    g_mutex_lock(&raw mut safe_c2rust_g__hidden_cache_lock);
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if safe_c2rust_hidden_cache.is_null() {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_hidden_cache = g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            Some(safe_c2rust_free_hidden_file_data as unsafe extern "C" fn(gpointer) -> ()),
        );
    }
    if g_hash_table_lookup_extended(
        safe_c2rust_hidden_cache,
        dirname as gconstpointer,
        ::core::ptr::null_mut::<gpointer>(),
        &raw mut data as *mut gpointer,
    ) == 0
    {
        data = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<HiddenCacheData>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut HiddenCacheData;
        table = safe_c2rust_read_hidden_file(dirname) as gpointer;
        (*data).hidden_files = table as *mut GHashTable;
        (*data).timestamp_secs = g_get_monotonic_time() / G_USEC_PER_SEC as gint64;
        g_hash_table_insert(
            safe_c2rust_hidden_cache,
            safe_c2rust_g_strdup_inline(dirname) as gpointer,
            data as gpointer,
        );
        if safe_c2rust_hidden_cache_source.is_null() {
            safe_c2rust_hidden_cache_source = g_timeout_source_new_seconds(
                safe_c2rust_hidden_cache_ttl_secs
                    .wrapping_add(safe_c2rust_hidden_cache_ttl_jitter_secs),
            );
            g_source_set_priority(safe_c2rust_hidden_cache_source, G_PRIORITY_DEFAULT);
            g_source_set_static_name(
                safe_c2rust_hidden_cache_source,
                b"[gio] remove_from_hidden_cache\0" as *const u8 as *const ::core::ffi::c_char,
            );
            g_source_set_callback(
                safe_c2rust_hidden_cache_source,
                Some(
                    safe_c2rust_remove_from_hidden_cache
                        as unsafe extern "C" fn(gpointer) -> gboolean,
                ),
                NULL_0,
                None,
            );
            g_source_attach(
                safe_c2rust_hidden_cache_source,
                (*glib__private__())
                    .g_get_worker_context
                    .expect("non-null function pointer")(),
            );
        }
    } else {
        table = (*data).hidden_files as gpointer;
    }
    result = (!table.is_null()
        && g_hash_table_contains(table as *mut GHashTable, basename as gconstpointer) != 0)
        as ::core::ffi::c_int as gboolean;
    g_mutex_unlock(&raw mut safe_c2rust_g__hidden_cache_lock);
    g_free(dirname as gpointer);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_info_get_nostat(
    mut info: *mut GFileInfo,
    mut basename: *const ::core::ffi::c_char,
    mut path: *const ::core::ffi::c_char,
    mut attribute_matcher: *mut GFileAttributeMatcher,
) {
    g_file_info_set_name(info, basename);
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_STANDARD_DISPLAY_NAME as guint32,
    ) != 0
    {
        let mut display_name: *mut ::core::ffi::c_char =
            g_filename_display_basename(path as *const gchar) as *mut ::core::ffi::c_char;
        if !strstr(
            display_name,
            b"\xEF\xBF\xBD\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            let mut p: *mut ::core::ffi::c_char = display_name;
            display_name = g_strconcat(
                display_name,
                glib_gettext(b" (invalid encoding)\0" as *const u8 as *const gchar),
                NULL_0,
            ) as *mut ::core::ffi::c_char;
            g_free(p as gpointer);
        }
        g_file_info_set_display_name(info, display_name);
        g_free(display_name as gpointer);
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_STANDARD_EDIT_NAME as guint32,
    ) != 0
    {
        let mut edit_name: *mut ::core::ffi::c_char =
            g_filename_display_basename(path as *const gchar) as *mut ::core::ffi::c_char;
        g_file_info_set_edit_name(info, edit_name);
        g_free(edit_name as gpointer);
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_STANDARD_COPY_NAME as guint32,
    ) != 0
    {
        let mut copy_name: *mut ::core::ffi::c_char = g_filename_to_utf8(
            basename as *const gchar,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        ) as *mut ::core::ffi::c_char;
        if !copy_name.is_null() {
            _g_file_info_set_attribute_string_by_id(
                info,
                G_FILE_ATTRIBUTE_ID_STANDARD_COPY_NAME as guint32,
                copy_name,
            );
        }
        g_free(copy_name as gpointer);
    }
}
unsafe extern "C" fn safe_c2rust_get_icon_name(
    mut path: *const ::core::ffi::c_char,
    mut use_symbolic: gboolean,
    mut with_fallbacks_out: *mut gboolean,
) -> *const ::core::ffi::c_char {
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut with_fallbacks: gboolean = TRUE;
    if g_strcmp0(path, g_get_home_dir() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        name = if use_symbolic != 0 {
            b"user-home-symbolic\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"user-home\0" as *const u8 as *const ::core::ffi::c_char
        };
        with_fallbacks = FALSE as gboolean;
    } else if g_strcmp0(
        path,
        g_get_user_special_dir(G_USER_DIRECTORY_DESKTOP) as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        name = if use_symbolic != 0 {
            b"user-desktop-symbolic\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"user-desktop\0" as *const u8 as *const ::core::ffi::c_char
        };
        with_fallbacks = FALSE as gboolean;
    } else if g_strcmp0(
        path,
        g_get_user_special_dir(G_USER_DIRECTORY_DOCUMENTS) as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        name = if use_symbolic != 0 {
            b"folder-documents-symbolic\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"folder-documents\0" as *const u8 as *const ::core::ffi::c_char
        };
    } else if g_strcmp0(
        path,
        g_get_user_special_dir(G_USER_DIRECTORY_DOWNLOAD) as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        name = if use_symbolic != 0 {
            b"folder-download-symbolic\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"folder-download\0" as *const u8 as *const ::core::ffi::c_char
        };
    } else if g_strcmp0(
        path,
        g_get_user_special_dir(G_USER_DIRECTORY_MUSIC) as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        name = if use_symbolic != 0 {
            b"folder-music-symbolic\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"folder-music\0" as *const u8 as *const ::core::ffi::c_char
        };
    } else if g_strcmp0(
        path,
        g_get_user_special_dir(G_USER_DIRECTORY_PICTURES) as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        name = if use_symbolic != 0 {
            b"folder-pictures-symbolic\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"folder-pictures\0" as *const u8 as *const ::core::ffi::c_char
        };
    } else if g_strcmp0(
        path,
        g_get_user_special_dir(G_USER_DIRECTORY_PUBLIC_SHARE) as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        name = if use_symbolic != 0 {
            b"folder-publicshare-symbolic\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"folder-publicshare\0" as *const u8 as *const ::core::ffi::c_char
        };
    } else if g_strcmp0(
        path,
        g_get_user_special_dir(G_USER_DIRECTORY_TEMPLATES) as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        name = if use_symbolic != 0 {
            b"folder-templates-symbolic\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"folder-templates\0" as *const u8 as *const ::core::ffi::c_char
        };
    } else if g_strcmp0(
        path,
        g_get_user_special_dir(G_USER_DIRECTORY_VIDEOS) as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        name = if use_symbolic != 0 {
            b"folder-videos-symbolic\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"folder-videos\0" as *const u8 as *const ::core::ffi::c_char
        };
    } else {
        name = ::core::ptr::null::<::core::ffi::c_char>();
    }
    if !with_fallbacks_out.is_null() {
        *with_fallbacks_out = with_fallbacks;
    }
    return name;
}
unsafe extern "C" fn safe_c2rust_get_icon(
    mut path: *const ::core::ffi::c_char,
    mut content_type: *const ::core::ffi::c_char,
    mut use_symbolic: gboolean,
) -> *mut GIcon {
    let mut icon: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
    let mut icon_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut with_fallbacks: gboolean = 0;
    icon_name = safe_c2rust_get_icon_name(path, use_symbolic, &raw mut with_fallbacks);
    if !icon_name.is_null() {
        if with_fallbacks != 0 {
            icon = g_themed_icon_new_with_default_fallbacks(icon_name);
        } else {
            icon = g_themed_icon_new(icon_name);
        }
    } else if use_symbolic != 0 {
        icon = g_content_type_get_symbolic_icon(content_type as *const gchar);
    } else {
        icon = g_content_type_get_icon(content_type as *const gchar);
    }
    return icon;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_info_get(
    mut basename: *const ::core::ffi::c_char,
    mut path: *const ::core::ffi::c_char,
    mut attribute_matcher: *mut GFileAttributeMatcher,
    mut flags: GFileQueryInfoFlags,
    mut parent_info: *mut GLocalParentFileInfo,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut statbuf: statx = statx {
        stx_mask: 0,
        stx_blksize: 0,
        stx_attributes: 0,
        stx_nlink: 0,
        stx_uid: 0,
        stx_gid: 0,
        stx_mode: 0,
        __spare0: [0; 1],
        stx_ino: 0,
        stx_size: 0,
        stx_blocks: 0,
        stx_attributes_mask: 0,
        stx_atime: statx_timestamp {
            tv_sec: 0,
            tv_nsec: 0,
            __reserved: 0,
        },
        stx_btime: statx_timestamp {
            tv_sec: 0,
            tv_nsec: 0,
            __reserved: 0,
        },
        stx_ctime: statx_timestamp {
            tv_sec: 0,
            tv_nsec: 0,
            __reserved: 0,
        },
        stx_mtime: statx_timestamp {
            tv_sec: 0,
            tv_nsec: 0,
            __reserved: 0,
        },
        stx_rdev_major: 0,
        stx_rdev_minor: 0,
        stx_dev_major: 0,
        stx_dev_minor: 0,
        stx_mnt_id: 0,
        stx_dio_mem_align: 0,
        stx_dio_offset_align: 0,
        __spare3: [0; 12],
    };
    let mut statbuf2: statx = statx {
        stx_mask: 0,
        stx_blksize: 0,
        stx_attributes: 0,
        stx_nlink: 0,
        stx_uid: 0,
        stx_gid: 0,
        stx_mode: 0,
        __spare0: [0; 1],
        stx_ino: 0,
        stx_size: 0,
        stx_blocks: 0,
        stx_attributes_mask: 0,
        stx_atime: statx_timestamp {
            tv_sec: 0,
            tv_nsec: 0,
            __reserved: 0,
        },
        stx_btime: statx_timestamp {
            tv_sec: 0,
            tv_nsec: 0,
            __reserved: 0,
        },
        stx_ctime: statx_timestamp {
            tv_sec: 0,
            tv_nsec: 0,
            __reserved: 0,
        },
        stx_mtime: statx_timestamp {
            tv_sec: 0,
            tv_nsec: 0,
            __reserved: 0,
        },
        stx_rdev_major: 0,
        stx_rdev_minor: 0,
        stx_dev_major: 0,
        stx_dev_minor: 0,
        stx_mnt_id: 0,
        stx_dio_mem_align: 0,
        stx_dio_offset_align: 0,
        __spare3: [0; 12],
    };
    let mut res: ::core::ffi::c_int = 0;
    let mut stat_ok: gboolean = 0;
    let mut is_symlink: gboolean = 0;
    let mut symlink_broken: gboolean = 0;
    let mut symlink_target: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut vfs: *mut GVfs = ::core::ptr::null_mut::<GVfs>();
    let mut class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
    let mut device: guint64 = 0;
    info = g_file_info_new();
    g_file_info_set_attribute_mask(info, attribute_matcher);
    safe_c2rust__g_local_file_info_get_nostat(info, basename, path, attribute_matcher);
    if attribute_matcher.is_null() {
        g_file_info_unset_attribute_mask(info);
        return info;
    }
    res = safe_c2rust_g_local_file_lstat(
        path,
        (G_LOCAL_FILE_STAT_FIELD_BASIC_STATS
            | G_LOCAL_FILE_STAT_FIELD_BTIME as ::core::ffi::c_int as ::core::ffi::c_uint)
            as GLocalFileStatField,
        (G_LOCAL_FILE_STAT_FIELD_ALL
            & !(G_LOCAL_FILE_STAT_FIELD_BTIME as ::core::ffi::c_int) as ::core::ffi::c_uint
            & !(G_LOCAL_FILE_STAT_FIELD_ATIME as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as GLocalFileStatField,
        &raw mut statbuf,
    );
    if res == -(1 as ::core::ffi::c_int) {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        if errsv != EACCES {
            let mut display_name: *mut ::core::ffi::c_char =
                g_filename_display_name(path as *const gchar) as *mut ::core::ffi::c_char;
            g_object_unref(info as gpointer);
            g_set_error(
                error,
                g_io_error_quark(),
                g_io_error_from_errno(errsv as gint) as gint,
                glib_gettext(
                    b"Error when getting information for file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0"
                        as *const u8 as *const gchar,
                ),
                display_name,
                g_strerror(errsv as gint),
            );
            g_free(display_name as gpointer);
            return ::core::ptr::null_mut::<GFileInfo>();
        }
    }
    stat_ok = (res != -(1 as ::core::ffi::c_int)) as ::core::ffi::c_int as gboolean;
    if stat_ok != 0 {
        device = safe_c2rust__g_stat_dev(&raw mut statbuf) as guint64;
    } else {
        device = 0 as guint64;
    }
    is_symlink = (stat_ok != 0
        && safe_c2rust__g_stat_mode(&raw mut statbuf) as ::core::ffi::c_int & __S_IFMT
            == 0o120000 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
    symlink_broken = FALSE as gboolean;
    if is_symlink != 0 {
        g_file_info_set_is_symlink(info, TRUE);
        if flags as ::core::ffi::c_uint
            & G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
        {
            res = safe_c2rust_g_local_file_stat(
                path,
                (G_LOCAL_FILE_STAT_FIELD_BASIC_STATS
                    | G_LOCAL_FILE_STAT_FIELD_BTIME as ::core::ffi::c_int as ::core::ffi::c_uint)
                    as GLocalFileStatField,
                (G_LOCAL_FILE_STAT_FIELD_ALL
                    & !(G_LOCAL_FILE_STAT_FIELD_BTIME as ::core::ffi::c_int) as ::core::ffi::c_uint
                    & !(G_LOCAL_FILE_STAT_FIELD_ATIME as ::core::ffi::c_int) as ::core::ffi::c_uint)
                    as GLocalFileStatField,
                &raw mut statbuf2,
            );
            if res != -(1 as ::core::ffi::c_int) {
                statbuf = statbuf2;
                stat_ok = TRUE as gboolean;
            } else {
                symlink_broken = TRUE as gboolean;
            }
        }
    } else {
        g_file_info_set_is_symlink(info, FALSE);
    }
    if stat_ok != 0 {
        safe_c2rust_set_info_from_stat(info, &raw mut statbuf, attribute_matcher);
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_STANDARD_IS_HIDDEN as guint32,
    ) != 0
    {
        g_file_info_set_is_hidden(
            info,
            (!basename.is_null()
                && (*basename.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '.' as i32
                    || safe_c2rust_file_is_hidden(path as *const gchar, basename as *const gchar)
                        != 0
                    || stat_ok != 0
                        && _g_local_file_is_lost_found_dir(
                            path,
                            safe_c2rust__g_stat_dev(&raw mut statbuf),
                        ) != 0)) as ::core::ffi::c_int,
        );
    }
    _g_file_info_set_attribute_boolean_by_id(
        info,
        G_FILE_ATTRIBUTE_ID_STANDARD_IS_BACKUP as guint32,
        (!basename.is_null()
            && *basename.offset(strlen(basename).wrapping_sub(1 as size_t) as isize)
                as ::core::ffi::c_int
                == '~' as i32
            && (stat_ok != 0
                && safe_c2rust__g_stat_mode(&raw mut statbuf) as ::core::ffi::c_int & __S_IFMT
                    == 0o100000 as ::core::ffi::c_int)) as ::core::ffi::c_int,
    );
    symlink_target = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if is_symlink != 0 {
        symlink_target = safe_c2rust_read_link(path as *const gchar) as *mut ::core::ffi::c_char;
        if !symlink_target.is_null()
            && _g_file_attribute_matcher_matches_id(
                attribute_matcher,
                G_FILE_ATTRIBUTE_ID_STANDARD_SYMLINK_TARGET as guint32,
            ) != 0
        {
            g_file_info_set_symlink_target(info, symlink_target);
        }
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_STANDARD_CONTENT_TYPE as guint32,
    ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_STANDARD_ICON as guint32,
        ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_STANDARD_SYMBOLIC_ICON as guint32,
        ) != 0
    {
        let mut content_type: *mut ::core::ffi::c_char = safe_c2rust_get_content_type(
            basename,
            path,
            if stat_ok != 0 {
                &raw mut statbuf
            } else {
                ::core::ptr::null_mut::<statx>()
            },
            is_symlink,
            symlink_broken,
            flags,
            FALSE,
        );
        if !content_type.is_null() {
            g_file_info_set_content_type(info, content_type);
            if _g_file_attribute_matcher_matches_id(
                attribute_matcher,
                G_FILE_ATTRIBUTE_ID_STANDARD_ICON as guint32,
            ) != 0
                || _g_file_attribute_matcher_matches_id(
                    attribute_matcher,
                    G_FILE_ATTRIBUTE_ID_STANDARD_SYMBOLIC_ICON as guint32,
                ) != 0
            {
                let mut icon: *mut GIcon = ::core::ptr::null_mut::<GIcon>();
                icon = safe_c2rust_get_icon(path, content_type, FALSE);
                if !icon.is_null() {
                    g_file_info_set_icon(info, icon);
                    g_object_unref(icon as gpointer);
                }
                icon = safe_c2rust_get_icon(path, content_type, TRUE);
                if !icon.is_null() {
                    g_file_info_set_symbolic_icon(info, icon);
                    g_object_unref(icon as gpointer);
                }
            }
            g_free(content_type as gpointer);
        }
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_STANDARD_FAST_CONTENT_TYPE as guint32,
    ) != 0
    {
        let mut content_type_0: *mut ::core::ffi::c_char = safe_c2rust_get_content_type(
            basename,
            path,
            if stat_ok != 0 {
                &raw mut statbuf
            } else {
                ::core::ptr::null_mut::<statx>()
            },
            is_symlink,
            symlink_broken,
            flags,
            TRUE,
        );
        if !content_type_0.is_null() {
            _g_file_info_set_attribute_string_by_id(
                info,
                G_FILE_ATTRIBUTE_ID_STANDARD_FAST_CONTENT_TYPE as guint32,
                content_type_0,
            );
            g_free(content_type_0 as gpointer);
        }
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_OWNER_USER as guint32,
    ) != 0
    {
        let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if stat_ok != 0 {
            name = safe_c2rust_get_username_from_uid(
                safe_c2rust__g_stat_uid(&raw mut statbuf) as uid_t
            );
        }
        if !name.is_null() {
            _g_file_info_set_attribute_string_by_id(
                info,
                G_FILE_ATTRIBUTE_ID_OWNER_USER as guint32,
                name,
            );
        }
        g_free(name as gpointer);
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_OWNER_USER_REAL as guint32,
    ) != 0
    {
        let mut name_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if stat_ok != 0 {
            name_0 = safe_c2rust_get_realname_from_uid(
                safe_c2rust__g_stat_uid(&raw mut statbuf) as uid_t
            );
        }
        if !name_0.is_null() {
            _g_file_info_set_attribute_string_by_id(
                info,
                G_FILE_ATTRIBUTE_ID_OWNER_USER_REAL as guint32,
                name_0,
            );
        }
        g_free(name_0 as gpointer);
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_OWNER_GROUP as guint32,
    ) != 0
    {
        let mut name_1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if stat_ok != 0 {
            name_1 = safe_c2rust_get_groupname_from_gid(
                safe_c2rust__g_stat_gid(&raw mut statbuf) as gid_t
            );
        }
        if !name_1.is_null() {
            _g_file_info_set_attribute_string_by_id(
                info,
                G_FILE_ATTRIBUTE_ID_OWNER_GROUP as guint32,
                name_1,
            );
        }
        g_free(name_1 as gpointer);
    }
    if stat_ok != 0
        && !parent_info.is_null()
        && (*parent_info).device != 0 as dev_t
        && _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_UNIX_IS_MOUNTPOINT as guint32,
        ) != 0
    {
        _g_file_info_set_attribute_boolean_by_id(
            info,
            G_FILE_ATTRIBUTE_ID_UNIX_IS_MOUNTPOINT as guint32,
            (safe_c2rust__g_stat_dev(&raw mut statbuf) != (*parent_info).device
                || safe_c2rust__g_stat_ino(&raw mut statbuf) == (*parent_info).inode)
                as ::core::ffi::c_int,
        );
    }
    if stat_ok != 0 {
        safe_c2rust_get_access_rights(
            attribute_matcher,
            info,
            path as *const gchar,
            &raw mut statbuf,
            parent_info,
        );
    }
    safe_c2rust_get_selinux_context(
        path,
        info,
        attribute_matcher,
        (flags as ::core::ffi::c_uint
            & G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
    );
    safe_c2rust_get_xattrs(
        path,
        TRUE,
        info,
        attribute_matcher,
        (flags as ::core::ffi::c_uint
            & G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
    );
    safe_c2rust_get_xattrs(
        path,
        FALSE,
        info,
        attribute_matcher,
        (flags as ::core::ffi::c_uint
            & G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint) as ::core::ffi::c_int,
    );
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH as guint32,
    ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID as guint32,
        ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED as guint32,
        ) != 0
    {
        safe_c2rust_get_thumbnail_attributes(
            path,
            info,
            if stat_ok != 0 {
                &raw mut statbuf
            } else {
                ::core::ptr::null_mut::<statx>()
            },
            THUMBNAIL_SIZE_AUTO,
        );
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_NORMAL as guint32,
    ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_NORMAL as guint32,
        ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_NORMAL as guint32,
        ) != 0
    {
        safe_c2rust_get_thumbnail_attributes(
            path,
            info,
            if stat_ok != 0 {
                &raw mut statbuf
            } else {
                ::core::ptr::null_mut::<statx>()
            },
            THUMBNAIL_SIZE_NORMAL,
        );
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_LARGE as guint32,
    ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_LARGE as guint32,
        ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_LARGE as guint32,
        ) != 0
    {
        safe_c2rust_get_thumbnail_attributes(
            path,
            info,
            if stat_ok != 0 {
                &raw mut statbuf
            } else {
                ::core::ptr::null_mut::<statx>()
            },
            THUMBNAIL_SIZE_LARGE,
        );
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_XLARGE as guint32,
    ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_XLARGE as guint32,
        ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_XLARGE as guint32,
        ) != 0
    {
        safe_c2rust_get_thumbnail_attributes(
            path,
            info,
            if stat_ok != 0 {
                &raw mut statbuf
            } else {
                ::core::ptr::null_mut::<statx>()
            },
            THUMBNAIL_SIZE_XLARGE,
        );
    }
    if _g_file_attribute_matcher_matches_id(
        attribute_matcher,
        G_FILE_ATTRIBUTE_ID_THUMBNAIL_PATH_XXLARGE as guint32,
    ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_THUMBNAIL_IS_VALID_XXLARGE as guint32,
        ) != 0
        || _g_file_attribute_matcher_matches_id(
            attribute_matcher,
            G_FILE_ATTRIBUTE_ID_THUMBNAILING_FAILED_XXLARGE as guint32,
        ) != 0
    {
        safe_c2rust_get_thumbnail_attributes(
            path,
            info,
            if stat_ok != 0 {
                &raw mut statbuf
            } else {
                ::core::ptr::null_mut::<statx>()
            },
            THUMBNAIL_SIZE_XXLARGE,
        );
    }
    vfs = g_vfs_get_default();
    class = (*(vfs as *mut GTypeInstance)).g_class as *mut GVfsClass;
    if (*class).local_file_add_info.is_some() {
        (*class)
            .local_file_add_info
            .expect("non-null function pointer")(
            vfs,
            path,
            device,
            attribute_matcher,
            info,
            ::core::ptr::null_mut::<GCancellable>(),
            &raw mut (*parent_info).extra_data,
            &raw mut (*parent_info).free_extra_data,
        );
    }
    g_file_info_unset_attribute_mask(info);
    g_free(symlink_target as gpointer);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_info_get_from_fd(
    mut fd: ::core::ffi::c_int,
    mut attributes: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut stat_buf: statx = statx {
        stx_mask: 0,
        stx_blksize: 0,
        stx_attributes: 0,
        stx_nlink: 0,
        stx_uid: 0,
        stx_gid: 0,
        stx_mode: 0,
        __spare0: [0; 1],
        stx_ino: 0,
        stx_size: 0,
        stx_blocks: 0,
        stx_attributes_mask: 0,
        stx_atime: statx_timestamp {
            tv_sec: 0,
            tv_nsec: 0,
            __reserved: 0,
        },
        stx_btime: statx_timestamp {
            tv_sec: 0,
            tv_nsec: 0,
            __reserved: 0,
        },
        stx_ctime: statx_timestamp {
            tv_sec: 0,
            tv_nsec: 0,
            __reserved: 0,
        },
        stx_mtime: statx_timestamp {
            tv_sec: 0,
            tv_nsec: 0,
            __reserved: 0,
        },
        stx_rdev_major: 0,
        stx_rdev_minor: 0,
        stx_dev_major: 0,
        stx_dev_minor: 0,
        stx_mnt_id: 0,
        stx_dio_mem_align: 0,
        stx_dio_offset_align: 0,
        __spare3: [0; 12],
    };
    let mut matcher: *mut GFileAttributeMatcher = ::core::ptr::null_mut::<GFileAttributeMatcher>();
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    if safe_c2rust_g_local_file_fstat(
        fd,
        (G_LOCAL_FILE_STAT_FIELD_BASIC_STATS
            | G_LOCAL_FILE_STAT_FIELD_BTIME as ::core::ffi::c_int as ::core::ffi::c_uint)
            as GLocalFileStatField,
        (G_LOCAL_FILE_STAT_FIELD_ALL
            & !(G_LOCAL_FILE_STAT_FIELD_BTIME as ::core::ffi::c_int) as ::core::ffi::c_uint
            & !(G_LOCAL_FILE_STAT_FIELD_ATIME as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as GLocalFileStatField,
        &raw mut stat_buf,
    ) == -(1 as ::core::ffi::c_int)
    {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(
                b"Error when getting information for file descriptor: %s\0" as *const u8
                    as *const gchar,
            ),
            g_strerror(errsv as gint),
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    info = g_file_info_new();
    matcher = g_file_attribute_matcher_new(attributes);
    g_file_info_set_attribute_mask(info, matcher);
    safe_c2rust_set_info_from_stat(info, &raw mut stat_buf, matcher);
    if _g_file_attribute_matcher_matches_id(matcher, G_FILE_ATTRIBUTE_ID_SELINUX_CONTEXT as guint32)
        != 0
        && is_selinux_enabled() != 0
    {
        let mut context: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if fgetfilecon_raw(fd, &raw mut context) >= 0 as ::core::ffi::c_int {
            _g_file_info_set_attribute_string_by_id(
                info,
                G_FILE_ATTRIBUTE_ID_SELINUX_CONTEXT as guint32,
                context,
            );
            freecon(context);
        }
    }
    safe_c2rust_get_xattrs_from_fd(fd, TRUE, info, matcher);
    safe_c2rust_get_xattrs_from_fd(fd, FALSE, info, matcher);
    g_file_attribute_matcher_unref(matcher);
    g_file_info_unset_attribute_mask(info);
    return info;
}
unsafe extern "C" fn safe_c2rust_get_uint32(
    mut value: *const GFileAttributeValue,
    mut val_out: *mut guint32,
    mut error: *mut *mut GError,
) -> gboolean {
    if (*value).type_0() as ::core::ffi::c_int != G_FILE_ATTRIBUTE_TYPE_UINT32 as ::core::ffi::c_int
    {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Invalid attribute type (uint32 expected)\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    *val_out = (*value).u.uint32;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_get_uint64(
    mut value: *const GFileAttributeValue,
    mut val_out: *mut guint64,
    mut error: *mut *mut GError,
) -> gboolean {
    if (*value).type_0() as ::core::ffi::c_int != G_FILE_ATTRIBUTE_TYPE_UINT64 as ::core::ffi::c_int
    {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Invalid attribute type (uint64 expected)\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    *val_out = (*value).u.uint64;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_get_byte_string(
    mut value: *const GFileAttributeValue,
    mut val_out: *mut *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    if (*value).type_0() as ::core::ffi::c_int
        != G_FILE_ATTRIBUTE_TYPE_BYTE_STRING as ::core::ffi::c_int
    {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Invalid attribute type (byte string expected)\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    *val_out = (*value).u.string;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_get_string(
    mut value: *const GFileAttributeValue,
    mut val_out: *mut *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    if (*value).type_0() as ::core::ffi::c_int != G_FILE_ATTRIBUTE_TYPE_STRING as ::core::ffi::c_int
    {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Invalid attribute type (byte string expected)\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    *val_out = (*value).u.string;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_set_unix_mode(
    mut filename: *mut ::core::ffi::c_char,
    mut flags: GFileQueryInfoFlags,
    mut value: *const GFileAttributeValue,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut val: guint32 = 0 as guint32;
    let mut res: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if safe_c2rust_get_uint32(value, &raw mut val, error) == 0 {
        return FALSE;
    }
    if flags as ::core::ffi::c_uint
        & G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        res = lchmod(filename, val as __mode_t);
    } else {
        res = chmod(filename, val as __mode_t);
    }
    if res == -(1 as ::core::ffi::c_int) {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(b"Error setting permissions: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv as gint),
        );
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_set_unix_uid_gid(
    mut filename: *mut ::core::ffi::c_char,
    mut uid_value: *const GFileAttributeValue,
    mut gid_value: *const GFileAttributeValue,
    mut flags: GFileQueryInfoFlags,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut res: ::core::ffi::c_int = 0;
    let mut val: guint32 = 0 as guint32;
    let mut uid: uid_t = 0;
    let mut gid: gid_t = 0;
    if !uid_value.is_null() {
        if safe_c2rust_get_uint32(uid_value, &raw mut val, error) == 0 {
            return FALSE;
        }
        uid = val as uid_t;
    } else {
        uid = -(1 as ::core::ffi::c_int) as uid_t;
    }
    if !gid_value.is_null() {
        if safe_c2rust_get_uint32(gid_value, &raw mut val, error) == 0 {
            return FALSE;
        }
        gid = val as gid_t;
    } else {
        gid = -(1 as ::core::ffi::c_int) as gid_t;
    }
    if flags as ::core::ffi::c_uint
        & G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        res = lchown(filename, uid as __uid_t, gid as __gid_t);
    } else {
        res = chown(filename, uid as __uid_t, gid as __gid_t);
    }
    if res == -(1 as ::core::ffi::c_int) {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(b"Error setting owner: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv as gint),
        );
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_set_symlink(
    mut filename: *mut ::core::ffi::c_char,
    mut value: *const GFileAttributeValue,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut val: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
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
    if safe_c2rust_get_byte_string(value, &raw mut val, error) == 0 {
        return FALSE;
    }
    if val.is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"symlink must be non-NULL\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    if lstat(filename, &raw mut statbuf) != 0 {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(b"Error setting symlink: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv as gint),
        );
        return FALSE;
    }
    if !(statbuf.st_mode & __S_IFMT as __mode_t == 0o120000 as __mode_t) {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SYMBOLIC_LINK as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Error setting symlink: file is not a symlink\0" as *const u8 as *const gchar,
            ),
        );
        return FALSE;
    }
    if g_unlink(filename) != 0 {
        let mut errsv_0: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv_0 as gint) as gint,
            glib_gettext(b"Error setting symlink: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv_0 as gint),
        );
        return FALSE;
    }
    if symlink(filename, val) != 0 as ::core::ffi::c_int {
        let mut errsv_1: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv_1 as gint) as gint,
            glib_gettext(b"Error setting symlink: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv_1 as gint),
        );
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_lazy_stat(
    mut filename: *const ::core::ffi::c_char,
    mut statbuf: *mut GStatBuf,
    mut called_stat: *mut gboolean,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 0;
    if *called_stat != 0 {
        return 0 as ::core::ffi::c_int;
    }
    res = stat(filename, statbuf as *mut stat);
    if res == 0 as ::core::ffi::c_int {
        *called_stat = TRUE as gboolean;
    }
    return res;
}
unsafe extern "C" fn safe_c2rust_set_mtime_atime(
    mut filename: *mut ::core::ffi::c_char,
    mut mtime_value: *const GFileAttributeValue,
    mut mtime_usec_value: *const GFileAttributeValue,
    mut mtime_nsec_value: *const GFileAttributeValue,
    mut atime_value: *const GFileAttributeValue,
    mut atime_usec_value: *const GFileAttributeValue,
    mut atime_nsec_value: *const GFileAttributeValue,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut res: ::core::ffi::c_int = 0;
    let mut val: guint64 = 0 as guint64;
    let mut statbuf: GStatBuf = stat {
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
    let mut got_stat: gboolean = FALSE;
    let mut times_n: [timespec; 2] = [
        timespec {
            tv_sec: 0 as __time_t,
            tv_nsec: 0 as __syscall_slong_t,
        },
        timespec {
            tv_sec: 0 as __time_t,
            tv_nsec: 0 as __syscall_slong_t,
        },
    ];
    if !atime_value.is_null() {
        if safe_c2rust_get_uint64(atime_value, &raw mut val, error) == 0 {
            return FALSE;
        }
        times_n[0 as ::core::ffi::c_int as usize].tv_sec = val as __time_t;
    } else if safe_c2rust_lazy_stat(filename, &raw mut statbuf, &raw mut got_stat)
        == 0 as ::core::ffi::c_int
    {
        times_n[0 as ::core::ffi::c_int as usize].tv_sec = statbuf.st_atim.tv_sec;
        times_n[0 as ::core::ffi::c_int as usize].tv_nsec = statbuf.st_atim.tv_nsec;
    }
    if !atime_usec_value.is_null() {
        let mut val_usec: guint32 = 0 as guint32;
        if safe_c2rust_get_uint32(atime_usec_value, &raw mut val_usec, error) == 0 {
            return FALSE;
        }
        times_n[0 as ::core::ffi::c_int as usize].tv_nsec =
            val_usec.wrapping_mul(1000 as guint32) as __syscall_slong_t;
    }
    if !atime_nsec_value.is_null() {
        let mut val_nsec: guint32 = 0 as guint32;
        if safe_c2rust_get_uint32(atime_nsec_value, &raw mut val_nsec, error) == 0 {
            return FALSE;
        }
        times_n[0 as ::core::ffi::c_int as usize].tv_nsec = val_nsec as __syscall_slong_t;
    }
    if !mtime_value.is_null() {
        if safe_c2rust_get_uint64(mtime_value, &raw mut val, error) == 0 {
            return FALSE;
        }
        times_n[1 as ::core::ffi::c_int as usize].tv_sec = val as __time_t;
    } else if safe_c2rust_lazy_stat(filename, &raw mut statbuf, &raw mut got_stat)
        == 0 as ::core::ffi::c_int
    {
        times_n[1 as ::core::ffi::c_int as usize].tv_sec = statbuf.st_mtim.tv_sec;
        times_n[1 as ::core::ffi::c_int as usize].tv_nsec = statbuf.st_mtim.tv_nsec;
    }
    if !mtime_usec_value.is_null() {
        let mut val_usec_0: guint32 = 0 as guint32;
        if safe_c2rust_get_uint32(mtime_usec_value, &raw mut val_usec_0, error) == 0 {
            return FALSE;
        }
        times_n[1 as ::core::ffi::c_int as usize].tv_nsec =
            val_usec_0.wrapping_mul(1000 as guint32) as __syscall_slong_t;
    }
    if !mtime_nsec_value.is_null() {
        let mut val_nsec_0: guint32 = 0 as guint32;
        if safe_c2rust_get_uint32(mtime_nsec_value, &raw mut val_nsec_0, error) == 0 {
            return FALSE;
        }
        times_n[1 as ::core::ffi::c_int as usize].tv_nsec = val_nsec_0 as __syscall_slong_t;
    }
    res = utimensat(
        AT_FDCWD,
        filename,
        &raw mut times_n as *mut timespec as *const timespec,
        0 as ::core::ffi::c_int,
    );
    if res == -(1 as ::core::ffi::c_int) {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(
                b"Error setting modification or access time: %s\0" as *const u8 as *const gchar,
            ),
            g_strerror(errsv as gint),
        );
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_set_selinux_context(
    mut filename: *mut ::core::ffi::c_char,
    mut value: *const GFileAttributeValue,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut val: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if safe_c2rust_get_string(value, &raw mut val, error) == 0 {
        return FALSE;
    }
    if val.is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"SELinux context must be non-NULL\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    if is_selinux_enabled() == 0 {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_ARGUMENT as ::core::ffi::c_int as gint,
            glib_gettext(b"SELinux is not enabled on this system\0" as *const u8 as *const gchar),
        );
        return FALSE;
    }
    if setfilecon_raw(filename, val) < 0 as ::core::ffi::c_int {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(errsv as gint) as gint,
            glib_gettext(b"Error setting SELinux context: %s\0" as *const u8 as *const gchar),
            g_strerror(errsv as gint),
        );
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_info_set_attribute(
    mut filename: *mut ::core::ffi::c_char,
    mut attribute: *const ::core::ffi::c_char,
    mut type_0: GFileAttributeType,
    mut value_p: gpointer,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut value: GFileAttributeValue = {
        let mut init = GFileAttributeValue {
            type_0_status: [0; 2],
            c2rust_padding: [0; 6],
            u: C2RustUnnamed_0 { boolean: 0 },
        };
        init.set_type_0(0 as guint);
        init.set_status(0);
        init
    };
    let mut class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
    let mut vfs: *mut GVfs = ::core::ptr::null_mut::<GVfs>();
    _g_file_attribute_value_set_from_pointer(&raw mut value, type_0, value_p, FALSE);
    if strcmp(attribute, G_FILE_ATTRIBUTE_UNIX_MODE.as_ptr()) == 0 as ::core::ffi::c_int {
        return safe_c2rust_set_unix_mode(filename, flags, &raw mut value, error);
    } else if strcmp(attribute, G_FILE_ATTRIBUTE_UNIX_UID.as_ptr()) == 0 as ::core::ffi::c_int {
        return safe_c2rust_set_unix_uid_gid(
            filename,
            &raw mut value,
            ::core::ptr::null::<GFileAttributeValue>(),
            flags,
            error,
        );
    } else if strcmp(attribute, G_FILE_ATTRIBUTE_UNIX_GID.as_ptr()) == 0 as ::core::ffi::c_int {
        return safe_c2rust_set_unix_uid_gid(
            filename,
            ::core::ptr::null::<GFileAttributeValue>(),
            &raw mut value,
            flags,
            error,
        );
    } else if strcmp(attribute, G_FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET.as_ptr())
        == 0 as ::core::ffi::c_int
    {
        return safe_c2rust_set_symlink(filename, &raw mut value, error);
    } else if strcmp(attribute, G_FILE_ATTRIBUTE_TIME_MODIFIED.as_ptr()) == 0 as ::core::ffi::c_int
    {
        return safe_c2rust_set_mtime_atime(
            filename,
            &raw mut value,
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            error,
        );
    } else if strcmp(attribute, G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC.as_ptr())
        == 0 as ::core::ffi::c_int
    {
        return safe_c2rust_set_mtime_atime(
            filename,
            ::core::ptr::null::<GFileAttributeValue>(),
            &raw mut value,
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            error,
        );
    } else if strcmp(attribute, G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC.as_ptr())
        == 0 as ::core::ffi::c_int
    {
        return safe_c2rust_set_mtime_atime(
            filename,
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            &raw mut value,
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            error,
        );
    } else if strcmp(attribute, G_FILE_ATTRIBUTE_TIME_ACCESS.as_ptr()) == 0 as ::core::ffi::c_int {
        return safe_c2rust_set_mtime_atime(
            filename,
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            &raw mut value,
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            error,
        );
    } else if strcmp(attribute, G_FILE_ATTRIBUTE_TIME_ACCESS_USEC.as_ptr())
        == 0 as ::core::ffi::c_int
    {
        return safe_c2rust_set_mtime_atime(
            filename,
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            &raw mut value,
            ::core::ptr::null::<GFileAttributeValue>(),
            error,
        );
    } else if strcmp(attribute, G_FILE_ATTRIBUTE_TIME_ACCESS_NSEC.as_ptr())
        == 0 as ::core::ffi::c_int
    {
        return safe_c2rust_set_mtime_atime(
            filename,
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            ::core::ptr::null::<GFileAttributeValue>(),
            &raw mut value,
            error,
        );
    } else if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = attribute;
            let __prefix: *const ::core::ffi::c_char =
                b"xattr::\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_22 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_22 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_22
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
            attribute as *const gchar,
            b"xattr::\0" as *const u8 as *const gchar,
        )
    } != 0
    {
        return safe_c2rust_set_xattr(filename, attribute, &raw mut value, error);
    } else if if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = attribute;
            let __prefix: *const ::core::ffi::c_char =
                b"xattr-sys::\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
                if __str.is_null() || __prefix.is_null() {
                    _g_boolean_var_23 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_23 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_23
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
            attribute as *const gchar,
            b"xattr-sys::\0" as *const u8 as *const gchar,
        )
    } != 0
    {
        return safe_c2rust_set_xattr(filename, attribute, &raw mut value, error);
    } else if strcmp(attribute, G_FILE_ATTRIBUTE_SELINUX_CONTEXT.as_ptr())
        == 0 as ::core::ffi::c_int
    {
        return safe_c2rust_set_selinux_context(filename, &raw mut value, error);
    }
    vfs = g_vfs_get_default();
    class = (*(vfs as *mut GTypeInstance)).g_class as *mut GVfsClass;
    if (*class).local_file_set_attributes.is_some() {
        let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
        info = g_file_info_new();
        g_file_info_set_attribute(info, attribute, type_0, value_p);
        if (*class)
            .local_file_set_attributes
            .expect("non-null function pointer")(
            vfs, filename, info, flags, cancellable, error
        ) == 0
        {
            g_object_unref(info as gpointer);
            return FALSE;
        }
        if g_file_info_get_attribute_status(info, attribute) as ::core::ffi::c_uint
            == G_FILE_ATTRIBUTE_STATUS_SET as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            g_object_unref(info as gpointer);
            return TRUE;
        }
        g_object_unref(info as gpointer);
    }
    g_set_error(
        error,
        g_io_error_quark(),
        G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
        glib_gettext(b"Setting attribute %s not supported\0" as *const u8 as *const gchar),
        attribute,
    );
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_info_set_attributes(
    mut filename: *mut ::core::ffi::c_char,
    mut info: *mut GFileInfo,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut value: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut uid: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut gid: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut mtime: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut mtime_usec: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut mtime_nsec: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut atime: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut atime_usec: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut atime_nsec: *mut GFileAttributeValue = ::core::ptr::null_mut::<GFileAttributeValue>();
    let mut status: GFileAttributeStatus = G_FILE_ATTRIBUTE_STATUS_UNSET;
    let mut res: gboolean = 0;
    let mut class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
    let mut vfs: *mut GVfs = ::core::ptr::null_mut::<GVfs>();
    res = TRUE as gboolean;
    value =
        _g_file_info_get_attribute_value(info, G_FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET.as_ptr());
    if !value.is_null() {
        if safe_c2rust_set_symlink(filename, value, error) == 0 {
            (*value).set_status(
                G_FILE_ATTRIBUTE_STATUS_ERROR_SETTING as ::core::ffi::c_int as guint as guint,
            );
            res = FALSE as gboolean;
            error = ::core::ptr::null_mut::<*mut GError>();
        } else {
            (*value)
                .set_status(G_FILE_ATTRIBUTE_STATUS_SET as ::core::ffi::c_int as guint as guint);
        }
    }
    uid = _g_file_info_get_attribute_value(info, G_FILE_ATTRIBUTE_UNIX_UID.as_ptr());
    gid = _g_file_info_get_attribute_value(info, G_FILE_ATTRIBUTE_UNIX_GID.as_ptr());
    if !uid.is_null() || !gid.is_null() {
        if safe_c2rust_set_unix_uid_gid(filename, uid, gid, flags, error) == 0 {
            status = G_FILE_ATTRIBUTE_STATUS_ERROR_SETTING;
            res = FALSE as gboolean;
            error = ::core::ptr::null_mut::<*mut GError>();
        } else {
            status = G_FILE_ATTRIBUTE_STATUS_SET;
        }
        if !uid.is_null() {
            (*uid).set_status(status as guint as guint);
        }
        if !gid.is_null() {
            (*gid).set_status(status as guint as guint);
        }
    }
    value = _g_file_info_get_attribute_value(info, G_FILE_ATTRIBUTE_UNIX_MODE.as_ptr());
    if !value.is_null() {
        if safe_c2rust_set_unix_mode(filename, flags, value, error) == 0 {
            (*value).set_status(
                G_FILE_ATTRIBUTE_STATUS_ERROR_SETTING as ::core::ffi::c_int as guint as guint,
            );
            res = FALSE as gboolean;
            error = ::core::ptr::null_mut::<*mut GError>();
        } else {
            (*value)
                .set_status(G_FILE_ATTRIBUTE_STATUS_SET as ::core::ffi::c_int as guint as guint);
        }
    }
    mtime = _g_file_info_get_attribute_value(info, G_FILE_ATTRIBUTE_TIME_MODIFIED.as_ptr());
    mtime_usec =
        _g_file_info_get_attribute_value(info, G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC.as_ptr());
    mtime_nsec =
        _g_file_info_get_attribute_value(info, G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC.as_ptr());
    atime = _g_file_info_get_attribute_value(info, G_FILE_ATTRIBUTE_TIME_ACCESS.as_ptr());
    atime_usec = _g_file_info_get_attribute_value(info, G_FILE_ATTRIBUTE_TIME_ACCESS_USEC.as_ptr());
    atime_nsec = _g_file_info_get_attribute_value(info, G_FILE_ATTRIBUTE_TIME_ACCESS_NSEC.as_ptr());
    if !mtime.is_null()
        || !mtime_usec.is_null()
        || !mtime_nsec.is_null()
        || !atime.is_null()
        || !atime_usec.is_null()
        || !atime_nsec.is_null()
    {
        if safe_c2rust_set_mtime_atime(
            filename, mtime, mtime_usec, mtime_nsec, atime, atime_usec, atime_nsec, error,
        ) == 0
        {
            status = G_FILE_ATTRIBUTE_STATUS_ERROR_SETTING;
            res = FALSE as gboolean;
            error = ::core::ptr::null_mut::<*mut GError>();
        } else {
            status = G_FILE_ATTRIBUTE_STATUS_SET;
        }
        if !mtime.is_null() {
            (*mtime).set_status(status as guint as guint);
        }
        if !mtime_usec.is_null() {
            (*mtime_usec).set_status(status as guint as guint);
        }
        if !mtime_nsec.is_null() {
            (*mtime_nsec).set_status(status as guint as guint);
        }
        if !atime.is_null() {
            (*atime).set_status(status as guint as guint);
        }
        if !atime_usec.is_null() {
            (*atime_usec).set_status(status as guint as guint);
        }
        if !atime_nsec.is_null() {
            (*atime_nsec).set_status(status as guint as guint);
        }
    }
    if is_selinux_enabled() != 0 {
        value = _g_file_info_get_attribute_value(info, G_FILE_ATTRIBUTE_SELINUX_CONTEXT.as_ptr());
        if !value.is_null() {
            if safe_c2rust_set_selinux_context(filename, value, error) == 0 {
                (*value).set_status(
                    G_FILE_ATTRIBUTE_STATUS_ERROR_SETTING as ::core::ffi::c_int as guint as guint,
                );
                res = FALSE as gboolean;
                error = ::core::ptr::null_mut::<*mut GError>();
            } else {
                (*value).set_status(
                    G_FILE_ATTRIBUTE_STATUS_SET as ::core::ffi::c_int as guint as guint,
                );
            }
        }
    }
    vfs = g_vfs_get_default();
    class = (*(vfs as *mut GTypeInstance)).g_class as *mut GVfsClass;
    if (*class).local_file_set_attributes.is_some() {
        if (*class)
            .local_file_set_attributes
            .expect("non-null function pointer")(
            vfs, filename, info, flags, cancellable, error
        ) == 0
        {
            res = FALSE as gboolean;
            error = ::core::ptr::null_mut::<*mut GError>();
        }
    }
    return res;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
