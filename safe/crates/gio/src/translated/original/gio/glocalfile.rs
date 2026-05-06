extern "C" {
    pub type _GDateTime;
    pub type _GData;
    pub type __dirstream;
    pub type _GDir;
    pub type _GHashTable;
    pub type _GMainContext;
    pub type _GVariant;
    pub type _GAsyncResult;
    pub type _GInputStreamPrivate;
    pub type _GOutputStreamPrivate;
    pub type _GCancellablePrivate;
    pub type _GFileEnumeratorPrivate;
    pub type _GFileMonitorPrivate;
    pub type _GFile;
    pub type _GFileInfo;
    pub type _GFileAttributeMatcher;
    pub type _GFileInputStreamPrivate;
    pub type _GFileOutputStreamPrivate;
    pub type _GFileIOStreamPrivate;
    pub type _GIOStreamPrivate;
    pub type _GIcon;
    pub type _GMount;
    pub type _GMountOperationPrivate;
    pub type _GWakeup;
    pub type _GUnixMountEntry;
    pub type _GUnixMountPoint;
    pub type _GLocalFileOutputStreamPrivate;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn mkdir(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn statx(
        __dirfd: ::core::ffi::c_int,
        __path: *const ::core::ffi::c_char,
        __flags: ::core::ffi::c_int,
        __mask: ::core::ffi::c_uint,
        __buf: *mut statx,
    ) -> ::core::ffi::c_int;
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
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn openat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn statfs(__file: *const ::core::ffi::c_char, __buf: *mut statfs) -> ::core::ffi::c_int;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_get_home_dir() -> *const gchar;
    fn g_get_user_data_dir() -> *const gchar;
    fn g_snprintf(string: *mut gchar, n: gulong, format: *const gchar, ...) -> gint;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    fn g_date_time_unref(datetime: *mut GDateTime);
    fn g_date_time_new_now_local() -> *mut GDateTime;
    fn g_date_time_format(datetime: *mut GDateTime, format: *const gchar) -> *mut gchar;
    fn g_convert(
        str: *const gchar,
        len: gssize,
        to_codeset: *const gchar,
        from_codeset: *const gchar,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_filename_from_utf8(
        utf8string: *const gchar,
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
    fn g_get_filename_charsets(filename_charsets: *mut *mut *const gchar) -> gboolean;
    fn fdopendir(__fd: ::core::ffi::c_int) -> *mut DIR;
    fn g_dir_read_name(dir: *mut GDir) -> *const gchar;
    fn g_dir_close(dir: *mut GDir);
    fn g_file_set_contents_full(
        filename: *const gchar,
        contents: *const gchar,
        length: gssize,
        flags: GFileSetContentsFlags,
        mode: ::core::ffi::c_int,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_mkdir_with_parents(pathname: *const gchar, mode: gint) -> gint;
    fn g_path_is_absolute(file_name: *const gchar) -> gboolean;
    fn g_path_skip_root(file_name: *const gchar) -> *const gchar;
    fn g_path_get_basename(file_name: *const gchar) -> *mut gchar;
    fn g_path_get_dirname(file_name: *const gchar) -> *mut gchar;
    fn g_canonicalize_filename(filename: *const gchar, relative_to: *const gchar) -> *mut gchar;
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
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_remove_all(hash_table: *mut GHashTable);
    fn g_hash_table_lookup_extended(
        hash_table: *mut GHashTable,
        lookup_key: gconstpointer,
        orig_key: *mut gpointer,
        value: *mut gpointer,
    ) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_get_monotonic_time() -> gint64;
    fn g_utf8_validate(str: *const gchar, max_len: gssize, end: *mut *const gchar) -> gboolean;
    static safe_c2rust_g_ascii_table: *const guint16;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_str_has_suffix(str: *const gchar, suffix: *const gchar) -> gboolean;
    fn g_ascii_strcasecmp(s1: *const gchar, s2: *const gchar) -> gint;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
    fn g_string_new(init: *const gchar) -> *mut GString;
    fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
    fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
    fn g_string_prepend(string: *mut GString, val: *const gchar) -> *mut GString;
    fn g_string_prepend_c(string: *mut GString, c: gchar) -> *mut GString;
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
    fn g_uri_escape_string(
        unescaped: *const ::core::ffi::c_char,
        reserved_chars_allowed: *const ::core::ffi::c_char,
        allow_utf8: gboolean,
    ) -> *mut ::core::ffi::c_char;
    fn g_type_class_peek_parent(g_class: gpointer) -> gpointer;
    fn g_type_default_interface_peek(g_type: GType) -> gpointer;
    fn g_type_register_static_simple(
        parent_type: GType,
        type_name: *const gchar,
        class_size: guint,
        class_init: GClassInitFunc,
        instance_size: guint,
        instance_init: GInstanceInitFunc,
        flags: GTypeFlags,
    ) -> GType;
    fn g_type_add_interface_static(
        instance_type: GType,
        interface_type: GType,
        info: *const GInterfaceInfo,
    );
    fn g_type_class_adjust_private_offset(g_class: gpointer, private_size_or_offset: *mut gint);
    fn g_type_check_instance_is_a(instance: *mut GTypeInstance, iface_type: GType) -> gboolean;
    fn g_object_new(object_type: GType, first_property_name: *const gchar, ...) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_file_attribute_info_list_new() -> *mut GFileAttributeInfoList;
    fn g_file_attribute_info_list_ref(
        list: *mut GFileAttributeInfoList,
    ) -> *mut GFileAttributeInfoList;
    fn g_file_attribute_info_list_add(
        list: *mut GFileAttributeInfoList,
        name: *const ::core::ffi::c_char,
        type_0: GFileAttributeType,
        flags: GFileAttributeInfoFlags,
    );
    fn g_file_info_new() -> *mut GFileInfo;
    fn g_file_info_get_attribute_string(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn g_file_info_set_attribute_string(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
        attr_value: *const ::core::ffi::c_char,
    );
    fn g_file_info_set_attribute_boolean(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
        attr_value: gboolean,
    );
    fn g_file_info_set_attribute_uint64(
        info: *mut GFileInfo,
        attribute: *const ::core::ffi::c_char,
        attr_value: guint64,
    );
    fn g_file_attribute_matcher_new(
        attributes: *const ::core::ffi::c_char,
    ) -> *mut GFileAttributeMatcher;
    fn g_file_attribute_matcher_unref(matcher: *mut GFileAttributeMatcher);
    fn g_file_attribute_matcher_matches(
        matcher: *mut GFileAttributeMatcher,
        attribute: *const ::core::ffi::c_char,
    ) -> gboolean;
    fn g_file_get_type() -> GType;
    fn g_file_get_parent(file: *mut GFile) -> *mut GFile;
    fn g_file_get_child(file: *mut GFile, name: *const ::core::ffi::c_char) -> *mut GFile;
    fn g_file_get_child_for_display_name(
        file: *mut GFile,
        display_name: *const ::core::ffi::c_char,
        error: *mut *mut GError,
    ) -> *mut GFile;
    fn glib__private__() -> *const GLibPrivateVTable;
    fn remove(__filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn g_access(filename: *const gchar, mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn g_unlink(filename: *const gchar) -> ::core::ffi::c_int;
    fn g_close(fd: gint, error: *mut *mut GError) -> gboolean;
    fn _g_local_file_info_get_parent_info(
        dir: *const ::core::ffi::c_char,
        attribute_matcher: *mut GFileAttributeMatcher,
        parent_info: *mut GLocalParentFileInfo,
    );
    fn _g_local_file_info_free_parent_info(parent_info: *mut GLocalParentFileInfo);
    fn _g_local_file_info_get(
        basename: *const ::core::ffi::c_char,
        path: *const ::core::ffi::c_char,
        attribute_matcher: *mut GFileAttributeMatcher,
        flags: GFileQueryInfoFlags,
        parent_info: *mut GLocalParentFileInfo,
        error: *mut *mut GError,
    ) -> *mut GFileInfo;
    fn _g_local_file_info_set_attribute(
        filename: *mut ::core::ffi::c_char,
        attribute: *const ::core::ffi::c_char,
        type_0: GFileAttributeType,
        value_p: gpointer,
        flags: GFileQueryInfoFlags,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn _g_local_file_info_set_attributes(
        filename: *mut ::core::ffi::c_char,
        info: *mut GFileInfo,
        flags: GFileQueryInfoFlags,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn _g_local_file_enumerator_new(
        file: *mut GLocalFile,
        attributes: *const ::core::ffi::c_char,
        flags: GFileQueryInfoFlags,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileEnumerator;
    fn _g_local_file_input_stream_new(fd: ::core::ffi::c_int) -> *mut GFileInputStream;
    fn _g_local_file_output_stream_open(
        filename: *const ::core::ffi::c_char,
        readable: gboolean,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileOutputStream;
    fn _g_local_file_output_stream_create(
        filename: *const ::core::ffi::c_char,
        readable: gboolean,
        flags: GFileCreateFlags,
        reference_info: *mut GFileInfo,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileOutputStream;
    fn _g_local_file_output_stream_append(
        filename: *const ::core::ffi::c_char,
        flags: GFileCreateFlags,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileOutputStream;
    fn _g_local_file_output_stream_replace(
        filename: *const ::core::ffi::c_char,
        readable: gboolean,
        etag: *const ::core::ffi::c_char,
        create_backup: gboolean,
        flags: GFileCreateFlags,
        reference_info: *mut GFileInfo,
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> *mut GFileOutputStream;
    fn g_cancellable_set_error_if_cancelled(
        cancellable: *mut GCancellable,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_io_error_quark() -> GQuark;
    fn g_io_error_from_errno(err_no: gint) -> GIOErrorEnum;
    fn _g_local_file_io_stream_new(
        output_stream: *mut GLocalFileOutputStream,
    ) -> *mut GFileIOStream;
    fn g_vfs_get_default() -> *mut GVfs;
    fn g_unix_mount_free(mount_entry: *mut GUnixMountEntry);
    fn g_unix_mount_point_free(mount_point: *mut GUnixMountPoint);
    fn g_unix_mount_get_mount_path(mount_entry: *mut GUnixMountEntry)
        -> *const ::core::ffi::c_char;
    fn g_unix_mount_get_fs_type(mount_entry: *mut GUnixMountEntry) -> *const ::core::ffi::c_char;
    fn g_unix_mount_get_options(mount_entry: *mut GUnixMountEntry) -> *const ::core::ffi::c_char;
    fn g_unix_mount_is_readonly(mount_entry: *mut GUnixMountEntry) -> gboolean;
    fn g_unix_mount_is_system_internal(mount_entry: *mut GUnixMountEntry) -> gboolean;
    fn g_unix_mount_point_get_options(
        mount_point: *mut GUnixMountPoint,
    ) -> *const ::core::ffi::c_char;
    fn g_unix_mount_point_at(
        mount_path: *const ::core::ffi::c_char,
        time_read: *mut guint64,
    ) -> *mut GUnixMountPoint;
    fn g_unix_mount_at(
        mount_path: *const ::core::ffi::c_char,
        time_read: *mut guint64,
    ) -> *mut GUnixMountEntry;
    fn g_unix_mounts_changed_since(time: guint64) -> gboolean;
    fn g_local_file_monitor_new_for_path(
        pathname: *const gchar,
        is_directory: gboolean,
        flags: GFileMonitorFlags,
        error: *mut *mut GError,
    ) -> *mut GFileMonitor;
    fn _g_mount_get_for_mount_path(
        mount_path: *const ::core::ffi::c_char,
        cancellable: *mut GCancellable,
    ) -> *mut GMount;
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn glib_should_use_portal() -> gboolean;
    fn g_trash_portal_trash_file(file: *mut GFile, error: *mut *mut GError) -> gboolean;
}
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __ino64_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __fsid_t {
    pub __val: [::core::ffi::c_int; 2],
}
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __fsblkcnt64_t = ::core::ffi::c_ulong;
pub type __fsfilcnt64_t = ::core::ffi::c_ulong;
pub type __fsword_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ino_t = __ino64_t;
pub type dev_t = __dev_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type ssize_t = isize;
pub type size_t = usize;
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
pub struct statfs {
    pub f_type: __fsword_t,
    pub f_bsize: __fsword_t,
    pub f_blocks: __fsblkcnt64_t,
    pub f_bfree: __fsblkcnt64_t,
    pub f_bavail: __fsblkcnt64_t,
    pub f_files: __fsfilcnt64_t,
    pub f_ffree: __fsfilcnt64_t,
    pub f_fsid: __fsid_t,
    pub f_namelen: __fsword_t,
    pub f_frsize: __fsword_t,
    pub f_flags: __fsword_t,
    pub f_spare: [__fsword_t; 4],
}
pub type guint16 = ::core::ffi::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
pub type GDateTime = _GDateTime;
pub type GData = _GData;
pub type DIR = __dirstream;
pub type GDir = _GDir;
pub type GFileSetContentsFlags = ::core::ffi::c_uint;
pub const G_FILE_SET_CONTENTS_ONLY_EXISTING: GFileSetContentsFlags = 4;
pub const G_FILE_SET_CONTENTS_DURABLE: GFileSetContentsFlags = 2;
pub const G_FILE_SET_CONTENTS_CONSISTENT: GFileSetContentsFlags = 1;
pub const G_FILE_SET_CONTENTS_NONE: GFileSetContentsFlags = 0;
pub type GHashTable = _GHashTable;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
pub type GVariant = _GVariant;
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
pub type GInstanceInitFunc = Option<unsafe extern "C" fn(*mut GTypeInstance, gpointer) -> ()>;
pub type GClassInitFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
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
pub type GFileQueryInfoFlags = ::core::ffi::c_uint;
pub const G_FILE_QUERY_INFO_NOFOLLOW_SYMLINKS: GFileQueryInfoFlags = 1;
pub const G_FILE_QUERY_INFO_NONE: GFileQueryInfoFlags = 0;
pub type GFileCreateFlags = ::core::ffi::c_uint;
pub const G_FILE_CREATE_REPLACE_DESTINATION: GFileCreateFlags = 2;
pub const G_FILE_CREATE_PRIVATE: GFileCreateFlags = 1;
pub const G_FILE_CREATE_NONE: GFileCreateFlags = 0;
pub type GFileMeasureFlags = ::core::ffi::c_uint;
pub const G_FILE_MEASURE_NO_XDEV: GFileMeasureFlags = 8;
pub const G_FILE_MEASURE_APPARENT_SIZE: GFileMeasureFlags = 4;
pub const G_FILE_MEASURE_REPORT_ANY_ERROR: GFileMeasureFlags = 2;
pub const G_FILE_MEASURE_NONE: GFileMeasureFlags = 0;
pub type GMountMountFlags = ::core::ffi::c_uint;
pub const G_MOUNT_MOUNT_NONE: GMountMountFlags = 0;
pub type GMountUnmountFlags = ::core::ffi::c_uint;
pub const G_MOUNT_UNMOUNT_FORCE: GMountUnmountFlags = 1;
pub const G_MOUNT_UNMOUNT_NONE: GMountUnmountFlags = 0;
pub type GDriveStartFlags = ::core::ffi::c_uint;
pub const G_DRIVE_START_NONE: GDriveStartFlags = 0;
pub type GFileCopyFlags = ::core::ffi::c_uint;
pub const G_FILE_COPY_TARGET_DEFAULT_MODIFIED_TIME: GFileCopyFlags = 64;
pub const G_FILE_COPY_TARGET_DEFAULT_PERMS: GFileCopyFlags = 32;
pub const G_FILE_COPY_NO_FALLBACK_FOR_MOVE: GFileCopyFlags = 16;
pub const G_FILE_COPY_ALL_METADATA: GFileCopyFlags = 8;
pub const G_FILE_COPY_NOFOLLOW_SYMLINKS: GFileCopyFlags = 4;
pub const G_FILE_COPY_BACKUP: GFileCopyFlags = 2;
pub const G_FILE_COPY_OVERWRITE: GFileCopyFlags = 1;
pub const G_FILE_COPY_NONE: GFileCopyFlags = 0;
pub type GFileMonitorFlags = ::core::ffi::c_uint;
pub const G_FILE_MONITOR_WATCH_MOVES: GFileMonitorFlags = 8;
pub const G_FILE_MONITOR_WATCH_HARD_LINKS: GFileMonitorFlags = 4;
pub const G_FILE_MONITOR_SEND_MOVED: GFileMonitorFlags = 2;
pub const G_FILE_MONITOR_WATCH_MOUNTS: GFileMonitorFlags = 1;
pub const G_FILE_MONITOR_NONE: GFileMonitorFlags = 0;
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
pub type GInputStream = _GInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GInputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GInputStreamPrivate,
}
pub type GInputStreamPrivate = _GInputStreamPrivate;
pub type GOutputStream = _GOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GOutputStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GOutputStreamPrivate,
}
pub type GOutputStreamPrivate = _GOutputStreamPrivate;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileMonitor {
    pub parent_instance: GObject,
    pub priv_0: *mut GFileMonitorPrivate,
}
pub type GFileMonitorPrivate = _GFileMonitorPrivate;
pub type GFileMonitor = _GFileMonitor;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileInputStream {
    pub parent_instance: GInputStream,
    pub priv_0: *mut GFileInputStreamPrivate,
}
pub type GFileInputStreamPrivate = _GFileInputStreamPrivate;
pub type GFileInputStream = _GFileInputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileOutputStream {
    pub parent_instance: GOutputStream,
    pub priv_0: *mut GFileOutputStreamPrivate,
}
pub type GFileOutputStreamPrivate = _GFileOutputStreamPrivate;
pub type GFileOutputStream = _GFileOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileIOStream {
    pub parent_instance: GIOStream,
    pub priv_0: *mut GFileIOStreamPrivate,
}
pub type GFileIOStreamPrivate = _GFileIOStreamPrivate;
pub type GIOStream = _GIOStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GIOStream {
    pub parent_instance: GObject,
    pub priv_0: *mut GIOStreamPrivate,
}
pub type GIOStreamPrivate = _GIOStreamPrivate;
pub type GFileIOStream = _GFileIOStream;
pub type GIcon = _GIcon;
pub type GMount = _GMount;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMountOperation {
    pub parent_instance: GObject,
    pub priv_0: *mut GMountOperationPrivate,
}
pub type GMountOperationPrivate = _GMountOperationPrivate;
pub type GMountOperation = _GMountOperation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GVfs {
    pub parent_instance: GObject,
}
pub type GVfs = _GVfs;
pub type GAsyncReadyCallback =
    Option<unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer) -> ()>;
pub type GFileProgressCallback = Option<unsafe extern "C" fn(goffset, goffset, gpointer) -> ()>;
pub type GFileMeasureProgressCallback =
    Option<unsafe extern "C" fn(gboolean, guint64, guint64, guint64, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFile {
    pub parent_instance: GObject,
    pub filename: *mut ::core::ffi::c_char,
}
pub type GLocalFile = _GLocalFile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileClass {
    pub parent_class: GObjectClass,
}
pub type GLocalFileClass = _GLocalFileClass;
pub type GFileIface = _GFileIface;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GFileIface {
    pub g_iface: GTypeInterface,
    pub dup: Option<unsafe extern "C" fn(*mut GFile) -> *mut GFile>,
    pub hash: Option<unsafe extern "C" fn(*mut GFile) -> guint>,
    pub equal: Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean>,
    pub is_native: Option<unsafe extern "C" fn(*mut GFile) -> gboolean>,
    pub has_uri_scheme:
        Option<unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> gboolean>,
    pub get_uri_scheme: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_basename: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_path: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_uri: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_parse_name: Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>,
    pub get_parent: Option<unsafe extern "C" fn(*mut GFile) -> *mut GFile>,
    pub prefix_matches: Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean>,
    pub get_relative_path:
        Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> *mut ::core::ffi::c_char>,
    pub resolve_relative_path:
        Option<unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> *mut GFile>,
    pub get_child_for_display_name: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            *mut *mut GError,
        ) -> *mut GFile,
    >,
    pub enumerate_children: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileEnumerator,
    >,
    pub enumerate_children_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileQueryInfoFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub enumerate_children_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileEnumerator,
    >,
    pub query_info: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub query_info_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileQueryInfoFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub query_info_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileInfo,
    >,
    pub query_filesystem_info: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInfo,
    >,
    pub query_filesystem_info_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub query_filesystem_info_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileInfo,
    >,
    pub find_enclosing_mount: Option<
        unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> *mut GMount,
    >,
    pub find_enclosing_mount_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub find_enclosing_mount_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GMount,
    >,
    pub set_display_name: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFile,
    >,
    pub set_display_name_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub set_display_name_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFile>,
    pub query_settable_attributes: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileAttributeInfoList,
    >,
    pub _query_settable_attributes_async: Option<unsafe extern "C" fn() -> ()>,
    pub _query_settable_attributes_finish: Option<unsafe extern "C" fn() -> ()>,
    pub query_writable_namespaces: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileAttributeInfoList,
    >,
    pub _query_writable_namespaces_async: Option<unsafe extern "C" fn() -> ()>,
    pub _query_writable_namespaces_finish: Option<unsafe extern "C" fn() -> ()>,
    pub set_attribute: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            GFileAttributeType,
            gpointer,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub set_attributes_from_info: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFileInfo,
            GFileQueryInfoFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub set_attributes_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFileInfo,
            GFileQueryInfoFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub set_attributes_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GFileInfo,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub read_fn: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileInputStream,
    >,
    pub read_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub read_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileInputStream,
    >,
    pub append_to: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub append_to_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub append_to_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub create: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub create_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub create_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub replace: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            gboolean,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub replace_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            gboolean,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub replace_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut *mut GError,
        ) -> *mut GFileOutputStream,
    >,
    pub delete_file:
        Option<unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean>,
    pub delete_file_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub delete_file_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub trash:
        Option<unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean>,
    pub trash_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub trash_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub make_directory:
        Option<unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean>,
    pub make_directory_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub make_directory_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub make_symbolic_link: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            *mut GCancellable,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub make_symbolic_link_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub make_symbolic_link_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub copy: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFile,
            GFileCopyFlags,
            *mut GCancellable,
            GFileProgressCallback,
            gpointer,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub copy_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFile,
            GFileCopyFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GFileProgressCallback,
            gpointer,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub copy_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub move_0: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFile,
            GFileCopyFlags,
            *mut GCancellable,
            GFileProgressCallback,
            gpointer,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub move_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GFile,
            GFileCopyFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GFileProgressCallback,
            gpointer,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub move_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub mount_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountMountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub mount_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFile>,
    pub unmount_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub unmount_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub eject_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub eject_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub mount_enclosing_volume: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountMountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub mount_enclosing_volume_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub monitor_dir: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileMonitorFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileMonitor,
    >,
    pub monitor_file: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileMonitorFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileMonitor,
    >,
    pub open_readwrite: Option<
        unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> *mut GFileIOStream,
    >,
    pub open_readwrite_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub open_readwrite_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileIOStream,
    >,
    pub create_readwrite: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileIOStream,
    >,
    pub create_readwrite_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub create_readwrite_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileIOStream,
    >,
    pub replace_readwrite: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            gboolean,
            GFileCreateFlags,
            *mut GCancellable,
            *mut *mut GError,
        ) -> *mut GFileIOStream,
    >,
    pub replace_readwrite_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *const ::core::ffi::c_char,
            gboolean,
            GFileCreateFlags,
            ::core::ffi::c_int,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub replace_readwrite_finish: Option<
        unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> *mut GFileIOStream,
    >,
    pub start_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GDriveStartFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub start_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub stop_mountable: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub stop_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub supports_thread_contexts: gboolean,
    pub unmount_mountable_with_operation: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub unmount_mountable_with_operation_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub eject_mountable_with_operation: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GMountUnmountFlags,
            *mut GMountOperation,
            *mut GCancellable,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub eject_mountable_with_operation_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub poll_mountable: Option<
        unsafe extern "C" fn(*mut GFile, *mut GCancellable, GAsyncReadyCallback, gpointer) -> (),
    >,
    pub poll_mountable_finish:
        Option<unsafe extern "C" fn(*mut GFile, *mut GAsyncResult, *mut *mut GError) -> gboolean>,
    pub measure_disk_usage: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileMeasureFlags,
            *mut GCancellable,
            GFileMeasureProgressCallback,
            gpointer,
            *mut guint64,
            *mut guint64,
            *mut guint64,
            *mut *mut GError,
        ) -> gboolean,
    >,
    pub measure_disk_usage_async: Option<
        unsafe extern "C" fn(
            *mut GFile,
            GFileMeasureFlags,
            gint,
            *mut GCancellable,
            GFileMeasureProgressCallback,
            gpointer,
            GAsyncReadyCallback,
            gpointer,
        ) -> (),
    >,
    pub measure_disk_usage_finish: Option<
        unsafe extern "C" fn(
            *mut GFile,
            *mut GAsyncResult,
            *mut guint64,
            *mut guint64,
            *mut guint64,
            *mut *mut GError,
        ) -> gboolean,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MeasureState {
    pub flags: GFileMeasureFlags,
    pub contained_on: dev_t,
    pub cancellable: *mut GCancellable,
    pub progress_callback: GFileMeasureProgressCallback,
    pub progress_data: gpointer,
    pub disk_usage: guint64,
    pub num_dirs: guint64,
    pub num_files: guint64,
    pub last_progress_report: guint64,
}
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
pub type GVfsClass = _GVfsClass;
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
pub type GStatBuf = stat;
pub type GUnixMountEntry = _GUnixMountEntry;
pub type GUnixMountPoint = _GUnixMountPoint;
pub type GLocalFileOutputStream = _GLocalFileOutputStream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GLocalFileOutputStream {
    pub parent_instance: GFileOutputStream,
    pub priv_0: *mut GLocalFileOutputStreamPrivate,
}
pub type GLocalFileOutputStreamPrivate = _GLocalFileOutputStreamPrivate;
pub const MOUNT_INFO_READONLY: C2RustUnnamed_1 = 1;
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
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_ISVTX: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
pub const STATX_BASIC_STATS: ::core::ffi::c_uint = 0x7ff as ::core::ffi::c_uint;
pub const STATX_ALL: ::core::ffi::c_uint = 0xfff as ::core::ffi::c_uint;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_assert_finalize_object\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const EXDEV: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const EISDIR: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const ENOTEMPTY: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const S_ISVTX: ::core::ffi::c_int = __S_ISVTX;
pub const AT_FDCWD: ::core::ffi::c_int = -(100 as ::core::ffi::c_int);
pub const AT_SYMLINK_NOFOLLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const AT_EMPTY_PATH: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_DIR_SEPARATOR: ::core::ffi::c_int = '/' as i32;
pub const W_OK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const G_TIME_SPAN_MILLISECOND: ::core::ffi::c_long = 1000 as ::core::ffi::c_long;
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
pub const G_FILE_ATTRIBUTE_FILESYSTEM_SIZE: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"filesystem::size\0")
};
pub const G_FILE_ATTRIBUTE_FILESYSTEM_FREE: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"filesystem::free\0")
};
pub const G_FILE_ATTRIBUTE_FILESYSTEM_USED: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"filesystem::used\0")
};
pub const G_FILE_ATTRIBUTE_FILESYSTEM_TYPE: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"filesystem::type\0")
};
pub const G_FILE_ATTRIBUTE_FILESYSTEM_READONLY: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"filesystem::readonly\0")
};
pub const G_FILE_ATTRIBUTE_FILESYSTEM_REMOTE: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"filesystem::remote\0")
};
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
unsafe extern "C" fn safe_c2rust_g_local_file_fstatat(
    mut fd: ::core::ffi::c_int,
    mut path: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut mask: GLocalFileStatField,
    mut mask_required: GLocalFileStatField,
    mut stat_buf: *mut statx,
) -> ::core::ffi::c_int {
    return safe_c2rust_g_local_file_statx(fd, path, flags, mask, mask_required, stat_buf);
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_mode(mut buf: *const statx) -> guint16 {
    return (*buf).stx_mode as guint16;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_dev(mut buf: *const statx) -> dev_t {
    return safe_c2rust_gnu_dev_makedev(
        (*buf).stx_dev_major as ::core::ffi::c_uint,
        (*buf).stx_dev_minor as ::core::ffi::c_uint,
    ) as dev_t;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_size(mut buf: *const statx) -> guint64 {
    return (*buf).stx_size as guint64;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_blocks(mut buf: *const statx) -> guint64 {
    return (*buf).stx_blocks as guint64;
}
static mut safe_c2rust_local_writable_attributes: *mut GFileAttributeInfoList =
    ::core::ptr::null::<GFileAttributeInfoList>() as *mut GFileAttributeInfoList;
static mut safe_c2rust_local_writable_namespaces: *mut GFileAttributeInfoList =
    ::core::ptr::null::<GFileAttributeInfoList>() as *mut GFileAttributeInfoList;
static mut safe_c2rust_g_local_file_parent_class: gpointer = NULL_0;
unsafe extern "C" fn safe_c2rust_g_local_file_class_intern_init(mut klass: gpointer) {
    safe_c2rust_g_local_file_parent_class = g_type_class_peek_parent(klass);
    if safe_c2rust_GLocalFile_private_offset != 0 as ::core::ffi::c_int {
        g_type_class_adjust_private_offset(klass, &raw mut safe_c2rust_GLocalFile_private_offset);
    }
    safe_c2rust_g_local_file_class_init(klass as *mut GLocalFileClass);
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_local_file_get_type_once() -> GType {
    let mut g_define_type_id: GType = g_type_register_static_simple(
        ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType,
        g_intern_static_string(b"GLocalFile\0" as *const u8 as *const gchar),
        ::core::mem::size_of::<GLocalFileClass>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GClassInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(gpointer) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_local_file_class_intern_init as unsafe extern "C" fn(gpointer) -> (),
            )),
        ),
        ::core::mem::size_of::<GLocalFile>() as guint,
        ::core::mem::transmute::<Option<unsafe extern "C" fn() -> ()>, GInstanceInitFunc>(
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut GLocalFile) -> ()>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(
                safe_c2rust_g_local_file_init as unsafe extern "C" fn(*mut GLocalFile) -> (),
            )),
        ),
        G_TYPE_FLAG_NONE,
    );
    let g_implement_interface_info: GInterfaceInfo = _GInterfaceInfo {
        interface_init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            GInterfaceInitFunc,
        >(::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GFileIface) -> ()>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(
            safe_c2rust_g_local_file_file_iface_init as unsafe extern "C" fn(*mut GFileIface) -> (),
        ))),
        interface_finalize: None,
        interface_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    g_type_add_interface_static(
        g_define_type_id,
        g_file_get_type(),
        &raw const g_implement_interface_info,
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_get_type() -> GType {
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
        let mut g_define_type_id: GType = safe_c2rust_g_local_file_get_type_once();
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
static mut safe_c2rust_GLocalFile_private_offset: gint = 0;
unsafe extern "C" fn safe_c2rust_g_local_file_finalize(mut object: *mut GObject) {
    let mut local: *mut GLocalFile = ::core::ptr::null_mut::<GLocalFile>();
    local = object as *mut ::core::ffi::c_void as *mut GLocalFile;
    g_free((*local).filename as gpointer);
    (*(safe_c2rust_g_local_file_parent_class as *mut GObjectClass))
        .finalize
        .expect("non-null function pointer")(object);
}
unsafe extern "C" fn safe_c2rust_g_local_file_class_init(mut klass: *mut GLocalFileClass) {
    let mut gobject_class: *mut GObjectClass =
        klass as *mut ::core::ffi::c_void as *mut GObjectClass;
    let mut list: *mut GFileAttributeInfoList = ::core::ptr::null_mut::<GFileAttributeInfoList>();
    (*gobject_class).finalize =
        Some(safe_c2rust_g_local_file_finalize as unsafe extern "C" fn(*mut GObject) -> ())
            as Option<unsafe extern "C" fn(*mut GObject) -> ()>;
    list = g_file_attribute_info_list_new();
    g_file_attribute_info_list_add(
        list,
        G_FILE_ATTRIBUTE_UNIX_MODE.as_ptr(),
        G_FILE_ATTRIBUTE_TYPE_UINT32,
        (G_FILE_ATTRIBUTE_INFO_COPY_WITH_FILE as ::core::ffi::c_int
            | G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED as ::core::ffi::c_int)
            as GFileAttributeInfoFlags,
    );
    g_file_attribute_info_list_add(
        list,
        G_FILE_ATTRIBUTE_UNIX_UID.as_ptr(),
        G_FILE_ATTRIBUTE_TYPE_UINT32,
        G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED,
    );
    g_file_attribute_info_list_add(
        list,
        G_FILE_ATTRIBUTE_UNIX_GID.as_ptr(),
        G_FILE_ATTRIBUTE_TYPE_UINT32,
        G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED,
    );
    g_file_attribute_info_list_add(
        list,
        G_FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET.as_ptr(),
        G_FILE_ATTRIBUTE_TYPE_BYTE_STRING,
        G_FILE_ATTRIBUTE_INFO_NONE,
    );
    g_file_attribute_info_list_add(
        list,
        G_FILE_ATTRIBUTE_TIME_MODIFIED.as_ptr(),
        G_FILE_ATTRIBUTE_TYPE_UINT64,
        (G_FILE_ATTRIBUTE_INFO_COPY_WITH_FILE as ::core::ffi::c_int
            | G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED as ::core::ffi::c_int)
            as GFileAttributeInfoFlags,
    );
    g_file_attribute_info_list_add(
        list,
        G_FILE_ATTRIBUTE_TIME_MODIFIED_USEC.as_ptr(),
        G_FILE_ATTRIBUTE_TYPE_UINT32,
        (G_FILE_ATTRIBUTE_INFO_COPY_WITH_FILE as ::core::ffi::c_int
            | G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED as ::core::ffi::c_int)
            as GFileAttributeInfoFlags,
    );
    g_file_attribute_info_list_add(
        list,
        G_FILE_ATTRIBUTE_TIME_ACCESS.as_ptr(),
        G_FILE_ATTRIBUTE_TYPE_UINT64,
        G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED,
    );
    g_file_attribute_info_list_add(
        list,
        G_FILE_ATTRIBUTE_TIME_ACCESS_USEC.as_ptr(),
        G_FILE_ATTRIBUTE_TYPE_UINT32,
        G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED,
    );
    g_file_attribute_info_list_add(
        list,
        G_FILE_ATTRIBUTE_TIME_MODIFIED_NSEC.as_ptr(),
        G_FILE_ATTRIBUTE_TYPE_UINT32,
        (G_FILE_ATTRIBUTE_INFO_COPY_WITH_FILE as ::core::ffi::c_int
            | G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED as ::core::ffi::c_int)
            as GFileAttributeInfoFlags,
    );
    g_file_attribute_info_list_add(
        list,
        G_FILE_ATTRIBUTE_TIME_ACCESS_NSEC.as_ptr(),
        G_FILE_ATTRIBUTE_TYPE_UINT32,
        G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED,
    );
    safe_c2rust_local_writable_attributes = list;
}
unsafe extern "C" fn safe_c2rust_g_local_file_init(mut local: *mut GLocalFile) {}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_get_filename(
    mut file: *mut GLocalFile,
) -> *const ::core::ffi::c_char {
    return (*file).filename;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_new(
    mut filename: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut local: *mut GLocalFile = ::core::ptr::null_mut::<GLocalFile>();
    local = g_object_new(
        safe_c2rust__g_local_file_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GLocalFile;
    (*local).filename =
        g_canonicalize_filename(filename as *const gchar, ::core::ptr::null::<gchar>())
            as *mut ::core::ffi::c_char;
    return local as *mut ::core::ffi::c_void as *mut GFile;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_local_file_new_from_dirname_and_basename(
    mut dirname: *const gchar,
    mut basename: *const gchar,
) -> *mut GFile {
    let mut local: *mut GLocalFile = ::core::ptr::null_mut::<GLocalFile>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !dirname.is_null() {
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
            b"dirname != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !basename.is_null()
            && *basename.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && strchr(basename as *const ::core::ffi::c_char, '/' as i32).is_null()
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
            b"basename && basename[0] && !strchr (basename, '/')\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    local = g_object_new(
        safe_c2rust__g_local_file_get_type(),
        ::core::ptr::null::<gchar>(),
    ) as *mut GLocalFile;
    (*local).filename = g_build_filename(dirname, basename, NULL_0) as *mut ::core::ffi::c_char;
    return local as *mut ::core::ffi::c_void as *mut GFile;
}
unsafe extern "C" fn safe_c2rust_g_local_file_is_native(mut file: *mut GFile) -> gboolean {
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_local_file_has_uri_scheme(
    mut file: *mut GFile,
    mut uri_scheme: *const ::core::ffi::c_char,
) -> gboolean {
    return (g_ascii_strcasecmp(
        uri_scheme as *const gchar,
        b"file\0" as *const u8 as *const gchar,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_local_file_get_uri_scheme(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    return safe_c2rust_g_strdup_inline(b"file\0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn safe_c2rust_g_local_file_get_basename(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    return g_path_get_basename((*(file as *mut ::core::ffi::c_void as *mut GLocalFile)).filename)
        as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_g_local_file_get_path(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    return safe_c2rust_g_strdup_inline(
        (*(file as *mut ::core::ffi::c_void as *mut GLocalFile)).filename,
    );
}
unsafe extern "C" fn safe_c2rust_g_local_file_get_uri(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    return g_filename_to_uri(
        (*(file as *mut ::core::ffi::c_void as *mut GLocalFile)).filename,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_get_filename_charset(
    mut filename_charset: *mut *const gchar,
) -> gboolean {
    let mut charsets: *mut *const gchar = ::core::ptr::null_mut::<*const gchar>();
    let mut is_utf8: gboolean = 0;
    is_utf8 = g_get_filename_charsets(&raw mut charsets);
    if !filename_charset.is_null() {
        *filename_charset = *charsets.offset(0 as ::core::ffi::c_int as isize);
    }
    return is_utf8;
}
unsafe extern "C" fn safe_c2rust_name_is_valid_for_display(
    mut string: *const ::core::ffi::c_char,
    mut is_valid_utf8: gboolean,
) -> gboolean {
    let mut c: ::core::ffi::c_char = 0;
    if is_valid_utf8 == 0
        && g_utf8_validate(
            string as *const gchar,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<*const gchar>(),
        ) == 0
    {
        return FALSE;
    }
    loop {
        let fresh1 = string;
        string = string.offset(1);
        c = *fresh1;
        if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
        if *safe_c2rust_g_ascii_table.offset(c as guchar as isize) as ::core::ffi::c_int
            & G_ASCII_CNTRL as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            return FALSE;
        }
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_local_file_get_parse_name(
    mut file: *mut GFile,
) -> *mut ::core::ffi::c_char {
    let mut filename: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut parse_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut charset: *const gchar = ::core::ptr::null::<gchar>();
    let mut utf8_filename: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut roundtripped_filename: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut free_utf8_filename: gboolean = 0;
    let mut is_valid_utf8: gboolean = 0;
    let mut escaped_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    filename = (*(file as *mut ::core::ffi::c_void as *mut GLocalFile)).filename;
    if safe_c2rust_get_filename_charset(&raw mut charset) != 0 {
        utf8_filename = filename as *mut ::core::ffi::c_char;
        free_utf8_filename = FALSE as gboolean;
        is_valid_utf8 = FALSE as gboolean;
    } else {
        utf8_filename = g_convert(
            filename as *const gchar,
            -(1 as ::core::ffi::c_int) as gssize,
            b"UTF-8\0" as *const u8 as *const gchar,
            charset,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        ) as *mut ::core::ffi::c_char;
        free_utf8_filename = TRUE as gboolean;
        is_valid_utf8 = TRUE as gboolean;
        if !utf8_filename.is_null() {
            roundtripped_filename = g_convert(
                utf8_filename,
                -(1 as ::core::ffi::c_int) as gssize,
                charset,
                b"UTF-8\0" as *const u8 as *const gchar,
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<gsize>(),
                ::core::ptr::null_mut::<*mut GError>(),
            ) as *mut ::core::ffi::c_char;
            if roundtripped_filename.is_null()
                || strcmp(filename, roundtripped_filename) != 0 as ::core::ffi::c_int
            {
                g_free(utf8_filename as gpointer);
                utf8_filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            g_free(roundtripped_filename as gpointer);
        }
    }
    if !utf8_filename.is_null()
        && safe_c2rust_name_is_valid_for_display(utf8_filename, is_valid_utf8) != 0
    {
        if free_utf8_filename != 0 {
            parse_name = utf8_filename;
        } else {
            parse_name = safe_c2rust_g_strdup_inline(utf8_filename);
        }
    } else {
        escaped_path = g_uri_escape_string(
            filename,
            b"!$&'()*+,;=:@/\0" as *const u8 as *const ::core::ffi::c_char,
            TRUE,
        );
        parse_name = g_strconcat(
            b"file://\0" as *const u8 as *const gchar,
            if *escaped_path as ::core::ffi::c_int != '/' as i32 {
                b"/\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            escaped_path,
            NULL_0,
        ) as *mut ::core::ffi::c_char;
        g_free(escaped_path as gpointer);
        if free_utf8_filename != 0 {
            g_free(utf8_filename as gpointer);
        }
    }
    return parse_name;
}
unsafe extern "C" fn safe_c2rust_g_local_file_get_parent(mut file: *mut GFile) -> *mut GFile {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut non_root: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut dirname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parent: *mut GFile = ::core::ptr::null_mut::<GFile>();
    non_root = g_path_skip_root((*local).filename) as *const ::core::ffi::c_char;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !non_root.is_null() {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfile.c\0" as *const u8
                as *const ::core::ffi::c_char,
            441 as ::core::ffi::c_int,
            G_STRFUNC,
            b"non_root != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *non_root as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<GFile>();
    }
    dirname = g_path_get_dirname((*local).filename) as *mut ::core::ffi::c_char;
    parent = safe_c2rust__g_local_file_new(dirname);
    g_free(dirname as gpointer);
    return parent;
}
unsafe extern "C" fn safe_c2rust_g_local_file_dup(mut file: *mut GFile) -> *mut GFile {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    return safe_c2rust__g_local_file_new((*local).filename);
}
unsafe extern "C" fn safe_c2rust_g_local_file_hash(mut file: *mut GFile) -> guint {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    return g_str_hash((*local).filename as gconstpointer);
}
unsafe extern "C" fn safe_c2rust_g_local_file_equal(
    mut file1: *mut GFile,
    mut file2: *mut GFile,
) -> gboolean {
    let mut local1: *mut GLocalFile = file1 as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut local2: *mut GLocalFile = file2 as *mut ::core::ffi::c_void as *mut GLocalFile;
    return (strcmp(
        (*local1).filename as *const ::core::ffi::c_char,
        (*local2).filename as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_match_prefix(
    mut path: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut prefix_len: ::core::ffi::c_int = 0;
    prefix_len = strlen(prefix) as ::core::ffi::c_int;
    if strncmp(path, prefix, prefix_len as size_t) != 0 as ::core::ffi::c_int {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if prefix_len > 0 as ::core::ffi::c_int
        && *prefix.offset((prefix_len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == G_DIR_SEPARATOR
    {
        prefix_len -= 1;
    }
    return path.offset(prefix_len as isize);
}
unsafe extern "C" fn safe_c2rust_g_local_file_prefix_matches(
    mut parent: *mut GFile,
    mut descendant: *mut GFile,
) -> gboolean {
    let mut parent_local: *mut GLocalFile = parent as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut descendant_local: *mut GLocalFile =
        descendant as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut remainder: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    remainder = safe_c2rust_match_prefix((*descendant_local).filename, (*parent_local).filename);
    if !remainder.is_null() && *remainder as ::core::ffi::c_int == G_DIR_SEPARATOR {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_local_file_get_relative_path(
    mut parent: *mut GFile,
    mut descendant: *mut GFile,
) -> *mut ::core::ffi::c_char {
    let mut parent_local: *mut GLocalFile = parent as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut descendant_local: *mut GLocalFile =
        descendant as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut remainder: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    remainder = safe_c2rust_match_prefix((*descendant_local).filename, (*parent_local).filename);
    if !remainder.is_null() && *remainder as ::core::ffi::c_int == G_DIR_SEPARATOR {
        return safe_c2rust_g_strdup_inline(remainder.offset(1 as ::core::ffi::c_int as isize));
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn safe_c2rust_g_local_file_resolve_relative_path(
    mut file: *mut GFile,
    mut relative_path: *const ::core::ffi::c_char,
) -> *mut GFile {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut child: *mut GFile = ::core::ptr::null_mut::<GFile>();
    if g_path_is_absolute(relative_path as *const gchar) != 0 {
        return safe_c2rust__g_local_file_new(relative_path);
    }
    filename =
        g_build_filename((*local).filename, relative_path, NULL_0) as *mut ::core::ffi::c_char;
    child = safe_c2rust__g_local_file_new(filename);
    g_free(filename as gpointer);
    return child;
}
unsafe extern "C" fn safe_c2rust_g_local_file_enumerate_children(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileEnumerator {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    return _g_local_file_enumerator_new(local, attributes, flags, cancellable, error);
}
unsafe extern "C" fn safe_c2rust_g_local_file_get_child_for_display_name(
    mut file: *mut GFile,
    mut display_name: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut GFile {
    let mut new_file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut basename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    basename = g_filename_from_utf8(
        display_name as *const gchar,
        -(1 as ::core::ffi::c_int) as gssize,
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<gsize>(),
        ::core::ptr::null_mut::<*mut GError>(),
    ) as *mut ::core::ffi::c_char;
    if basename.is_null() {
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_INVALID_FILENAME as ::core::ffi::c_int as gint,
            glib_gettext(b"Invalid filename %s\0" as *const u8 as *const gchar),
            display_name,
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    new_file = g_file_get_child(file, basename);
    g_free(basename as gpointer);
    return new_file;
}
unsafe extern "C" fn safe_c2rust_get_fs_type(
    mut f_type: ::core::ffi::c_long,
) -> *const ::core::ffi::c_char {
    match f_type {
        44533 => return b"adfs\0" as *const u8 as *const ::core::ffi::c_char,
        1397113167 => return b"afs\0" as *const u8 as *const ::core::ffi::c_char,
        391 => return b"autofs\0" as *const u8 as *const ::core::ffi::c_char,
        44543 => return b"affs\0" as *const u8 as *const ::core::ffi::c_char,
        1650746742 => return b"bdevfs\0" as *const u8 as *const ::core::ffi::c_char,
        1111905073 => return b"befs\0" as *const u8 as *const ::core::ffi::c_char,
        464386766 => return b"bfs\0" as *const u8 as *const ::core::ffi::c_char,
        1112100429 => return b"binfmt_misc\0" as *const u8 as *const ::core::ffi::c_char,
        2435016766 => return b"btrfs\0" as *const u8 as *const ::core::ffi::c_char,
        1936880249 => {
            return b"btrfs_test_fs\0" as *const u8 as *const ::core::ffi::c_char;
        }
        2613483 => return b"cgroup\0" as *const u8 as *const ::core::ffi::c_char,
        1667723888 => return b"cgroup2\0" as *const u8 as *const ::core::ffi::c_char,
        4283649346 => return b"cifs\0" as *const u8 as *const ::core::ffi::c_char,
        1937076805 => return b"coda\0" as *const u8 as *const ::core::ffi::c_char,
        19920823 => return b"coh\0" as *const u8 as *const ::core::ffi::c_char,
        1650812272 => return b"configfs\0" as *const u8 as *const ::core::ffi::c_char,
        684539205 => return b"cramfs\0" as *const u8 as *const ::core::ffi::c_char,
        1684170528 => return b"debugfs\0" as *const u8 as *const ::core::ffi::c_char,
        4979 => return b"devfs\0" as *const u8 as *const ::core::ffi::c_char,
        7377 => return b"devpts\0" as *const u8 as *const ::core::ffi::c_char,
        61791 => return b"ecryptfs\0" as *const u8 as *const ::core::ffi::c_char,
        3730735588 => return b"efivarfs\0" as *const u8 as *const ::core::ffi::c_char,
        4278867 => return b"efs\0" as *const u8 as *const ::core::ffi::c_char,
        538032816 => return b"exfat\0" as *const u8 as *const ::core::ffi::c_char,
        4989 => return b"ext\0" as *const u8 as *const ::core::ffi::c_char,
        61265 => return b"ext2\0" as *const u8 as *const ::core::ffi::c_char,
        61267 => return b"ext3/ext4\0" as *const u8 as *const ::core::ffi::c_char,
        4076150800 => return b"f2fs\0" as *const u8 as *const ::core::ffi::c_char,
        1702057286 => return b"fuse\0" as *const u8 as *const ::core::ffi::c_char,
        1702057283 => return b"fusectl\0" as *const u8 as *const ::core::ffi::c_char,
        195894762 => return b"futexfs\0" as *const u8 as *const ::core::ffi::c_char,
        16964 => return b"hfs\0" as *const u8 as *const ::core::ffi::c_char,
        12648430 => return b"hostfs\0" as *const u8 as *const ::core::ffi::c_char,
        4187351113 => return b"hpfs\0" as *const u8 as *const ::core::ffi::c_char,
        2508478710 => return b"hugetlbfs\0" as *const u8 as *const ::core::ffi::c_char,
        38496 => return b"isofs\0" as *const u8 as *const ::core::ffi::c_char,
        29366 => return b"jffs2\0" as *const u8 as *const ::core::ffi::c_char,
        827541066 => return b"jfs\0" as *const u8 as *const ::core::ffi::c_char,
        4991 => return b"minix\0" as *const u8 as *const ::core::ffi::c_char,
        5007 => return b"minix2\0" as *const u8 as *const ::core::ffi::c_char,
        9320 => return b"minix2\0" as *const u8 as *const ::core::ffi::c_char,
        9336 => return b"minix22\0" as *const u8 as *const ::core::ffi::c_char,
        19802 => return b"minix3\0" as *const u8 as *const ::core::ffi::c_char,
        427819522 => return b"mqueue\0" as *const u8 as *const ::core::ffi::c_char,
        19780 => return b"msdos\0" as *const u8 as *const ::core::ffi::c_char,
        22092 => return b"ncp\0" as *const u8 as *const ::core::ffi::c_char,
        26985 => return b"nfs\0" as *const u8 as *const ::core::ffi::c_char,
        13364 => return b"nilfs\0" as *const u8 as *const ::core::ffi::c_char,
        1853056627 => return b"nsfs\0" as *const u8 as *const ::core::ffi::c_char,
        1397118030 => return b"ntfs\0" as *const u8 as *const ::core::ffi::c_char,
        1952539503 => return b"ocfs2\0" as *const u8 as *const ::core::ffi::c_char,
        40865 => return b"openprom\0" as *const u8 as *const ::core::ffi::c_char,
        2035054128 => return b"overlay\0" as *const u8 as *const ::core::ffi::c_char,
        1346981957 => return b"pipefs\0" as *const u8 as *const ::core::ffi::c_char,
        40864 => return b"proc\0" as *const u8 as *const ::core::ffi::c_char,
        1634035564 => return b"pstore\0" as *const u8 as *const ::core::ffi::c_char,
        47 => return b"qnx4\0" as *const u8 as *const ::core::ffi::c_char,
        1746473250 => return b"qnx6\0" as *const u8 as *const ::core::ffi::c_char,
        2240043254 => return b"ramfs\0" as *const u8 as *const ::core::ffi::c_char,
        1382369651 => return b"reiserfs\0" as *const u8 as *const ::core::ffi::c_char,
        29301 => return b"romfs\0" as *const u8 as *const ::core::ffi::c_char,
        1733912937 => return b"rpc_pipefs\0" as *const u8 as *const ::core::ffi::c_char,
        1935894131 => return b"securityfs\0" as *const u8 as *const ::core::ffi::c_char,
        4185718668 => return b"selinuxfs\0" as *const u8 as *const ::core::ffi::c_char,
        1128357203 => return b"smackfs\0" as *const u8 as *const ::core::ffi::c_char,
        20859 => return b"smb\0" as *const u8 as *const ::core::ffi::c_char,
        4266872130 => return b"smb2\0" as *const u8 as *const ::core::ffi::c_char,
        1397703499 => return b"sockfs\0" as *const u8 as *const ::core::ffi::c_char,
        1936814952 => return b"squashfs\0" as *const u8 as *const ::core::ffi::c_char,
        1650812274 => return b"sysfs\0" as *const u8 as *const ::core::ffi::c_char,
        19920822 => return b"sysv2\0" as *const u8 as *const ::core::ffi::c_char,
        19920821 => return b"sysv4\0" as *const u8 as *const ::core::ffi::c_char,
        16914836 => return b"tmpfs\0" as *const u8 as *const ::core::ffi::c_char,
        1953653091 => return b"tracefs\0" as *const u8 as *const ::core::ffi::c_char,
        352400198 => return b"udf\0" as *const u8 as *const ::core::ffi::c_char,
        72020 => return b"ufs\0" as *const u8 as *const ::core::ffi::c_char,
        40866 => return b"usbdevice\0" as *const u8 as *const ::core::ffi::c_char,
        16914839 => return b"v9fs\0" as *const u8 as *const ::core::ffi::c_char,
        2768370933 => return b"vxfs\0" as *const u8 as *const ::core::ffi::c_char,
        2881100148 => return b"xenfs\0" as *const u8 as *const ::core::ffi::c_char,
        19920820 => return b"xenix\0" as *const u8 as *const ::core::ffi::c_char,
        1481003842 => return b"xfs\0" as *const u8 as *const ::core::ffi::c_char,
        19911021 => return b"xiafs\0" as *const u8 as *const ::core::ffi::c_char,
        1379160930 => return b"reiser4\0" as *const u8 as *const ::core::ffi::c_char,
        _ => return ::core::ptr::null::<::core::ffi::c_char>(),
    };
}
static mut safe_c2rust_g__mount_info_hash_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut safe_c2rust_mount_info_hash: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut safe_c2rust_mount_info_hash_cache_time: guint64 = 0 as guint64;
unsafe extern "C" fn safe_c2rust_device_equal(
    mut v1: gconstpointer,
    mut v2: gconstpointer,
) -> gboolean {
    return (*(v1 as *mut dev_t) == *(v2 as *mut dev_t)) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_device_hash(mut v: gconstpointer) -> guint {
    return *(v as *mut dev_t) as guint;
}
unsafe extern "C" fn safe_c2rust_get_mount_info(
    mut fs_info: *mut GFileInfo,
    mut path: *const ::core::ffi::c_char,
    mut matcher: *mut GFileAttributeMatcher,
) {
    let mut buf: GStatBuf = stat {
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
    let mut got_info: gboolean = 0;
    let mut info_as_ptr: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut mount_info: guint = 0;
    let mut mountpoint: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dev: *mut dev_t = ::core::ptr::null_mut::<dev_t>();
    let mut mount: *mut GUnixMountEntry = ::core::ptr::null_mut::<GUnixMountEntry>();
    let mut cache_time: guint64 = 0;
    let mut is_remote: gboolean = FALSE;
    if lstat(path, &raw mut buf) != 0 as ::core::ffi::c_int {
        return;
    }
    g_mutex_lock(&raw mut safe_c2rust_g__mount_info_hash_lock);
    if safe_c2rust_mount_info_hash.is_null() {
        safe_c2rust_mount_info_hash = g_hash_table_new_full(
            Some(safe_c2rust_device_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(
                safe_c2rust_device_equal
                    as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
            ),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            None,
        );
    }
    if g_unix_mounts_changed_since(safe_c2rust_mount_info_hash_cache_time) != 0 {
        g_hash_table_remove_all(safe_c2rust_mount_info_hash);
    }
    got_info = g_hash_table_lookup_extended(
        safe_c2rust_mount_info_hash,
        &raw mut buf.st_dev as gconstpointer,
        ::core::ptr::null_mut::<gpointer>(),
        &raw mut info_as_ptr,
    );
    g_mutex_unlock(&raw mut safe_c2rust_g__mount_info_hash_lock);
    mount_info = info_as_ptr as gulong as guint;
    if got_info == 0 {
        mount_info = 0 as guint;
        mountpoint = safe_c2rust_find_mountpoint_for(path, buf.st_dev as dev_t, FALSE);
        if mountpoint.is_null() {
            mountpoint =
                safe_c2rust_g_strdup_inline(b"/\0" as *const u8 as *const ::core::ffi::c_char);
        }
        mount = g_unix_mount_at(mountpoint, &raw mut cache_time);
        if !mount.is_null() {
            if g_unix_mount_is_readonly(mount) != 0 {
                mount_info |= MOUNT_INFO_READONLY as ::core::ffi::c_int as guint;
            }
            if safe_c2rust_is_remote_fs_type(g_unix_mount_get_fs_type(mount) as *const gchar) != 0 {
                is_remote = TRUE as gboolean;
            }
            g_unix_mount_free(mount);
        }
        g_free(mountpoint as gpointer);
        dev = ({
            let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
            let mut __s: gsize = ::core::mem::size_of::<dev_t>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __s == 1 as gsize {
                __p = g_malloc0(__n);
            } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                __p = g_malloc0(__n.wrapping_mul(__s));
            } else {
                __p = g_malloc0_n(__n, __s);
            }
            __p
        }) as *mut dev_t;
        *dev = buf.st_dev as dev_t;
        g_mutex_lock(&raw mut safe_c2rust_g__mount_info_hash_lock);
        safe_c2rust_mount_info_hash_cache_time = cache_time;
        g_hash_table_insert(
            safe_c2rust_mount_info_hash,
            dev as gpointer,
            mount_info as gulong as gpointer,
        );
        g_mutex_unlock(&raw mut safe_c2rust_g__mount_info_hash_lock);
    }
    if g_file_attribute_matcher_matches(matcher, G_FILE_ATTRIBUTE_FILESYSTEM_READONLY.as_ptr()) != 0
    {
        g_file_info_set_attribute_boolean(
            fs_info,
            G_FILE_ATTRIBUTE_FILESYSTEM_READONLY.as_ptr(),
            (mount_info & MOUNT_INFO_READONLY as ::core::ffi::c_int as guint) as gboolean,
        );
    }
    if g_file_attribute_matcher_matches(matcher, G_FILE_ATTRIBUTE_FILESYSTEM_REMOTE.as_ptr()) != 0 {
        g_file_info_set_attribute_boolean(
            fs_info,
            G_FILE_ATTRIBUTE_FILESYSTEM_REMOTE.as_ptr(),
            is_remote,
        );
    }
}
unsafe extern "C" fn safe_c2rust_g_set_io_error(
    mut error: *mut *mut GError,
    mut msg: *const gchar,
    mut file: *mut GFile,
    mut errsv: gint,
) {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut display_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    display_name = g_filename_display_name((*local).filename);
    g_set_error(
        error,
        g_io_error_quark(),
        g_io_error_from_errno(errsv) as gint,
        msg,
        display_name,
        g_strerror(errsv),
    );
    g_free(display_name as gpointer);
}
unsafe extern "C" fn safe_c2rust_g_local_file_query_filesystem_info(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut statfs_result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut no_size: gboolean = 0;
    let mut fstype: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut block_size: guint64 = 0;
    let mut statfs_buffer: statfs = statfs {
        f_type: 0,
        f_bsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_fsid: __fsid_t { __val: [0; 2] },
        f_namelen: 0,
        f_frsize: 0,
        f_flags: 0,
        f_spare: [0; 4],
    };
    let mut attribute_matcher: *mut GFileAttributeMatcher =
        ::core::ptr::null_mut::<GFileAttributeMatcher>();
    no_size = FALSE as gboolean;
    statfs_result = statfs((*local).filename, &raw mut statfs_buffer);
    block_size = statfs_buffer.f_bsize as guint64;
    if statfs_result == 0 as ::core::ffi::c_int
        && statfs_buffer.f_bavail == 0 as __fsblkcnt64_t
        && statfs_buffer.f_bfree == 0 as __fsblkcnt64_t
        && (statfs_buffer.f_type == 0x564c as __fsword_t
            || statfs_buffer.f_type == 0x65735546 as __fsword_t)
    {
        no_size = TRUE as gboolean;
    }
    if statfs_result == -(1 as ::core::ffi::c_int) {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        safe_c2rust_g_set_io_error(
            error,
            glib_gettext(
                b"Error getting filesystem info for %s: %s\0" as *const u8 as *const gchar,
            ),
            file,
            errsv as gint,
        );
        return ::core::ptr::null_mut::<GFileInfo>();
    }
    info = g_file_info_new();
    attribute_matcher = g_file_attribute_matcher_new(attributes);
    if no_size == 0
        && g_file_attribute_matcher_matches(
            attribute_matcher,
            G_FILE_ATTRIBUTE_FILESYSTEM_FREE.as_ptr(),
        ) != 0
    {
        g_file_info_set_attribute_uint64(
            info,
            G_FILE_ATTRIBUTE_FILESYSTEM_FREE.as_ptr(),
            block_size.wrapping_mul(statfs_buffer.f_bavail as guint64),
        );
    }
    if no_size == 0
        && g_file_attribute_matcher_matches(
            attribute_matcher,
            G_FILE_ATTRIBUTE_FILESYSTEM_SIZE.as_ptr(),
        ) != 0
    {
        g_file_info_set_attribute_uint64(
            info,
            G_FILE_ATTRIBUTE_FILESYSTEM_SIZE.as_ptr(),
            block_size.wrapping_mul(statfs_buffer.f_blocks as guint64),
        );
    }
    if no_size == 0
        && g_file_attribute_matcher_matches(
            attribute_matcher,
            G_FILE_ATTRIBUTE_FILESYSTEM_USED.as_ptr(),
        ) != 0
    {
        g_file_info_set_attribute_uint64(
            info,
            G_FILE_ATTRIBUTE_FILESYSTEM_USED.as_ptr(),
            block_size.wrapping_mul(
                (statfs_buffer.f_blocks as guint64).wrapping_sub(statfs_buffer.f_bfree as guint64),
            ),
        );
    }
    fstype = safe_c2rust_get_fs_type(statfs_buffer.f_type as ::core::ffi::c_long);
    if !fstype.is_null()
        && g_file_attribute_matcher_matches(
            attribute_matcher,
            G_FILE_ATTRIBUTE_FILESYSTEM_TYPE.as_ptr(),
        ) != 0
    {
        g_file_info_set_attribute_string(info, G_FILE_ATTRIBUTE_FILESYSTEM_TYPE.as_ptr(), fstype);
    }
    if g_file_attribute_matcher_matches(
        attribute_matcher,
        G_FILE_ATTRIBUTE_FILESYSTEM_READONLY.as_ptr(),
    ) != 0
        || g_file_attribute_matcher_matches(
            attribute_matcher,
            G_FILE_ATTRIBUTE_FILESYSTEM_REMOTE.as_ptr(),
        ) != 0
    {
        safe_c2rust_get_mount_info(info, (*local).filename, attribute_matcher);
    }
    g_file_attribute_matcher_unref(attribute_matcher);
    return info;
}
unsafe extern "C" fn safe_c2rust_g_local_file_find_enclosing_mount(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GMount {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut buf: GStatBuf = stat {
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
    let mut mountpoint: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut mount: *mut GMount = ::core::ptr::null_mut::<GMount>();
    if !(lstat((*local).filename, &raw mut buf) != 0 as ::core::ffi::c_int) {
        mountpoint = safe_c2rust_find_mountpoint_for((*local).filename, buf.st_dev as dev_t, FALSE);
        if !mountpoint.is_null() {
            mount = _g_mount_get_for_mount_path(mountpoint, cancellable);
            g_free(mountpoint as gpointer);
            if !mount.is_null() {
                return mount;
            }
        }
    }
    safe_c2rust_g_set_io_error(
        error,
        glib_gettext(b"Containing mount for file %s not found\0" as *const u8 as *const gchar),
        file,
        0 as gint,
    );
    return ::core::ptr::null_mut::<GMount>();
}
unsafe extern "C" fn safe_c2rust_g_local_file_set_display_name(
    mut file: *mut GFile,
    mut display_name: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFile {
    let mut local: *mut GLocalFile = ::core::ptr::null_mut::<GLocalFile>();
    let mut new_local: *mut GLocalFile = ::core::ptr::null_mut::<GLocalFile>();
    let mut new_file: *mut GFile = ::core::ptr::null_mut::<GFile>();
    let mut parent: *mut GFile = ::core::ptr::null_mut::<GFile>();
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
    let mut class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
    let mut vfs: *mut GVfs = ::core::ptr::null_mut::<GVfs>();
    let mut errsv: ::core::ffi::c_int = 0;
    parent = g_file_get_parent(file);
    if parent.is_null() {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(b"Can\xE2\x80\x99t rename root directory\0" as *const u8 as *const gchar),
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    new_file = g_file_get_child_for_display_name(parent, display_name, error);
    g_object_unref(parent as gpointer);
    if new_file.is_null() {
        return ::core::ptr::null_mut::<GFile>();
    }
    local = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    new_local = new_file as *mut ::core::ffi::c_void as *mut GLocalFile;
    if lstat((*new_local).filename, &raw mut statbuf) == -(1 as ::core::ffi::c_int) {
        errsv = *__errno_location();
        if errsv != ENOENT {
            safe_c2rust_g_set_io_error(
                error,
                glib_gettext(b"Error renaming file %s: %s\0" as *const u8 as *const gchar),
                new_file,
                errsv as gint,
            );
            return ::core::ptr::null_mut::<GFile>();
        }
    } else {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_EXISTS as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Can\xE2\x80\x99t rename file, filename already exists\0" as *const u8
                    as *const gchar,
            ),
        );
        return ::core::ptr::null_mut::<GFile>();
    }
    if rename((*local).filename, (*new_local).filename) == -(1 as ::core::ffi::c_int) {
        errsv = *__errno_location();
        if errsv == EINVAL {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_FILENAME as ::core::ffi::c_int as gint,
                glib_gettext(b"Invalid filename\0" as *const u8 as *const gchar),
            );
        } else {
            safe_c2rust_g_set_io_error(
                error,
                glib_gettext(b"Error renaming file %s: %s\0" as *const u8 as *const gchar),
                file,
                errsv as gint,
            );
        }
        g_object_unref(new_file as gpointer);
        return ::core::ptr::null_mut::<GFile>();
    }
    vfs = g_vfs_get_default();
    class = (*(vfs as *mut GTypeInstance)).g_class as *mut GVfsClass;
    if (*class).local_file_moved.is_some() {
        (*class)
            .local_file_moved
            .expect("non-null function pointer")(
            vfs, (*local).filename, (*new_local).filename
        );
    }
    return new_file;
}
unsafe extern "C" fn safe_c2rust_g_local_file_query_info(
    mut file: *mut GFile,
    mut attributes: *const ::core::ffi::c_char,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInfo {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
    let mut matcher: *mut GFileAttributeMatcher = ::core::ptr::null_mut::<GFileAttributeMatcher>();
    let mut basename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dirname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parent_info: GLocalParentFileInfo = GLocalParentFileInfo {
        writable: 0,
        is_sticky: 0,
        has_trash_dir: 0,
        owner: 0,
        device: 0,
        inode: 0,
        extra_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        free_extra_data: None,
    };
    matcher = g_file_attribute_matcher_new(attributes);
    basename = g_path_get_basename((*local).filename) as *mut ::core::ffi::c_char;
    dirname = g_path_get_dirname((*local).filename) as *mut ::core::ffi::c_char;
    _g_local_file_info_get_parent_info(dirname, matcher, &raw mut parent_info);
    g_free(dirname as gpointer);
    info = _g_local_file_info_get(
        basename,
        (*local).filename,
        matcher,
        flags,
        &raw mut parent_info,
        error,
    );
    _g_local_file_info_free_parent_info(&raw mut parent_info);
    g_free(basename as gpointer);
    g_file_attribute_matcher_unref(matcher);
    return info;
}
unsafe extern "C" fn safe_c2rust_g_local_file_query_settable_attributes(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileAttributeInfoList {
    return g_file_attribute_info_list_ref(safe_c2rust_local_writable_attributes);
}
unsafe extern "C" fn safe_c2rust_g_local_file_query_writable_namespaces(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileAttributeInfoList {
    let mut list: *mut GFileAttributeInfoList = ::core::ptr::null_mut::<GFileAttributeInfoList>();
    let mut class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
    let mut vfs: *mut GVfs = ::core::ptr::null_mut::<GVfs>();
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_local_writable_namespaces;
        } else {
        };
        (({
            let mut gapg_temp_newval: *mut GFileAttributeInfoList =
                ::core::ptr::null_mut::<GFileAttributeInfoList>();
            let mut gapg_temp_atomic: *mut *mut GFileAttributeInfoList =
                &raw mut safe_c2rust_local_writable_namespaces;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        })
        .is_null()
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_local_writable_namespaces as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        list = g_file_attribute_info_list_new();
        g_file_attribute_info_list_add(
            list,
            b"xattr\0" as *const u8 as *const ::core::ffi::c_char,
            G_FILE_ATTRIBUTE_TYPE_STRING,
            (G_FILE_ATTRIBUTE_INFO_COPY_WITH_FILE as ::core::ffi::c_int
                | G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED as ::core::ffi::c_int)
                as GFileAttributeInfoFlags,
        );
        g_file_attribute_info_list_add(
            list,
            b"xattr-sys\0" as *const u8 as *const ::core::ffi::c_char,
            G_FILE_ATTRIBUTE_TYPE_STRING,
            G_FILE_ATTRIBUTE_INFO_COPY_WHEN_MOVED,
        );
        vfs = g_vfs_get_default();
        class = (*(vfs as *mut GTypeInstance)).g_class as *mut GVfsClass;
        if (*class).add_writable_namespaces.is_some() {
            (*class)
                .add_writable_namespaces
                .expect("non-null function pointer")(vfs, list);
        }
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_local_writable_namespaces = list;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_local_writable_namespaces as *mut ::core::ffi::c_void,
            list as guintptr as gpointer,
        );
    }
    list = safe_c2rust_local_writable_namespaces;
    return g_file_attribute_info_list_ref(list);
}
unsafe extern "C" fn safe_c2rust_g_local_file_set_attribute(
    mut file: *mut GFile,
    mut attribute: *const ::core::ffi::c_char,
    mut type_0: GFileAttributeType,
    mut value_p: gpointer,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    return _g_local_file_info_set_attribute(
        (*local).filename,
        attribute,
        type_0,
        value_p,
        flags,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_local_file_set_attributes_from_info(
    mut file: *mut GFile,
    mut info: *mut GFileInfo,
    mut flags: GFileQueryInfoFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut res: ::core::ffi::c_int = 0;
    let mut chained_res: ::core::ffi::c_int = 0;
    let mut default_iface: *mut GFileIface = ::core::ptr::null_mut::<GFileIface>();
    res = _g_local_file_info_set_attributes((*local).filename, info, flags, cancellable, error)
        as ::core::ffi::c_int;
    if res == 0 {
        error = ::core::ptr::null_mut::<*mut GError>();
    }
    default_iface = g_type_default_interface_peek(g_file_get_type()) as *mut GFileIface;
    chained_res = (*default_iface)
        .set_attributes_from_info
        .expect("non-null function pointer")(file, info, flags, cancellable, error)
        as ::core::ffi::c_int;
    return (res != 0 && chained_res != 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_local_file_read(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileInputStream {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut fd: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut buf: statx = statx {
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
    fd = open(
        (*local).filename,
        O_RDONLY | O_BINARY | O_CLOEXEC,
        0 as ::core::ffi::c_int,
    );
    if fd == -(1 as ::core::ffi::c_int) {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        safe_c2rust_g_set_io_error(
            error,
            glib_gettext(b"Error opening file %s: %s\0" as *const u8 as *const gchar),
            file,
            errsv as gint,
        );
        return ::core::ptr::null_mut::<GFileInputStream>();
    }
    ret = safe_c2rust_g_local_file_fstat(
        fd,
        G_LOCAL_FILE_STAT_FIELD_TYPE,
        4095 as GLocalFileStatField,
        &raw mut buf,
    );
    if ret == 0 as ::core::ffi::c_int
        && safe_c2rust__g_stat_mode(&raw mut buf) as ::core::ffi::c_int & __S_IFMT
            == 0o40000 as ::core::ffi::c_int
    {
        g_close(fd as gint, ::core::ptr::null_mut::<*mut GError>());
        safe_c2rust_g_set_io_error(
            error,
            glib_gettext(b"Error opening file %s: %s\0" as *const u8 as *const gchar),
            file,
            EISDIR,
        );
        return ::core::ptr::null_mut::<GFileInputStream>();
    }
    return _g_local_file_input_stream_new(fd);
}
unsafe extern "C" fn safe_c2rust_g_local_file_append_to(
    mut file: *mut GFile,
    mut flags: GFileCreateFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    return _g_local_file_output_stream_append(
        (*(file as *mut ::core::ffi::c_void as *mut GLocalFile)).filename,
        flags,
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_local_file_create(
    mut file: *mut GFile,
    mut flags: GFileCreateFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    return _g_local_file_output_stream_create(
        (*(file as *mut ::core::ffi::c_void as *mut GLocalFile)).filename,
        FALSE,
        flags,
        ::core::ptr::null_mut::<GFileInfo>(),
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_local_file_replace(
    mut file: *mut GFile,
    mut etag: *const ::core::ffi::c_char,
    mut make_backup: gboolean,
    mut flags: GFileCreateFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileOutputStream {
    return _g_local_file_output_stream_replace(
        (*(file as *mut ::core::ffi::c_void as *mut GLocalFile)).filename,
        FALSE,
        etag,
        make_backup,
        flags,
        ::core::ptr::null_mut::<GFileInfo>(),
        cancellable,
        error,
    );
}
unsafe extern "C" fn safe_c2rust_g_local_file_open_readwrite(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileIOStream {
    let mut output: *mut GFileOutputStream = ::core::ptr::null_mut::<GFileOutputStream>();
    let mut res: *mut GFileIOStream = ::core::ptr::null_mut::<GFileIOStream>();
    output = _g_local_file_output_stream_open(
        (*(file as *mut ::core::ffi::c_void as *mut GLocalFile)).filename,
        TRUE,
        cancellable,
        error,
    );
    if output.is_null() {
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    res = _g_local_file_io_stream_new(
        output as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream,
    );
    g_object_unref(output as gpointer);
    return res;
}
unsafe extern "C" fn safe_c2rust_g_local_file_create_readwrite(
    mut file: *mut GFile,
    mut flags: GFileCreateFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileIOStream {
    let mut output: *mut GFileOutputStream = ::core::ptr::null_mut::<GFileOutputStream>();
    let mut res: *mut GFileIOStream = ::core::ptr::null_mut::<GFileIOStream>();
    output = _g_local_file_output_stream_create(
        (*(file as *mut ::core::ffi::c_void as *mut GLocalFile)).filename,
        TRUE,
        flags,
        ::core::ptr::null_mut::<GFileInfo>(),
        cancellable,
        error,
    );
    if output.is_null() {
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    res = _g_local_file_io_stream_new(
        output as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream,
    );
    g_object_unref(output as gpointer);
    return res;
}
unsafe extern "C" fn safe_c2rust_g_local_file_replace_readwrite(
    mut file: *mut GFile,
    mut etag: *const ::core::ffi::c_char,
    mut make_backup: gboolean,
    mut flags: GFileCreateFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileIOStream {
    let mut output: *mut GFileOutputStream = ::core::ptr::null_mut::<GFileOutputStream>();
    let mut res: *mut GFileIOStream = ::core::ptr::null_mut::<GFileIOStream>();
    output = _g_local_file_output_stream_replace(
        (*(file as *mut ::core::ffi::c_void as *mut GLocalFile)).filename,
        TRUE,
        etag,
        make_backup,
        flags,
        ::core::ptr::null_mut::<GFileInfo>(),
        cancellable,
        error,
    );
    if output.is_null() {
        return ::core::ptr::null_mut::<GFileIOStream>();
    }
    res = _g_local_file_io_stream_new(
        output as *mut ::core::ffi::c_void as *mut GLocalFileOutputStream,
    );
    g_object_unref(output as gpointer);
    return res;
}
unsafe extern "C" fn safe_c2rust_g_local_file_delete(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
    let mut vfs: *mut GVfs = ::core::ptr::null_mut::<GVfs>();
    if remove((*local).filename) == -(1 as ::core::ffi::c_int) {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        if errsv == EEXIST {
            errsv = ENOTEMPTY;
        }
        safe_c2rust_g_set_io_error(
            error,
            glib_gettext(b"Error removing file %s: %s\0" as *const u8 as *const gchar),
            file,
            errsv as gint,
        );
        return FALSE;
    }
    vfs = g_vfs_get_default();
    class = (*(vfs as *mut GTypeInstance)).g_class as *mut GVfsClass;
    if (*class).local_file_removed.is_some() {
        (*class)
            .local_file_removed
            .expect("non-null function pointer")(vfs, (*local).filename);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_strip_trailing_slashes(
    mut path: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut path_copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    path_copy = safe_c2rust_g_strdup_inline(path);
    len = strlen(path_copy) as ::core::ffi::c_int;
    while len > 1 as ::core::ffi::c_int
        && *path_copy.offset((len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == '/' as i32
    {
        len -= 1;
        *path_copy.offset(len as isize) = 0 as ::core::ffi::c_char;
    }
    return path_copy;
}
unsafe extern "C" fn safe_c2rust_expand_symlink(
    mut link: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut resolved: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut canonical: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parent: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut link2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut symlink_value: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut res: gssize = 0;
    res = readlink(
        link,
        &raw mut symlink_value as *mut ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t).wrapping_sub(1 as size_t),
    ) as gssize;
    if res == -(1 as ::core::ffi::c_int) as gssize {
        return safe_c2rust_g_strdup_inline(link);
    }
    symlink_value[res as usize] = 0 as ::core::ffi::c_char;
    if g_path_is_absolute(&raw mut symlink_value as *mut ::core::ffi::c_char) != 0 {
        return g_canonicalize_filename(
            &raw mut symlink_value as *mut ::core::ffi::c_char,
            ::core::ptr::null::<gchar>(),
        ) as *mut ::core::ffi::c_char;
    } else {
        link2 = safe_c2rust_strip_trailing_slashes(link);
        parent = g_path_get_dirname(link2) as *mut ::core::ffi::c_char;
        g_free(link2 as gpointer);
        resolved = g_build_filename(
            parent,
            &raw mut symlink_value as *mut ::core::ffi::c_char,
            NULL_0,
        ) as *mut ::core::ffi::c_char;
        g_free(parent as gpointer);
        canonical = g_canonicalize_filename(resolved, ::core::ptr::null::<gchar>())
            as *mut ::core::ffi::c_char;
        g_free(resolved as gpointer);
        return canonical;
    };
}
unsafe extern "C" fn safe_c2rust_expand_symlinks(
    mut path: *const ::core::ffi::c_char,
    mut dev: *mut dev_t,
) -> *mut ::core::ffi::c_char {
    let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut target: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut target_stat: GStatBuf = stat {
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
    let mut num_recursions: ::core::ffi::c_int = 0;
    target = safe_c2rust_g_strdup_inline(path);
    num_recursions = 0 as ::core::ffi::c_int;
    loop {
        if lstat(target, &raw mut target_stat) != 0 as ::core::ffi::c_int {
            g_free(target as gpointer);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if target_stat.st_mode & __S_IFMT as __mode_t == 0o120000 as __mode_t {
            tmp = target;
            target = safe_c2rust_expand_symlink(target);
            g_free(tmp as gpointer);
        }
        num_recursions += 1;
        if num_recursions > 40 as ::core::ffi::c_int {
            g_free(target as gpointer);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !(target_stat.st_mode & __S_IFMT as __mode_t == 0o120000 as __mode_t) {
            break;
        }
    }
    if !dev.is_null() {
        *dev = target_stat.st_dev as dev_t;
    }
    return target;
}
unsafe extern "C" fn safe_c2rust_get_parent(
    mut path: *const ::core::ffi::c_char,
    mut parent_dev: *mut dev_t,
) -> *mut ::core::ffi::c_char {
    let mut parent: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut path_copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    path_copy = safe_c2rust_strip_trailing_slashes(path);
    parent = g_path_get_dirname(path_copy) as *mut ::core::ffi::c_char;
    if strcmp(parent, b".\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        g_free(parent as gpointer);
        g_free(path_copy as gpointer);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    g_free(path_copy as gpointer);
    res = safe_c2rust_expand_symlinks(parent, parent_dev);
    g_free(parent as gpointer);
    return res;
}
unsafe extern "C" fn safe_c2rust_expand_all_symlinks(
    mut path: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut parent: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parent_expanded: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut basename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parent_dev: dev_t = 0;
    parent = safe_c2rust_get_parent(path, &raw mut parent_dev);
    if parent.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if g_strcmp0(parent, b"/\0" as *const u8 as *const ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        parent_expanded = safe_c2rust_expand_all_symlinks(parent);
        basename = g_path_get_basename(path as *const gchar) as *mut ::core::ffi::c_char;
        res = g_build_filename(parent_expanded, basename, NULL_0) as *mut ::core::ffi::c_char;
        g_free(basename as gpointer);
        g_free(parent_expanded as gpointer);
    } else {
        res = safe_c2rust_g_strdup_inline(path);
    }
    g_free(parent as gpointer);
    return res;
}
unsafe extern "C" fn safe_c2rust_find_mountpoint_for(
    mut file: *const ::core::ffi::c_char,
    mut dev: dev_t,
    mut resolve_basename_symlink: gboolean,
) -> *mut ::core::ffi::c_char {
    let mut dir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut parent: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dir_dev: dev_t = 0;
    let mut parent_dev: dev_t = 0;
    if resolve_basename_symlink != 0 {
        dir = safe_c2rust_expand_symlinks(file, ::core::ptr::null_mut::<dev_t>());
        if dir.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    } else {
        dir = safe_c2rust_g_strdup_inline(file);
    }
    dir_dev = dev;
    while g_strcmp0(dir, b"/\0" as *const u8 as *const ::core::ffi::c_char)
        != 0 as ::core::ffi::c_int
    {
        parent = safe_c2rust_get_parent(dir, &raw mut parent_dev);
        if parent.is_null() {
            g_free(dir as gpointer);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if parent_dev != dir_dev {
            g_free(parent as gpointer);
            return dir;
        }
        g_free(dir as gpointer);
        dir = parent;
    }
    return dir;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_find_topdir_for(
    mut file: *const ::core::ffi::c_char,
) -> *mut gchar {
    let mut dir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut mountpoint: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dir_dev: dev_t = 0;
    dir = safe_c2rust_get_parent(file, &raw mut dir_dev);
    if dir.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    mountpoint = safe_c2rust_find_mountpoint_for(dir, dir_dev, TRUE);
    g_free(dir as gpointer);
    return mountpoint as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_get_unique_filename(
    mut basename: *const ::core::ffi::c_char,
    mut id: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut dot: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if id == 1 as ::core::ffi::c_int {
        return safe_c2rust_g_strdup_inline(basename);
    }
    dot = strchr(basename, '.' as i32);
    if !dot.is_null() {
        return g_strdup_printf(
            b"%.*s.%d%s\0" as *const u8 as *const gchar,
            dot.offset_from(basename) as ::core::ffi::c_long as ::core::ffi::c_int,
            basename,
            id,
            dot,
        ) as *mut ::core::ffi::c_char;
    } else {
        return g_strdup_printf(b"%s.%d\0" as *const u8 as *const gchar, basename, id)
            as *mut ::core::ffi::c_char;
    };
}
unsafe extern "C" fn safe_c2rust_path_has_prefix(
    mut path: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) -> gboolean {
    let mut prefix_len: ::core::ffi::c_int = 0;
    if prefix.is_null() {
        return TRUE;
    }
    prefix_len = strlen(prefix) as ::core::ffi::c_int;
    if strncmp(path, prefix, prefix_len as size_t) == 0 as ::core::ffi::c_int
        && (prefix_len == 0 as ::core::ffi::c_int
            || *prefix.offset((prefix_len - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                == '/' as i32
            || *path.offset(prefix_len as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || *path.offset(prefix_len as isize) as ::core::ffi::c_int == '/' as i32)
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_try_make_relative(
    mut path: *const ::core::ffi::c_char,
    mut base: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut path2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut base2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut relative: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    path2 = safe_c2rust_expand_all_symlinks(path);
    base2 = safe_c2rust_expand_all_symlinks(base);
    relative = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !path2.is_null() && !base2.is_null() && safe_c2rust_path_has_prefix(path2, base2) != 0 {
        relative = path2.offset(strlen(base2) as isize);
        while *relative as ::core::ffi::c_int == '/' as i32 {
            relative = relative.offset(1);
        }
        relative = safe_c2rust_g_strdup_inline(relative);
    }
    g_free(path2 as gpointer);
    g_free(base2 as gpointer);
    if !relative.is_null() {
        return relative;
    }
    return safe_c2rust_g_strdup_inline(path);
}
unsafe extern "C" fn safe_c2rust_ignore_trash_mount(mut mount: *mut GUnixMountEntry) -> gboolean {
    let mut mount_point: *mut GUnixMountPoint = ::core::ptr::null_mut::<GUnixMountPoint>();
    let mut mount_options: *const gchar = ::core::ptr::null::<gchar>();
    let mut retval: gboolean = TRUE;
    if g_unix_mount_is_system_internal(mount) != 0 {
        return TRUE;
    }
    mount_options = g_unix_mount_get_options(mount) as *const gchar;
    if mount_options.is_null() {
        mount_point = g_unix_mount_point_at(
            g_unix_mount_get_mount_path(mount),
            ::core::ptr::null_mut::<guint64>(),
        );
        if !mount_point.is_null() {
            mount_options = g_unix_mount_point_get_options(mount_point) as *const gchar;
        }
    }
    if mount_options.is_null()
        || strstr(
            mount_options as *const ::core::ffi::c_char,
            b"x-gvfs-notrash\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        retval = FALSE as gboolean;
    }
    let mut _pp: *mut *mut GUnixMountPoint = &raw mut mount_point;
    let mut _ptr: *mut GUnixMountPoint = *_pp;
    *_pp = ::core::ptr::null_mut::<GUnixMountPoint>();
    if !_ptr.is_null() {
        g_unix_mount_point_free(_ptr as *mut GUnixMountPoint);
    }
    return retval;
}
unsafe extern "C" fn safe_c2rust_ignore_trash_path(mut topdir: *const gchar) -> gboolean {
    let mut mount: *mut GUnixMountEntry = ::core::ptr::null_mut::<GUnixMountEntry>();
    let mut retval: gboolean = TRUE;
    mount = g_unix_mount_at(
        topdir as *const ::core::ffi::c_char,
        ::core::ptr::null_mut::<guint64>(),
    );
    if !mount.is_null() {
        retval = safe_c2rust_ignore_trash_mount(mount);
    }
    let mut _pp: *mut *mut GUnixMountEntry = &raw mut mount;
    let mut _ptr: *mut GUnixMountEntry = *_pp;
    *_pp = ::core::ptr::null_mut::<GUnixMountEntry>();
    if !_ptr.is_null() {
        g_unix_mount_free(_ptr as *mut GUnixMountEntry);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_has_trash_dir(
    mut dirname: *const ::core::ffi::c_char,
    mut dir_dev: dev_t,
) -> gboolean {
    static mut safe_c2rust_home_dev_set: gsize = 0 as gsize;
    static mut safe_c2rust_home_dev: dev_t = 0;
    static mut safe_c2rust_home_dev_valid: gboolean = FALSE;
    let mut topdir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut globaldir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut trashdir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tmpname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut uid: uid_t = 0;
    let mut uid_str: [::core::ffi::c_char; 32] = [0; 32];
    let mut global_stat: GStatBuf = stat {
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
    let mut trash_stat: GStatBuf = stat {
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
    let mut res: gboolean = 0;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_home_dev_set;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_home_dev_set;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&raw mut safe_c2rust_home_dev_set as *mut ::core::ffi::c_void)
                != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut home_stat: GStatBuf = stat {
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
        if stat(
            g_get_home_dir() as *const ::core::ffi::c_char,
            &raw mut home_stat,
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust_home_dev = home_stat.st_dev as dev_t;
            safe_c2rust_home_dev_valid = TRUE as gboolean;
        } else {
            safe_c2rust_home_dev_valid = FALSE as gboolean;
        }
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_home_dev_set = 1 as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_home_dev_set as *mut ::core::ffi::c_void,
            1 as ::core::ffi::c_int as gsize,
        );
    }
    if safe_c2rust_home_dev_valid == 0 {
        return FALSE;
    } else if dir_dev == safe_c2rust_home_dev {
        return TRUE;
    }
    topdir = safe_c2rust_find_mountpoint_for(dirname, dir_dev, TRUE);
    if topdir.is_null() {
        return FALSE;
    }
    if safe_c2rust_ignore_trash_path(topdir) != 0 {
        g_free(topdir as gpointer);
        return FALSE;
    }
    globaldir = g_build_filename(
        topdir,
        b".Trash\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    ) as *mut ::core::ffi::c_char;
    if lstat(globaldir, &raw mut global_stat) == 0 as ::core::ffi::c_int
        && global_stat.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
        && global_stat.st_mode & S_ISVTX as __mode_t != 0 as __mode_t
    {
        g_free(globaldir as gpointer);
        g_free(topdir as gpointer);
        return TRUE;
    }
    g_free(globaldir as gpointer);
    uid = geteuid() as uid_t;
    g_snprintf(
        &raw mut uid_str as *mut gchar,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as gulong,
        b"%lu\0" as *const u8 as *const gchar,
        uid as ::core::ffi::c_ulong,
    );
    tmpname = g_strdup_printf(
        b".Trash-%s\0" as *const u8 as *const gchar,
        &raw mut uid_str as *mut ::core::ffi::c_char,
    ) as *mut ::core::ffi::c_char;
    trashdir = g_build_filename(topdir, tmpname, NULL_0) as *mut ::core::ffi::c_char;
    g_free(tmpname as gpointer);
    if lstat(trashdir, &raw mut trash_stat) == 0 as ::core::ffi::c_int {
        g_free(topdir as gpointer);
        g_free(trashdir as gpointer);
        return (trash_stat.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
            && trash_stat.st_uid == uid) as ::core::ffi::c_int;
    }
    g_free(trashdir as gpointer);
    res = (g_access(topdir, W_OK) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as gboolean;
    g_free(topdir as gpointer);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_local_file_is_lost_found_dir(
    mut path: *const ::core::ffi::c_char,
    mut path_dev: dev_t,
) -> gboolean {
    let mut ret: gboolean = FALSE;
    let mut mount_dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut mount_dir_len: size_t = 0;
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
    if !(if 0 != 0 {
        ({
            let __str: *const ::core::ffi::c_char = path;
            let __suffix: *const ::core::ffi::c_char =
                b"/lost+found\0" as *const u8 as *const ::core::ffi::c_char;
            let mut __result: gboolean = FALSE;
            if ({
                let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
                if __str.is_null() || __suffix.is_null() {
                    _g_boolean_var_13 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_13 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_13
            }) as ::core::ffi::c_long
                != 0
            {
                __result = g_str_has_suffix(__str as *const gchar, __suffix as *const gchar);
            } else {
                let __str_len: size_t =
                    strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize)) as size_t;
                let __suffix_len: size_t =
                    strlen(__suffix.offset(__suffix.is_null() as ::core::ffi::c_int as isize))
                        as size_t;
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
                        as ::core::ffi::c_int as gboolean;
                }
            }
            __result
        })
    } else {
        g_str_has_suffix(
            path as *const gchar,
            b"/lost+found\0" as *const u8 as *const gchar,
        )
    } == 0)
    {
        mount_dir = safe_c2rust_find_mountpoint_for(path, path_dev, FALSE) as *mut gchar;
        if !mount_dir.is_null() {
            mount_dir_len = strlen(mount_dir);
            if mount_dir_len == 1 as size_t {
                mount_dir_len = mount_dir_len.wrapping_sub(1);
            }
            if !(mount_dir_len.wrapping_add(strlen(
                b"/lost+found\0" as *const u8 as *const ::core::ffi::c_char,
            )) != strlen(path))
            {
                if !(lstat(path, &raw mut statbuf) != 0 as ::core::ffi::c_int) {
                    if statbuf.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
                        && statbuf.st_uid == 0 as __uid_t
                        && statbuf.st_gid == 0 as __gid_t
                    {
                        ret = TRUE as gboolean;
                    }
                }
            }
        }
    }
    g_free(mount_dir as gpointer);
    return ret;
}
unsafe extern "C" fn safe_c2rust_g_local_file_trash(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut file_stat: GStatBuf = stat {
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
    let mut home_stat: GStatBuf = stat {
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
    let mut homedir: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut trashdir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut topdir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut infodir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut filesdir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut basename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut trashname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut trashfile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut infoname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut infofile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut original_name: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut original_name_escaped: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut is_homedir_trash: gboolean = 0;
    let mut delete_time: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fd: ::core::ffi::c_int = 0;
    let mut trash_stat: GStatBuf = stat {
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
    let mut global_stat: GStatBuf = stat {
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
    let mut dirname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut globaldir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
    let mut vfs: *mut GVfs = ::core::ptr::null_mut::<GVfs>();
    let mut errsv: ::core::ffi::c_int = 0;
    if glib_should_use_portal() != 0 {
        return g_trash_portal_trash_file(file, error);
    }
    if lstat((*local).filename, &raw mut file_stat) != 0 as ::core::ffi::c_int {
        errsv = *__errno_location();
        safe_c2rust_g_set_io_error(
            error,
            glib_gettext(b"Error trashing file %s: %s\0" as *const u8 as *const gchar),
            file,
            errsv as gint,
        );
        return FALSE;
    }
    homedir = g_get_home_dir() as *const ::core::ffi::c_char;
    if stat(homedir, &raw mut home_stat) != 0 as ::core::ffi::c_int {
        errsv = *__errno_location();
        safe_c2rust_g_set_io_error(
            error,
            glib_gettext(b"Error trashing file %s: %s\0" as *const u8 as *const gchar),
            file,
            errsv as gint,
        );
        return FALSE;
    }
    is_homedir_trash = FALSE as gboolean;
    trashdir = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(file_stat.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t) {
        path = g_path_get_dirname((*local).filename) as *mut ::core::ffi::c_char;
        if stat(path, &raw mut file_stat) != 0 {
            errsv = *__errno_location();
            g_free(path as gpointer);
            safe_c2rust_g_set_io_error(
                error,
                glib_gettext(b"Error trashing file %s: %s\0" as *const u8 as *const gchar),
                file,
                errsv as gint,
            );
            return FALSE;
        }
        g_free(path as gpointer);
    }
    if file_stat.st_dev == home_stat.st_dev {
        is_homedir_trash = TRUE as gboolean;
        *__errno_location() = 0 as ::core::ffi::c_int;
        trashdir = g_build_filename(
            g_get_user_data_dir(),
            b"Trash\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        ) as *mut ::core::ffi::c_char;
        if g_mkdir_with_parents(trashdir, 0o700 as gint) < 0 as ::core::ffi::c_int {
            let mut display_name: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            errsv = *__errno_location();
            display_name = g_filename_display_name(trashdir) as *mut ::core::ffi::c_char;
            g_set_error(
                error,
                g_io_error_quark(),
                g_io_error_from_errno(errsv as gint) as gint,
                glib_gettext(
                    b"Unable to create trash directory %s: %s\0" as *const u8 as *const gchar,
                ),
                display_name,
                g_strerror(errsv as gint),
            );
            g_free(display_name as gpointer);
            g_free(trashdir as gpointer);
            return FALSE;
        }
        topdir = safe_c2rust_g_strdup_inline(g_get_user_data_dir() as *const ::core::ffi::c_char);
    } else {
        let mut uid: uid_t = 0;
        let mut uid_str: [::core::ffi::c_char; 32] = [0; 32];
        let mut success: gboolean = FALSE;
        uid = geteuid() as uid_t;
        g_snprintf(
            &raw mut uid_str as *mut gchar,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as gulong,
            b"%lu\0" as *const u8 as *const gchar,
            uid as ::core::ffi::c_ulong,
        );
        topdir = safe_c2rust__g_local_file_find_topdir_for((*local).filename)
            as *mut ::core::ffi::c_char;
        if topdir.is_null() {
            safe_c2rust_g_set_io_error(
                error,
                glib_gettext(
                    b"Unable to find toplevel directory to trash %s\0" as *const u8 as *const gchar,
                ),
                file,
                ENOTSUP,
            );
            return FALSE;
        }
        if safe_c2rust_ignore_trash_path(topdir) != 0 {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Trashing on system internal mounts is not supported\0" as *const u8
                        as *const gchar,
                ),
            );
            g_free(topdir as gpointer);
            return FALSE;
        }
        globaldir = g_build_filename(
            topdir,
            b".Trash\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        ) as *mut ::core::ffi::c_char;
        if lstat(globaldir, &raw mut global_stat) == 0 as ::core::ffi::c_int
            && global_stat.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
            && global_stat.st_mode & S_ISVTX as __mode_t != 0 as __mode_t
        {
            trashdir = g_build_filename(
                globaldir,
                &raw mut uid_str as *mut ::core::ffi::c_char,
                NULL_0,
            ) as *mut ::core::ffi::c_char;
            success = TRUE as gboolean;
            if lstat(trashdir, &raw mut trash_stat) == 0 as ::core::ffi::c_int {
                if !(trash_stat.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
                    || trash_stat.st_uid != uid
                {
                    g_free(trashdir as gpointer);
                    trashdir = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    success = FALSE as gboolean;
                }
            } else if mkdir(trashdir, 0o700 as __mode_t) == -(1 as ::core::ffi::c_int) {
                g_free(trashdir as gpointer);
                trashdir = ::core::ptr::null_mut::<::core::ffi::c_char>();
                success = FALSE as gboolean;
            }
        }
        g_free(globaldir as gpointer);
        if trashdir.is_null() {
            let mut tried_create: gboolean = 0;
            dirname = g_strdup_printf(
                b".Trash-%s\0" as *const u8 as *const gchar,
                &raw mut uid_str as *mut ::core::ffi::c_char,
            ) as *mut ::core::ffi::c_char;
            trashdir = g_build_filename(topdir, dirname, NULL_0) as *mut ::core::ffi::c_char;
            success = TRUE as gboolean;
            g_free(dirname as gpointer);
            tried_create = FALSE as gboolean;
            loop {
                if lstat(trashdir, &raw mut trash_stat) == 0 as ::core::ffi::c_int {
                    if !(trash_stat.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
                        || trash_stat.st_uid != uid
                    {
                        if tried_create != 0 {
                            remove(trashdir);
                        }
                        success = FALSE as gboolean;
                    }
                    break;
                } else if tried_create == 0
                    && mkdir(trashdir, 0o700 as __mode_t) != -(1 as ::core::ffi::c_int)
                {
                    tried_create = TRUE as gboolean;
                } else {
                    success = FALSE as gboolean;
                    break;
                }
            }
        }
        if success == 0 {
            let mut trashdir_display_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
            let mut file_display_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
            trashdir_display_name = g_filename_display_name(trashdir);
            file_display_name = g_filename_display_name((*local).filename);
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Unable to find or create trash directory %s to trash %s\0" as *const u8
                        as *const gchar,
                ),
                trashdir_display_name,
                file_display_name,
            );
            g_free(trashdir_display_name as gpointer);
            g_free(file_display_name as gpointer);
            g_free(topdir as gpointer);
            g_free(trashdir as gpointer);
            return FALSE;
        }
    }
    infodir = g_build_filename(
        trashdir,
        b"info\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    ) as *mut ::core::ffi::c_char;
    filesdir = g_build_filename(
        trashdir,
        b"files\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    ) as *mut ::core::ffi::c_char;
    if mkdir(infodir, 0o700 as __mode_t) == -(1 as ::core::ffi::c_int)
        && *__errno_location() != EEXIST
        || mkdir(filesdir, 0o700 as __mode_t) == -(1 as ::core::ffi::c_int)
            && *__errno_location() != EEXIST
    {
        let mut trashdir_display_name_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut file_display_name_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
        trashdir_display_name_0 = g_filename_display_name(trashdir);
        file_display_name_0 = g_filename_display_name((*local).filename);
        g_set_error(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Unable to find or create trash directory %s to trash %s\0" as *const u8
                    as *const gchar,
            ),
            trashdir_display_name_0,
            file_display_name_0,
        );
        g_free(trashdir_display_name_0 as gpointer);
        g_free(file_display_name_0 as gpointer);
        g_free(topdir as gpointer);
        g_free(trashdir as gpointer);
        g_free(infodir as gpointer);
        g_free(filesdir as gpointer);
        return FALSE;
    }
    g_free(trashdir as gpointer);
    basename = g_path_get_basename((*local).filename) as *mut ::core::ffi::c_char;
    i = 1 as ::core::ffi::c_int;
    trashname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    infofile = ::core::ptr::null_mut::<::core::ffi::c_char>();
    loop {
        g_free(trashname as gpointer);
        g_free(infofile as gpointer);
        let fresh0 = i;
        i = i + 1;
        trashname = safe_c2rust_get_unique_filename(basename, fresh0);
        infoname = g_strconcat(
            trashname,
            b".trashinfo\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        ) as *mut ::core::ffi::c_char;
        infofile = g_build_filename(infodir, infoname, NULL_0) as *mut ::core::ffi::c_char;
        g_free(infoname as gpointer);
        fd = open(
            infofile,
            O_CREAT | O_EXCL | O_CLOEXEC,
            0o666 as ::core::ffi::c_int,
        );
        errsv = *__errno_location();
        if !(fd == -(1 as ::core::ffi::c_int) && errsv == EEXIST) {
            break;
        }
    }
    g_free(basename as gpointer);
    g_free(infodir as gpointer);
    if fd == -(1 as ::core::ffi::c_int) {
        g_free(filesdir as gpointer);
        g_free(topdir as gpointer);
        g_free(trashname as gpointer);
        g_free(infofile as gpointer);
        safe_c2rust_g_set_io_error(
            error,
            glib_gettext(
                b"Unable to create trashing info file for %s: %s\0" as *const u8 as *const gchar,
            ),
            file,
            errsv as gint,
        );
        return FALSE;
    }
    g_close(fd as gint, ::core::ptr::null_mut::<*mut GError>());
    if is_homedir_trash != 0 {
        original_name = safe_c2rust_g_strdup_inline((*local).filename);
    } else {
        original_name = safe_c2rust_try_make_relative((*local).filename, topdir);
    }
    original_name_escaped = g_uri_escape_string(
        original_name,
        b"/\0" as *const u8 as *const ::core::ffi::c_char,
        FALSE,
    );
    g_free(original_name as gpointer);
    g_free(topdir as gpointer);
    let mut now: *mut GDateTime = g_date_time_new_now_local();
    if !now.is_null() {
        delete_time = g_date_time_format(now, b"%Y-%m-%dT%H:%M:%S\0" as *const u8 as *const gchar)
            as *mut ::core::ffi::c_char;
    } else {
        delete_time = safe_c2rust_g_strdup_inline(
            b"9999-12-31T23:59:59\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_date_time_unref(now);
    data = g_strdup_printf(
        b"[Trash Info]\nPath=%s\nDeletionDate=%s\n\0" as *const u8 as *const gchar,
        original_name_escaped,
        delete_time,
    ) as *mut ::core::ffi::c_char;
    g_free(delete_time as gpointer);
    if g_file_set_contents_full(
        infofile,
        data,
        -(1 as ::core::ffi::c_int) as gssize,
        (G_FILE_SET_CONTENTS_CONSISTENT as ::core::ffi::c_int
            | G_FILE_SET_CONTENTS_ONLY_EXISTING as ::core::ffi::c_int)
            as GFileSetContentsFlags,
        0o600 as ::core::ffi::c_int,
        error,
    ) == 0
    {
        g_unlink(infofile);
        g_free(filesdir as gpointer);
        g_free(trashname as gpointer);
        g_free(infofile as gpointer);
        return FALSE;
    }
    trashfile = g_build_filename(filesdir, trashname, NULL_0) as *mut ::core::ffi::c_char;
    g_free(filesdir as gpointer);
    if rename((*local).filename, trashfile) == -(1 as ::core::ffi::c_int) {
        errsv = *__errno_location();
        g_unlink(infofile);
        g_free(trashname as gpointer);
        g_free(infofile as gpointer);
        g_free(trashfile as gpointer);
        if errsv == EXDEV {
            safe_c2rust_g_set_io_error(
                error,
                glib_gettext(
                    b"Unable to trash file %s across filesystem boundaries\0" as *const u8
                        as *const gchar,
                ),
                file,
                ENOTSUP,
            );
        } else {
            safe_c2rust_g_set_io_error(
                error,
                glib_gettext(b"Unable to trash file %s: %s\0" as *const u8 as *const gchar),
                file,
                errsv as gint,
            );
        }
        return FALSE;
    }
    vfs = g_vfs_get_default();
    class = (*(vfs as *mut GTypeInstance)).g_class as *mut GVfsClass;
    if (*class).local_file_moved.is_some() {
        (*class)
            .local_file_moved
            .expect("non-null function pointer")(vfs, (*local).filename, trashfile);
    }
    g_free(trashfile as gpointer);
    g_free(infofile as gpointer);
    g_free(data as gpointer);
    g_free(original_name_escaped as gpointer);
    g_free(trashname as gpointer);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_local_file_make_directory(
    mut file: *mut GFile,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    if mkdir((*local).filename, 0o777 as __mode_t) == -(1 as ::core::ffi::c_int) {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        if errsv == EINVAL {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_FILENAME as ::core::ffi::c_int as gint,
                glib_gettext(b"Invalid filename\0" as *const u8 as *const gchar),
            );
        } else {
            safe_c2rust_g_set_io_error(
                error,
                glib_gettext(b"Error creating directory %s: %s\0" as *const u8 as *const gchar),
                file,
                errsv as gint,
            );
        }
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_local_file_make_symbolic_link(
    mut file: *mut GFile,
    mut symlink_value: *const ::core::ffi::c_char,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut local: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    if symlink(symlink_value, (*local).filename) == -(1 as ::core::ffi::c_int) {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        if errsv == EINVAL {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_FILENAME as ::core::ffi::c_int as gint,
                glib_gettext(b"Invalid filename\0" as *const u8 as *const gchar),
            );
        } else if errsv == EPERM {
            g_set_error(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Filesystem does not support symbolic links\0" as *const u8 as *const gchar,
                ),
            );
        } else {
            safe_c2rust_g_set_io_error(
                error,
                glib_gettext(b"Error making symbolic link %s: %s\0" as *const u8 as *const gchar),
                file,
                errsv as gint,
            );
        }
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_local_file_move(
    mut source: *mut GFile,
    mut destination: *mut GFile,
    mut flags: GFileCopyFlags,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileProgressCallback,
    mut progress_callback_data: gpointer,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut local_source: *mut GLocalFile = ::core::ptr::null_mut::<GLocalFile>();
    let mut local_destination: *mut GLocalFile = ::core::ptr::null_mut::<GLocalFile>();
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
    let mut destination_exist: gboolean = 0;
    let mut source_is_dir: gboolean = 0;
    let mut backup_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut res: ::core::ffi::c_int = 0;
    let mut source_size: off_t = 0;
    let mut class: *mut GVfsClass = ::core::ptr::null_mut::<GVfsClass>();
    let mut vfs: *mut GVfs = ::core::ptr::null_mut::<GVfs>();
    if ({
        let mut __inst: *mut GTypeInstance = source as *mut GTypeInstance;
        let mut __t: GType = safe_c2rust__g_local_file_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE as gboolean;
        } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE as gboolean;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) == 0
        || ({
            let mut __inst: *mut GTypeInstance = destination as *mut GTypeInstance;
            let mut __t: GType = safe_c2rust__g_local_file_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = FALSE as gboolean;
            } else if !(*__inst).g_class.is_null() && (*(*__inst).g_class).g_type == __t {
                __r = TRUE as gboolean;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) == 0
    {
        g_set_error_literal(
            error,
            g_io_error_quark(),
            G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
            b"Move not supported\0" as *const u8 as *const gchar,
        );
        return FALSE;
    }
    local_source = source as *mut ::core::ffi::c_void as *mut GLocalFile;
    local_destination = destination as *mut ::core::ffi::c_void as *mut GLocalFile;
    res = lstat((*local_source).filename, &raw mut statbuf);
    if res == -(1 as ::core::ffi::c_int) {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        safe_c2rust_g_set_io_error(
            error,
            glib_gettext(b"Error moving file %s: %s\0" as *const u8 as *const gchar),
            source,
            errsv as gint,
        );
        return FALSE;
    }
    source_is_dir = (statbuf.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
        as ::core::ffi::c_int as gboolean;
    source_size = statbuf.st_size as off_t;
    destination_exist = FALSE as gboolean;
    res = lstat((*local_destination).filename, &raw mut statbuf);
    if res == 0 as ::core::ffi::c_int {
        destination_exist = TRUE as gboolean;
        if flags as ::core::ffi::c_uint
            & G_FILE_COPY_OVERWRITE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            if statbuf.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t {
                if source_is_dir != 0 {
                    g_set_error_literal(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_WOULD_MERGE as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Can\xE2\x80\x99t move directory over directory\0" as *const u8
                                as *const gchar,
                        ),
                    );
                } else {
                    g_set_error_literal(
                        error,
                        g_io_error_quark(),
                        G_IO_ERROR_IS_DIRECTORY as ::core::ffi::c_int as gint,
                        glib_gettext(
                            b"Can\xE2\x80\x99t copy over directory\0" as *const u8 as *const gchar,
                        ),
                    );
                }
                return FALSE;
            }
        } else {
            safe_c2rust_g_set_io_error(
                error,
                glib_gettext(b"Error moving file %s: %s\0" as *const u8 as *const gchar),
                source,
                EEXIST,
            );
            return FALSE;
        }
    }
    if flags as ::core::ffi::c_uint
        & G_FILE_COPY_BACKUP as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && destination_exist != 0
    {
        backup_name = g_strconcat(
            (*local_destination).filename,
            b"~\0" as *const u8 as *const ::core::ffi::c_char,
            NULL_0,
        ) as *mut ::core::ffi::c_char;
        if rename((*local_destination).filename, backup_name) == -(1 as ::core::ffi::c_int) {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_CANT_CREATE_BACKUP as ::core::ffi::c_int as gint,
                glib_gettext(b"Backup file creation failed\0" as *const u8 as *const gchar),
            );
            g_free(backup_name as gpointer);
            return FALSE;
        }
        g_free(backup_name as gpointer);
        destination_exist = FALSE as gboolean;
    }
    if source_is_dir != 0
        && destination_exist != 0
        && flags as ::core::ffi::c_uint
            & G_FILE_COPY_OVERWRITE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
    {
        res = g_unlink((*local_destination).filename);
        if res == -(1 as ::core::ffi::c_int) {
            let mut errsv_0: ::core::ffi::c_int = *__errno_location();
            g_set_error(
                error,
                g_io_error_quark(),
                g_io_error_from_errno(errsv_0 as gint) as gint,
                glib_gettext(b"Error removing target file: %s\0" as *const u8 as *const gchar),
                g_strerror(errsv_0 as gint),
            );
            return FALSE;
        }
    }
    if rename((*local_source).filename, (*local_destination).filename) == -(1 as ::core::ffi::c_int)
    {
        let mut errsv_1: ::core::ffi::c_int = *__errno_location();
        if errsv_1 == EXDEV {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_NOT_SUPPORTED as ::core::ffi::c_int as gint,
                glib_gettext(b"Move between mounts not supported\0" as *const u8 as *const gchar),
            );
        } else if errsv_1 == EINVAL {
            g_set_error_literal(
                error,
                g_io_error_quark(),
                G_IO_ERROR_INVALID_FILENAME as ::core::ffi::c_int as gint,
                glib_gettext(b"Invalid filename\0" as *const u8 as *const gchar),
            );
        } else {
            safe_c2rust_g_set_io_error(
                error,
                glib_gettext(b"Error moving file %s: %s\0" as *const u8 as *const gchar),
                source,
                errsv_1 as gint,
            );
        }
        return FALSE;
    }
    vfs = g_vfs_get_default();
    class = (*(vfs as *mut GTypeInstance)).g_class as *mut GVfsClass;
    if (*class).local_file_moved.is_some() {
        (*class)
            .local_file_moved
            .expect("non-null function pointer")(
            vfs,
            (*local_source).filename,
            (*local_destination).filename,
        );
    }
    if progress_callback.is_some() {
        progress_callback.expect("non-null function pointer")(
            source_size as goffset,
            source_size as goffset,
            progress_callback_data,
        );
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_is_remote_fs_type(mut fsname: *const gchar) -> gboolean {
    if !fsname.is_null() {
        if strcmp(
            fsname as *const ::core::ffi::c_char,
            b"nfs\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            fsname as *const ::core::ffi::c_char,
            b"nfs4\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            fsname as *const ::core::ffi::c_char,
            b"cifs\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            fsname as *const ::core::ffi::c_char,
            b"smb\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            fsname as *const ::core::ffi::c_char,
            b"smb2\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            fsname as *const ::core::ffi::c_char,
            b"fuse.sshfs\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_local_file_is_nfs_home(
    mut filename: *const gchar,
) -> gboolean {
    static mut safe_c2rust_remote_home: gboolean = FALSE;
    static mut safe_c2rust_initialized: gsize = 0;
    let mut home: *const gchar = ::core::ptr::null::<gchar>();
    home = g_get_home_dir();
    if safe_c2rust_path_has_prefix(
        filename as *const ::core::ffi::c_char,
        home as *const ::core::ffi::c_char,
    ) != 0
    {
        if ({
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_initialized;
            } else {
            };
            (({
                let mut gapg_temp_newval: gsize = 0;
                let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_initialized;
                *&raw mut gapg_temp_newval =
                    crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
                gapg_temp_newval
            }) == 0
                && g_once_init_enter(&raw mut safe_c2rust_initialized as *mut ::core::ffi::c_void)
                    != 0) as ::core::ffi::c_int
        }) != 0
        {
            let mut file: *mut GFile = ::core::ptr::null_mut::<GFile>();
            let mut info: *mut GFileInfo = ::core::ptr::null_mut::<GFileInfo>();
            let mut fs_type: *const gchar = ::core::ptr::null::<gchar>();
            file = safe_c2rust__g_local_file_new(home as *const ::core::ffi::c_char);
            info = safe_c2rust_g_local_file_query_filesystem_info(
                file,
                G_FILE_ATTRIBUTE_FILESYSTEM_TYPE.as_ptr(),
                ::core::ptr::null_mut::<GCancellable>(),
                ::core::ptr::null_mut::<*mut GError>(),
            );
            if !info.is_null() {
                fs_type = g_file_info_get_attribute_string(
                    info,
                    G_FILE_ATTRIBUTE_FILESYSTEM_TYPE.as_ptr(),
                ) as *const gchar;
            }
            if g_strcmp0(
                fs_type as *const ::core::ffi::c_char,
                b"nfs\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || g_strcmp0(
                    fs_type as *const ::core::ffi::c_char,
                    b"nfs4\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                safe_c2rust_remote_home = TRUE as gboolean;
            }
            let mut _pp: *mut *mut GFileInfo = &raw mut info;
            let mut _ptr: *mut GFileInfo = *_pp;
            *_pp = ::core::ptr::null_mut::<GFileInfo>();
            if !_ptr.is_null() {
                g_object_unref(_ptr as gpointer);
            }
            g_object_unref(file as gpointer);
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_initialized =
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize;
            } else {
            };
            g_once_init_leave(
                &raw mut safe_c2rust_initialized as *mut ::core::ffi::c_void,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize,
            );
        }
        return safe_c2rust_remote_home;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_local_file_monitor_dir(
    mut file: *mut GFile,
    mut flags: GFileMonitorFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileMonitor {
    let mut local_file: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    return g_local_file_monitor_new_for_path((*local_file).filename, TRUE, flags, error);
}
unsafe extern "C" fn safe_c2rust_g_local_file_monitor_file(
    mut file: *mut GFile,
    mut flags: GFileMonitorFlags,
    mut cancellable: *mut GCancellable,
    mut error: *mut *mut GError,
) -> *mut GFileMonitor {
    let mut local_file: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    return g_local_file_monitor_new_for_path((*local_file).filename, FALSE, flags, error);
}
unsafe extern "C" fn safe_c2rust_g_local_file_measure_size_error(
    mut flags: GFileMeasureFlags,
    mut saved_errno: gint,
    mut name: *mut GSList,
    mut error: *mut *mut GError,
) -> gboolean {
    if (*name).next.is_null()
        || flags as ::core::ffi::c_uint
            & G_FILE_MEASURE_REPORT_ANY_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
    {
        let mut filename: *mut GString = ::core::ptr::null_mut::<GString>();
        let mut node: *mut GSList = ::core::ptr::null_mut::<GSList>();
        if error.is_null() {
            return FALSE;
        }
        filename = g_string_new((*name).data as *const gchar);
        node = (*name).next;
        while !node.is_null() {
            let mut utf8: *mut gchar = ::core::ptr::null_mut::<gchar>();
            g_string_prepend_c(filename, G_DIR_SEPARATOR as gchar);
            utf8 = g_filename_display_name((*node).data as *const gchar);
            g_string_prepend(filename, utf8);
            g_free(utf8 as gpointer);
            node = (*node).next;
        }
        g_set_error(
            error,
            g_io_error_quark(),
            g_io_error_from_errno(saved_errno) as gint,
            glib_gettext(
                b"Could not determine the disk usage of %s: %s\0" as *const u8 as *const gchar,
            ),
            (*filename).str_0,
            g_strerror(saved_errno),
        );
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(
                    filename,
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
                );
            } else {
                g_string_free_and_steal(filename);
            };
        } else {
            g_string_free(
                filename,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
            );
        };
        return FALSE;
    } else {
        return TRUE;
    };
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_is_size_usable(mut buf: *const statx) -> gboolean {
    return (safe_c2rust__g_stat_mode(buf) as ::core::ffi::c_int & __S_IFMT
        == 0o100000 as ::core::ffi::c_int
        || safe_c2rust__g_stat_mode(buf) as ::core::ffi::c_int & __S_IFMT
            == 0o120000 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_g_local_file_measure_size_of_file(
    mut parent_fd: gint,
    mut name: *mut GSList,
    mut state: *mut MeasureState,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut buf: statx = statx {
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
    if g_cancellable_set_error_if_cancelled((*state).cancellable, error) != 0 {
        return FALSE;
    }
    if safe_c2rust_g_local_file_fstatat(
        parent_fd as ::core::ffi::c_int,
        (*name).data as *const ::core::ffi::c_char,
        AT_SYMLINK_NOFOLLOW,
        2047 as GLocalFileStatField,
        (G_LOCAL_FILE_STAT_FIELD_ALL
            & !(G_LOCAL_FILE_STAT_FIELD_ATIME as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as GLocalFileStatField,
        &raw mut buf,
    ) != 0 as ::core::ffi::c_int
    {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        return safe_c2rust_g_local_file_measure_size_error(
            (*state).flags,
            errsv as gint,
            name,
            error,
        );
    }
    if !(*name).next.is_null() {
        if (*state).flags as ::core::ffi::c_uint
            & G_FILE_MEASURE_NO_XDEV as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            if (*state).contained_on != safe_c2rust__g_stat_dev(&raw mut buf) {
                return TRUE;
            }
        }
    } else {
        (*state).contained_on = safe_c2rust__g_stat_dev(&raw mut buf);
    }
    if !((*state).flags as ::core::ffi::c_uint)
        & G_FILE_MEASURE_APPARENT_SIZE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        (*state).disk_usage = ((*state).disk_usage as ::core::ffi::c_ulong).wrapping_add(
            (safe_c2rust__g_stat_blocks(&raw mut buf) as ::core::ffi::c_ulong)
                .wrapping_mul(512 as ::core::ffi::c_ulong),
        ) as guint64 as guint64;
    } else if safe_c2rust__g_stat_is_size_usable(&raw mut buf) != 0 {
        (*state).disk_usage = (*state)
            .disk_usage
            .wrapping_add(safe_c2rust__g_stat_size(&raw mut buf));
    }
    if safe_c2rust__g_stat_mode(&raw mut buf) as ::core::ffi::c_int & __S_IFMT
        == 0o40000 as ::core::ffi::c_int
    {
        (*state).num_dirs = (*state).num_dirs.wrapping_add(1);
    } else {
        (*state).num_files = (*state).num_files.wrapping_add(1);
    }
    if (*state).progress_callback.is_some() {
        if (*state).last_progress_report != 0 {
            let mut now: guint64 = 0;
            now = g_get_monotonic_time() as guint64;
            if (*state)
                .last_progress_report
                .wrapping_add((200 as ::core::ffi::c_long * G_TIME_SPAN_MILLISECOND) as guint64)
                < now
            {
                Some(
                    (*state)
                        .progress_callback
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    TRUE,
                    (*state).disk_usage,
                    (*state).num_dirs,
                    (*state).num_files,
                    (*state).progress_data,
                );
                (*state).last_progress_report = now;
            }
        } else {
            Some(
                (*state)
                    .progress_callback
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                TRUE,
                0 as guint64,
                0 as guint64,
                0 as guint64,
                (*state).progress_data,
            );
            (*state).last_progress_report = g_get_monotonic_time() as guint64;
        }
    }
    if safe_c2rust__g_stat_mode(&raw mut buf) as ::core::ffi::c_int & __S_IFMT
        == 0o40000 as ::core::ffi::c_int
    {
        let mut dir_fd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut errsv_0: ::core::ffi::c_int = 0;
        if g_cancellable_set_error_if_cancelled((*state).cancellable, error) != 0 {
            return FALSE;
        }
        dir_fd = openat(
            parent_fd as ::core::ffi::c_int,
            (*name).data as *const ::core::ffi::c_char,
            O_RDONLY | O_DIRECTORY | O_CLOEXEC,
        );
        errsv_0 = *__errno_location();
        if dir_fd < 0 as ::core::ffi::c_int {
            return safe_c2rust_g_local_file_measure_size_error(
                (*state).flags,
                errsv_0 as gint,
                name,
                error,
            );
        }
        if safe_c2rust_g_local_file_measure_size_of_contents(dir_fd as gint, name, state, error)
            == 0
        {
            return FALSE;
        }
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_local_file_measure_size_of_contents(
    mut fd: gint,
    mut dir_name: *mut GSList,
    mut state: *mut MeasureState,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut success: gboolean = TRUE;
    let mut name: *const gchar = ::core::ptr::null::<gchar>();
    let mut dir: *mut GDir = ::core::ptr::null_mut::<GDir>();
    let mut saved_errno: gint = 0;
    let mut dirp: *mut DIR = ::core::ptr::null_mut::<DIR>();
    dirp = fdopendir(fd as ::core::ffi::c_int);
    saved_errno = *__errno_location() as gint;
    dir = if !dirp.is_null() {
        (*glib__private__())
            .g_dir_new_from_dirp
            .expect("non-null function pointer")(dirp as gpointer)
    } else {
        ::core::ptr::null_mut::<GDir>()
    };
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if (dirp == ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut DIR)
            as ::core::ffi::c_int
            == (dir == ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut GDir)
                as ::core::ffi::c_int
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"/home/yans/safelibs/pipeline/ports/port-glib/original/gio/glocalfile.c\0" as *const u8
                as *const ::core::ffi::c_char,
            2961 as ::core::ffi::c_int,
            G_STRFUNC,
            b"(dirp == NULL) == (dir == NULL)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if dir.is_null() {
        close(fd as ::core::ffi::c_int);
        return safe_c2rust_g_local_file_measure_size_error(
            (*state).flags,
            saved_errno,
            dir_name,
            error,
        );
    }
    while success != 0 && {
        name = g_dir_read_name(dir);
        !name.is_null()
    } {
        let mut node: GSList = _GSList {
            data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            next: ::core::ptr::null_mut::<GSList>(),
        };
        node.next = dir_name;
        node.data = name as *mut gchar as gpointer;
        success = safe_c2rust_g_local_file_measure_size_of_file(fd, &raw mut node, state, error);
    }
    g_dir_close(dir);
    return success;
}
unsafe extern "C" fn safe_c2rust_g_local_file_measure_disk_usage(
    mut file: *mut GFile,
    mut flags: GFileMeasureFlags,
    mut cancellable: *mut GCancellable,
    mut progress_callback: GFileMeasureProgressCallback,
    mut progress_data: gpointer,
    mut disk_usage: *mut guint64,
    mut num_dirs: *mut guint64,
    mut num_files: *mut guint64,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut local_file: *mut GLocalFile = file as *mut ::core::ffi::c_void as *mut GLocalFile;
    let mut state: MeasureState = MeasureState {
        flags: G_FILE_MEASURE_NONE,
        contained_on: 0,
        cancellable: ::core::ptr::null_mut::<GCancellable>(),
        progress_callback: None,
        progress_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        disk_usage: 0,
        num_dirs: 0,
        num_files: 0,
        last_progress_report: 0,
    };
    let mut root_fd: gint = -(1 as gint);
    let mut node: GSList = _GSList {
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        next: ::core::ptr::null_mut::<GSList>(),
    };
    state.flags = flags;
    state.cancellable = cancellable;
    state.progress_callback = progress_callback;
    state.progress_data = progress_data;
    root_fd = AT_FDCWD as gint;
    node.data = (*local_file).filename as gpointer;
    node.next = ::core::ptr::null_mut::<GSList>();
    if safe_c2rust_g_local_file_measure_size_of_file(root_fd, &raw mut node, &raw mut state, error)
        == 0
    {
        return FALSE;
    }
    if !disk_usage.is_null() {
        *disk_usage = state.disk_usage;
    }
    if !num_dirs.is_null() {
        *num_dirs = state.num_dirs;
    }
    if !num_files.is_null() {
        *num_files = state.num_files;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_local_file_file_iface_init(mut iface: *mut GFileIface) {
    (*iface).dup =
        Some(safe_c2rust_g_local_file_dup as unsafe extern "C" fn(*mut GFile) -> *mut GFile)
            as Option<unsafe extern "C" fn(*mut GFile) -> *mut GFile>;
    (*iface).hash = Some(safe_c2rust_g_local_file_hash as unsafe extern "C" fn(*mut GFile) -> guint)
        as Option<unsafe extern "C" fn(*mut GFile) -> guint>;
    (*iface).equal = Some(
        safe_c2rust_g_local_file_equal as unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean,
    ) as Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean>;
    (*iface).is_native =
        Some(safe_c2rust_g_local_file_is_native as unsafe extern "C" fn(*mut GFile) -> gboolean)
            as Option<unsafe extern "C" fn(*mut GFile) -> gboolean>;
    (*iface).has_uri_scheme = Some(
        safe_c2rust_g_local_file_has_uri_scheme
            as unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> gboolean>;
    (*iface).get_uri_scheme = Some(
        safe_c2rust_g_local_file_get_uri_scheme
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_basename = Some(
        safe_c2rust_g_local_file_get_basename
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_path = Some(
        safe_c2rust_g_local_file_get_path
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_uri = Some(
        safe_c2rust_g_local_file_get_uri
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_parse_name = Some(
        safe_c2rust_g_local_file_get_parse_name
            as unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).get_parent =
        Some(safe_c2rust_g_local_file_get_parent as unsafe extern "C" fn(*mut GFile) -> *mut GFile)
            as Option<unsafe extern "C" fn(*mut GFile) -> *mut GFile>;
    (*iface).prefix_matches = Some(
        safe_c2rust_g_local_file_prefix_matches
            as unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean,
    )
        as Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> gboolean>;
    (*iface).get_relative_path = Some(
        safe_c2rust_g_local_file_get_relative_path
            as unsafe extern "C" fn(*mut GFile, *mut GFile) -> *mut ::core::ffi::c_char,
    )
        as Option<unsafe extern "C" fn(*mut GFile, *mut GFile) -> *mut ::core::ffi::c_char>;
    (*iface).resolve_relative_path = Some(
        safe_c2rust_g_local_file_resolve_relative_path
            as unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> *mut GFile,
    )
        as Option<unsafe extern "C" fn(*mut GFile, *const ::core::ffi::c_char) -> *mut GFile>;
    (*iface).get_child_for_display_name = Some(
        safe_c2rust_g_local_file_get_child_for_display_name
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                *mut *mut GError,
            ) -> *mut GFile,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                *mut *mut GError,
            ) -> *mut GFile,
        >;
    (*iface).set_display_name = Some(
        safe_c2rust_g_local_file_set_display_name
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFile,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFile,
        >;
    (*iface).enumerate_children = Some(
        safe_c2rust_g_local_file_enumerate_children
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                GFileQueryInfoFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileEnumerator,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                GFileQueryInfoFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileEnumerator,
        >;
    (*iface).query_info = Some(
        safe_c2rust_g_local_file_query_info
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                GFileQueryInfoFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                GFileQueryInfoFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
        >;
    (*iface).query_filesystem_info = Some(
        safe_c2rust_g_local_file_query_filesystem_info
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInfo,
        >;
    (*iface).find_enclosing_mount = Some(
        safe_c2rust_g_local_file_find_enclosing_mount
            as unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> *mut GMount,
    )
        as Option<
            unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> *mut GMount,
        >;
    (*iface).query_settable_attributes = Some(
        safe_c2rust_g_local_file_query_settable_attributes
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileAttributeInfoList,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileAttributeInfoList,
        >;
    (*iface).query_writable_namespaces = Some(
        safe_c2rust_g_local_file_query_writable_namespaces
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileAttributeInfoList,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileAttributeInfoList,
        >;
    (*iface).set_attribute = Some(
        safe_c2rust_g_local_file_set_attribute
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                GFileAttributeType,
                gpointer,
                GFileQueryInfoFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                GFileAttributeType,
                gpointer,
                GFileQueryInfoFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).set_attributes_from_info = Some(
        safe_c2rust_g_local_file_set_attributes_from_info
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GFileInfo,
                GFileQueryInfoFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GFileInfo,
                GFileQueryInfoFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).read_fn = Some(
        safe_c2rust_g_local_file_read
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInputStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileInputStream,
        >;
    (*iface).append_to = Some(
        safe_c2rust_g_local_file_append_to
            as unsafe extern "C" fn(
                *mut GFile,
                GFileCreateFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileOutputStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                GFileCreateFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileOutputStream,
        >;
    (*iface).create = Some(
        safe_c2rust_g_local_file_create
            as unsafe extern "C" fn(
                *mut GFile,
                GFileCreateFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileOutputStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                GFileCreateFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileOutputStream,
        >;
    (*iface).replace = Some(
        safe_c2rust_g_local_file_replace
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                gboolean,
                GFileCreateFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileOutputStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                gboolean,
                GFileCreateFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileOutputStream,
        >;
    (*iface).open_readwrite = Some(
        safe_c2rust_g_local_file_open_readwrite
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileIOStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileIOStream,
        >;
    (*iface).create_readwrite = Some(
        safe_c2rust_g_local_file_create_readwrite
            as unsafe extern "C" fn(
                *mut GFile,
                GFileCreateFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileIOStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                GFileCreateFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileIOStream,
        >;
    (*iface).replace_readwrite = Some(
        safe_c2rust_g_local_file_replace_readwrite
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                gboolean,
                GFileCreateFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileIOStream,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                gboolean,
                GFileCreateFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileIOStream,
        >;
    (*iface).delete_file = Some(
        safe_c2rust_g_local_file_delete
            as unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
    (*iface).trash = Some(
        safe_c2rust_g_local_file_trash
            as unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
    (*iface).make_directory = Some(
        safe_c2rust_g_local_file_make_directory
            as unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(*mut GFile, *mut GCancellable, *mut *mut GError) -> gboolean,
        >;
    (*iface).make_symbolic_link = Some(
        safe_c2rust_g_local_file_make_symbolic_link
            as unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *const ::core::ffi::c_char,
                *mut GCancellable,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).move_0 = Some(
        safe_c2rust_g_local_file_move
            as unsafe extern "C" fn(
                *mut GFile,
                *mut GFile,
                GFileCopyFlags,
                *mut GCancellable,
                GFileProgressCallback,
                gpointer,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                *mut GFile,
                GFileCopyFlags,
                *mut GCancellable,
                GFileProgressCallback,
                gpointer,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).monitor_dir = Some(
        safe_c2rust_g_local_file_monitor_dir
            as unsafe extern "C" fn(
                *mut GFile,
                GFileMonitorFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileMonitor,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                GFileMonitorFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileMonitor,
        >;
    (*iface).monitor_file = Some(
        safe_c2rust_g_local_file_monitor_file
            as unsafe extern "C" fn(
                *mut GFile,
                GFileMonitorFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileMonitor,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                GFileMonitorFlags,
                *mut GCancellable,
                *mut *mut GError,
            ) -> *mut GFileMonitor,
        >;
    (*iface).measure_disk_usage = Some(
        safe_c2rust_g_local_file_measure_disk_usage
            as unsafe extern "C" fn(
                *mut GFile,
                GFileMeasureFlags,
                *mut GCancellable,
                GFileMeasureProgressCallback,
                gpointer,
                *mut guint64,
                *mut guint64,
                *mut guint64,
                *mut *mut GError,
            ) -> gboolean,
    )
        as Option<
            unsafe extern "C" fn(
                *mut GFile,
                GFileMeasureFlags,
                *mut GCancellable,
                GFileMeasureProgressCallback,
                gpointer,
                *mut guint64,
                *mut guint64,
                *mut guint64,
                *mut *mut GError,
            ) -> gboolean,
        >;
    (*iface).supports_thread_contexts = TRUE as gboolean;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
