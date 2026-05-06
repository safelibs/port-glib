extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn mkdir(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fallocate(
        __fd: ::core::ffi::c_int,
        __mode: ::core::ffi::c_int,
        __offset: __off64_t,
        __len: __off64_t,
    ) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn access(__name: *const ::core::ffi::c_char, __type: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn getcwd(__buf: *mut ::core::ffi::c_char, __size: size_t) -> *mut ::core::ffi::c_char;
    fn getuid() -> __uid_t;
    fn readlink(
        __path: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ftruncate(__fd: ::core::ffi::c_int, __length: __off64_t) -> ::core::ffi::c_int;
    fn g_quark_from_static_string(string: *const gchar) -> GQuark;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_propagate_error(dest: *mut *mut GError, src: *mut GError);
    fn g_get_tmp_dir() -> *const gchar;
    fn g_format_size_full(size: guint64, flags: GFormatSizeFlags) -> *mut gchar;
    fn g_filename_display_name(filename: *const gchar) -> *mut gchar;
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_try_malloc(n_bytes: gsize) -> gpointer;
    fn g_try_realloc(mem: gpointer, n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_get_real_time() -> gint64;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_strrstr(haystack: *const gchar, needle: *const gchar) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strdup_printf(format: *const gchar, ...) -> *mut gchar;
    fn g_strconcat(string1: *const gchar, ...) -> *mut gchar;
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
    fn g_unlink(filename: *const gchar) -> ::core::ffi::c_int;
    fn g_close(fd: gint, error: *mut *mut GError) -> gboolean;
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
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type off_t = __off64_t;
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
pub type __gnuc_va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type va_list = __gnuc_va_list;
pub type ssize_t = __ssize_t;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
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
pub type GFileTest = ::core::ffi::c_uint;
pub const G_FILE_TEST_EXISTS: GFileTest = 16;
pub const G_FILE_TEST_IS_EXECUTABLE: GFileTest = 8;
pub const G_FILE_TEST_IS_DIR: GFileTest = 4;
pub const G_FILE_TEST_IS_SYMLINK: GFileTest = 2;
pub const G_FILE_TEST_IS_REGULAR: GFileTest = 1;
pub type GFileSetContentsFlags = ::core::ffi::c_uint;
pub const G_FILE_SET_CONTENTS_ONLY_EXISTING: GFileSetContentsFlags = 4;
pub const G_FILE_SET_CONTENTS_DURABLE: GFileSetContentsFlags = 2;
pub const G_FILE_SET_CONTENTS_CONSISTENT: GFileSetContentsFlags = 1;
pub const G_FILE_SET_CONTENTS_NONE: GFileSetContentsFlags = 0;
pub type GFormatSizeFlags = ::core::ffi::c_uint;
pub const G_FORMAT_SIZE_ONLY_UNIT: GFormatSizeFlags = 16;
pub const G_FORMAT_SIZE_ONLY_VALUE: GFormatSizeFlags = 8;
pub const G_FORMAT_SIZE_BITS: GFormatSizeFlags = 4;
pub const G_FORMAT_SIZE_IEC_UNITS: GFormatSizeFlags = 2;
pub const G_FORMAT_SIZE_LONG_FORMAT: GFormatSizeFlags = 1;
pub const G_FORMAT_SIZE_DEFAULT: GFormatSizeFlags = 0;
pub type GTmpFileCallback = Option<unsafe extern "C" fn(*const gchar, gint, gint) -> gint>;
pub type GString = _GString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
static mut safe_c2rust_NLETTERS: ::core::ffi::c_int = 0;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const PATH_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_IEXEC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const G_MAXOFFSET: ::core::ffi::c_long = G_MAXINT64;
pub const G_DIR_SEPARATOR: ::core::ffi::c_int = '/' as i32;
pub const G_DIR_SEPARATOR_S: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"/\0") };
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENAMETOOLONG: ::core::ffi::c_int = 36;
pub const ENOSYS: ::core::ffi::c_int = 38;
pub const ELOOP: ::core::ffi::c_int = 40;
pub const EPERM: ::core::ffi::c_int = 1;
pub const ENOENT: ::core::ffi::c_int = 2;
pub const EINTR: ::core::ffi::c_int = 4;
pub const EIO: ::core::ffi::c_int = 5;
pub const ENXIO: ::core::ffi::c_int = 6;
pub const EBADF: ::core::ffi::c_int = 9;
pub const EAGAIN: ::core::ffi::c_int = 11;
pub const ENOMEM: ::core::ffi::c_int = 12;
pub const EACCES: ::core::ffi::c_int = 13;
pub const EFAULT: ::core::ffi::c_int = 14;
pub const EEXIST: ::core::ffi::c_int = 17;
pub const ENODEV: ::core::ffi::c_int = 19;
pub const ENOTDIR: ::core::ffi::c_int = 20;
pub const EISDIR: ::core::ffi::c_int = 21;
pub const EINVAL: ::core::ffi::c_int = 22;
pub const ENFILE: ::core::ffi::c_int = 23;
pub const EMFILE: ::core::ffi::c_int = 24;
pub const ETXTBSY: ::core::ffi::c_int = 26;
pub const ENOSPC: ::core::ffi::c_int = 28;
pub const EROFS: ::core::ffi::c_int = 30;
pub const EPIPE: ::core::ffi::c_int = 32;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const S_IXUSR: ::core::ffi::c_int = __S_IEXEC;
pub const S_IXGRP: ::core::ffi::c_int = S_IXUSR >> 3 as ::core::ffi::c_int;
pub const S_IXOTH: ::core::ffi::c_int = S_IXGRP >> 3 as ::core::ffi::c_int;
pub const X_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const G_MAXINT64: ::core::ffi::c_long = 0x7fffffffffffffff as ::core::ffi::c_long;
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_fd(
    mut fd_ptr: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = *fd_ptr;
    *fd_ptr = -(1 as ::core::ffi::c_int);
    return fd;
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mkdir_with_parents(
    mut pathname: *const gchar,
    mut mode: ::core::ffi::c_int,
) -> gint {
    let mut fn_0: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if pathname.is_null() || *pathname as ::core::ffi::c_int == '\0' as i32 {
        *__errno_location() = EINVAL;
        return -(1 as gint);
    }
    if mkdir(pathname as *const ::core::ffi::c_char, mode as __mode_t) == 0 as ::core::ffi::c_int {
        return 0 as gint;
    } else if *__errno_location() == EEXIST {
        if safe_c2rust_g_file_test(pathname, G_FILE_TEST_IS_DIR) == 0 {
            *__errno_location() = ENOTDIR;
            return -(1 as gint);
        }
        return 0 as gint;
    }
    fn_0 = safe_c2rust_g_strdup_inline(pathname as *const ::core::ffi::c_char) as *mut gchar;
    if safe_c2rust_g_path_is_absolute(fn_0) != 0 {
        p = safe_c2rust_g_path_skip_root(fn_0) as *mut gchar;
    } else {
        p = fn_0;
    }
    loop {
        while *p as ::core::ffi::c_int != 0 && !(*p as ::core::ffi::c_int == G_DIR_SEPARATOR) {
            p = p.offset(1);
        }
        if *p == 0 {
            p = ::core::ptr::null_mut::<gchar>();
        } else {
            *p = '\0' as i32 as gchar;
        }
        if safe_c2rust_g_file_test(fn_0, G_FILE_TEST_EXISTS) == 0 {
            if mkdir(fn_0, mode as __mode_t) == -(1 as ::core::ffi::c_int)
                && *__errno_location() != EEXIST
            {
                let mut errno_save: ::core::ffi::c_int = *__errno_location();
                if *__errno_location() != ENOENT || p.is_null() {
                    g_free(fn_0 as gpointer);
                    *__errno_location() = errno_save;
                    return -(1 as gint);
                }
            }
        } else if safe_c2rust_g_file_test(fn_0, G_FILE_TEST_IS_DIR) == 0 {
            g_free(fn_0 as gpointer);
            *__errno_location() = ENOTDIR;
            return -(1 as gint);
        }
        if !p.is_null() {
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = G_DIR_SEPARATOR as gchar;
            while *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int == G_DIR_SEPARATOR {
                p = p.offset(1);
            }
        }
        if p.is_null() {
            break;
        }
    }
    g_free(fn_0 as gpointer);
    return 0 as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_test(
    mut filename: *const gchar,
    mut test: GFileTest,
) -> gboolean {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !filename.is_null() {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
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
    if test as ::core::ffi::c_uint & G_FILE_TEST_EXISTS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && access(filename as *const ::core::ffi::c_char, F_OK) == 0 as ::core::ffi::c_int
    {
        return TRUE;
    }
    if test as ::core::ffi::c_uint
        & G_FILE_TEST_IS_EXECUTABLE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && access(filename as *const ::core::ffi::c_char, X_OK) == 0 as ::core::ffi::c_int
    {
        if getuid() != 0 as __uid_t {
            return TRUE;
        }
    } else {
        test = ::core::mem::transmute::<::core::ffi::c_uint, GFileTest>(
            test as ::core::ffi::c_uint
                & !(G_FILE_TEST_IS_EXECUTABLE as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    }
    if test as ::core::ffi::c_uint
        & G_FILE_TEST_IS_SYMLINK as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        let mut s: stat = stat {
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
        if lstat(filename as *const ::core::ffi::c_char, &raw mut s) == 0 as ::core::ffi::c_int
            && s.st_mode & __S_IFMT as __mode_t == 0o120000 as __mode_t
        {
            return TRUE;
        }
    }
    if test as ::core::ffi::c_uint
        & (G_FILE_TEST_IS_REGULAR as ::core::ffi::c_int
            | G_FILE_TEST_IS_DIR as ::core::ffi::c_int
            | G_FILE_TEST_IS_EXECUTABLE as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
    {
        let mut s_0: stat = stat {
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
        if stat(filename as *const ::core::ffi::c_char, &raw mut s_0) == 0 as ::core::ffi::c_int {
            if test as ::core::ffi::c_uint
                & G_FILE_TEST_IS_REGULAR as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
                && s_0.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
            {
                return TRUE;
            }
            if test as ::core::ffi::c_uint
                & G_FILE_TEST_IS_DIR as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
                && s_0.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
            {
                return TRUE;
            }
            if test as ::core::ffi::c_uint
                & G_FILE_TEST_IS_EXECUTABLE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
                && (s_0.st_mode & S_IXOTH as __mode_t != 0
                    || s_0.st_mode & S_IXUSR as __mode_t != 0
                    || s_0.st_mode & S_IXGRP as __mode_t != 0)
            {
                return TRUE;
            }
        }
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_error_quark() -> GQuark {
    static mut safe_c2rust_q: GQuark = 0;
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if safe_c2rust_q == 0 as GQuark {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_q =
            g_quark_from_static_string(b"g-file-error-quark\0" as *const u8 as *const gchar);
    }
    return safe_c2rust_q;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_error_from_errno(mut err_no: gint) -> GFileError {
    match err_no {
        EEXIST => return G_FILE_ERROR_EXIST,
        EISDIR => return G_FILE_ERROR_ISDIR,
        EACCES => return G_FILE_ERROR_ACCES,
        ENAMETOOLONG => return G_FILE_ERROR_NAMETOOLONG,
        ENOENT => return G_FILE_ERROR_NOENT,
        ENOTDIR => return G_FILE_ERROR_NOTDIR,
        ENXIO => return G_FILE_ERROR_NXIO,
        ENODEV => return G_FILE_ERROR_NODEV,
        EROFS => return G_FILE_ERROR_ROFS,
        ETXTBSY => return G_FILE_ERROR_TXTBSY,
        EFAULT => return G_FILE_ERROR_FAULT,
        ELOOP => return G_FILE_ERROR_LOOP,
        ENOSPC => return G_FILE_ERROR_NOSPC,
        ENOMEM => return G_FILE_ERROR_NOMEM,
        EMFILE => return G_FILE_ERROR_MFILE,
        ENFILE => return G_FILE_ERROR_NFILE,
        EBADF => return G_FILE_ERROR_BADF,
        EINVAL => return G_FILE_ERROR_INVAL,
        EPIPE => return G_FILE_ERROR_PIPE,
        EAGAIN => return G_FILE_ERROR_AGAIN,
        EINTR => return G_FILE_ERROR_INTR,
        EIO => return G_FILE_ERROR_IO,
        EPERM => return G_FILE_ERROR_PERM,
        ENOSYS => return G_FILE_ERROR_NOSYS,
        _ => return G_FILE_ERROR_FAILED,
    };
}
unsafe extern "C" fn safe_c2rust_format_error_message(
    mut filename: *const gchar,
    mut format_string: *const gchar,
    mut saved_errno: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut display_name: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut msg: *mut gchar = ::core::ptr::null_mut::<gchar>();
    display_name = g_filename_display_name(filename);
    msg = g_strdup_printf(format_string, display_name, g_strerror(saved_errno as gint));
    g_free(display_name as gpointer);
    return msg as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn safe_c2rust_set_file_error(
    mut error: *mut *mut GError,
    mut filename: *const gchar,
    mut format_string: *const gchar,
    mut saved_errno: ::core::ffi::c_int,
) {
    let mut msg: *mut ::core::ffi::c_char =
        safe_c2rust_format_error_message(filename, format_string, saved_errno);
    g_set_error_literal(
        error,
        safe_c2rust_g_file_error_quark(),
        safe_c2rust_g_file_error_from_errno(saved_errno as gint) as gint,
        msg,
    );
    g_free(msg as gpointer);
}
unsafe extern "C" fn safe_c2rust_get_contents_stdio(
    mut filename: *const gchar,
    mut f: *mut FILE,
    mut contents: *mut *mut gchar,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut buf: [gchar; 4096] = [0; 4096];
    let mut bytes: gsize = 0;
    let mut str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut total_bytes: gsize = 0 as gsize;
    let mut total_allocated: gsize = 0 as gsize;
    let mut tmp: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut display_filename: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !f.is_null() {
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
            b"../original/glib/gfileutils.c\0" as *const u8 as *const ::core::ffi::c_char,
            682 as ::core::ffi::c_int,
            G_STRFUNC,
            b"f != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    's_36: loop {
        if !(feof(f) == 0) {
            current_block = 2569451025026770673;
            break;
        }
        let mut save_errno: gint = 0;
        bytes = fread(
            &raw mut buf as *mut gchar as *mut ::core::ffi::c_void,
            1 as size_t,
            ::core::mem::size_of::<[gchar; 4096]>() as size_t,
            f,
        ) as gsize;
        save_errno = *__errno_location() as gint;
        if total_bytes > G_MAXSIZE.wrapping_sub(bytes) {
            current_block = 11793043946306791178;
            break;
        }
        while total_bytes.wrapping_add(bytes) >= total_allocated {
            if !str.is_null() {
                if total_allocated > G_MAXSIZE.wrapping_div(2 as ::core::ffi::c_ulong) {
                    current_block = 11793043946306791178;
                    break 's_36;
                }
                total_allocated = total_allocated.wrapping_mul(2 as gsize);
            } else {
                total_allocated = (if (bytes as usize).wrapping_add(1 as usize)
                    < ::core::mem::size_of::<[gchar; 4096]>() as usize
                {
                    (bytes as usize).wrapping_add(1 as usize)
                } else {
                    ::core::mem::size_of::<[gchar; 4096]>() as usize
                }) as gsize;
            }
            tmp = g_try_realloc(str as gpointer, total_allocated) as *mut gchar;
            if tmp.is_null() {
                let mut display_size: *mut ::core::ffi::c_char =
                    g_format_size_full(total_allocated as guint64, G_FORMAT_SIZE_LONG_FORMAT)
                        as *mut ::core::ffi::c_char;
                display_filename = g_filename_display_name(filename);
                g_set_error(
                    error,
                    safe_c2rust_g_file_error_quark(),
                    G_FILE_ERROR_NOMEM as ::core::ffi::c_int as gint,
                    glib_gettext(
                        b"Could not allocate %s to read file \xE2\x80\x9C%s\xE2\x80\x9D\0"
                            as *const u8 as *const gchar,
                    ),
                    display_size,
                    display_filename,
                );
                g_free(display_filename as gpointer);
                g_free(display_size as gpointer);
                current_block = 12799301927417478855;
                break 's_36;
            } else {
                str = tmp;
            }
        }
        if ferror(f) != 0 {
            display_filename = g_filename_display_name(filename);
            g_set_error(
                error,
                safe_c2rust_g_file_error_quark(),
                safe_c2rust_g_file_error_from_errno(save_errno) as gint,
                glib_gettext(
                    b"Error reading file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                        as *const gchar,
                ),
                display_filename,
                g_strerror(save_errno),
            );
            g_free(display_filename as gpointer);
            current_block = 12799301927417478855;
            break;
        } else {
            if ({
                let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
                if !str.is_null() {
                    _g_boolean_var_11 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_11 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_11
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gfileutils.c\0" as *const u8 as *const ::core::ffi::c_char,
                    746 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            memcpy(
                str.offset(total_bytes as isize) as *mut ::core::ffi::c_void,
                &raw mut buf as *mut gchar as *const ::core::ffi::c_void,
                bytes as size_t,
            );
            total_bytes = total_bytes.wrapping_add(bytes);
        }
    }
    match current_block {
        2569451025026770673 => {
            fclose(f);
            if total_allocated == 0 as gsize {
                str = ({
                    let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
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
                total_bytes = 0 as gsize;
            }
            *str.offset(total_bytes as isize) = '\0' as i32 as gchar;
            if !length.is_null() {
                *length = total_bytes;
            }
            *contents = str;
            return TRUE;
        }
        11793043946306791178 => {
            display_filename = g_filename_display_name(filename);
            g_set_error(
                error,
                safe_c2rust_g_file_error_quark(),
                G_FILE_ERROR_FAILED as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"File \xE2\x80\x9C%s\xE2\x80\x9D is too large\0" as *const u8 as *const gchar,
                ),
                display_filename,
            );
            g_free(display_filename as gpointer);
        }
        _ => {}
    }
    g_free(str as gpointer);
    fclose(f);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_get_contents_regfile(
    mut filename: *const gchar,
    mut stat_buf: *mut stat,
    mut fd: gint,
    mut contents: *mut *mut gchar,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut current_block: u64;
    let mut buf: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut bytes_read: gsize = 0;
    let mut size: gsize = 0;
    let mut alloc_size: gsize = 0;
    let mut display_filename: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if G_MAXOFFSET as ::core::ffi::c_ulong >= G_MAXSIZE
        && (*stat_buf).st_size > G_MAXSIZE.wrapping_sub(1 as ::core::ffi::c_ulong) as goffset
    {
        display_filename = g_filename_display_name(filename);
        g_set_error(
            error,
            safe_c2rust_g_file_error_quark(),
            G_FILE_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"File \xE2\x80\x9C%s\xE2\x80\x9D is too large\0" as *const u8 as *const gchar,
            ),
            display_filename,
        );
        g_free(display_filename as gpointer);
    } else {
        size = (*stat_buf).st_size as gsize;
        alloc_size = size.wrapping_add(1 as gsize);
        buf = g_try_malloc(alloc_size) as *mut gchar;
        if buf.is_null() {
            let mut display_size: *mut ::core::ffi::c_char =
                g_format_size_full(alloc_size as guint64, G_FORMAT_SIZE_LONG_FORMAT)
                    as *mut ::core::ffi::c_char;
            display_filename = g_filename_display_name(filename);
            g_set_error(
                error,
                safe_c2rust_g_file_error_quark(),
                G_FILE_ERROR_NOMEM as ::core::ffi::c_int as gint,
                glib_gettext(
                    b"Could not allocate %s to read file \xE2\x80\x9C%s\xE2\x80\x9D\0" as *const u8
                        as *const gchar,
                ),
                display_size,
                display_filename,
            );
            g_free(display_filename as gpointer);
            g_free(display_size as gpointer);
        } else {
            bytes_read = 0 as gsize;
            loop {
                if !(bytes_read < size) {
                    current_block = 5783071609795492627;
                    break;
                }
                let mut rc: gssize = 0;
                rc = read(
                    fd as ::core::ffi::c_int,
                    buf.offset(bytes_read as isize) as *mut ::core::ffi::c_void,
                    (size as size_t).wrapping_sub(bytes_read as size_t),
                ) as gssize;
                if rc < 0 as gssize {
                    if !(*__errno_location() != EINTR) {
                        continue;
                    }
                    let mut save_errno: ::core::ffi::c_int = *__errno_location();
                    g_free(buf as gpointer);
                    display_filename = g_filename_display_name(filename);
                    g_set_error(
                        error,
                        safe_c2rust_g_file_error_quark(),
                        safe_c2rust_g_file_error_from_errno(save_errno as gint) as gint,
                        glib_gettext(
                            b"Failed to read from file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0"
                                as *const u8 as *const gchar,
                        ),
                        display_filename,
                        g_strerror(save_errno as gint),
                    );
                    g_free(display_filename as gpointer);
                    current_block = 4433193763129298609;
                    break;
                } else {
                    if rc == 0 as gssize {
                        current_block = 5783071609795492627;
                        break;
                    }
                    bytes_read = bytes_read.wrapping_add(rc as gsize);
                }
            }
            match current_block {
                4433193763129298609 => {}
                _ => {
                    *buf.offset(bytes_read as isize) = '\0' as i32 as gchar;
                    if !length.is_null() {
                        *length = bytes_read;
                    }
                    *contents = buf;
                    close(fd as ::core::ffi::c_int);
                    return TRUE;
                }
            }
        }
    }
    close(fd as ::core::ffi::c_int);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_get_contents_posix(
    mut filename: *const gchar,
    mut contents: *mut *mut gchar,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> gboolean {
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
    let mut fd: gint = 0;
    fd = open(
        filename as *const ::core::ffi::c_char,
        O_RDONLY | O_BINARY | O_CLOEXEC,
    ) as gint;
    if fd < 0 as ::core::ffi::c_int {
        let mut saved_errno: ::core::ffi::c_int = *__errno_location();
        if !error.is_null() {
            safe_c2rust_set_file_error(
                error,
                filename,
                glib_gettext(
                    b"Failed to open file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                        as *const gchar,
                ),
                saved_errno,
            );
        }
        return FALSE;
    }
    if fstat(fd as ::core::ffi::c_int, &raw mut stat_buf) < 0 as ::core::ffi::c_int {
        let mut saved_errno_0: ::core::ffi::c_int = *__errno_location();
        if !error.is_null() {
            safe_c2rust_set_file_error(
                error,
                filename,
                glib_gettext(
                    b"Failed to get attributes of file \xE2\x80\x9C%s\xE2\x80\x9D: fstat() failed: %s\0"
                        as *const u8 as *const gchar,
                ),
                saved_errno_0,
            );
        }
        close(fd as ::core::ffi::c_int);
        return FALSE;
    }
    if stat_buf.st_size > 0 as __off_t
        && stat_buf.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
    {
        let mut retval: gboolean = safe_c2rust_get_contents_regfile(
            filename,
            &raw mut stat_buf,
            fd,
            contents,
            length,
            error,
        );
        return retval;
    } else {
        let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut retval_0: gboolean = 0;
        f = fdopen(
            fd as ::core::ffi::c_int,
            b"r\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if f.is_null() {
            let mut saved_errno_1: ::core::ffi::c_int = *__errno_location();
            if !error.is_null() {
                safe_c2rust_set_file_error(
                    error,
                    filename,
                    glib_gettext(
                        b"Failed to open file \xE2\x80\x9C%s\xE2\x80\x9D: fdopen() failed: %s\0"
                            as *const u8 as *const gchar,
                    ),
                    saved_errno_1,
                );
            }
            return FALSE;
        }
        retval_0 = safe_c2rust_get_contents_stdio(filename, f, contents, length, error);
        return retval_0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_get_contents(
    mut filename: *const gchar,
    mut contents: *mut *mut gchar,
    mut length: *mut gsize,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !filename.is_null() {
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
            b"filename != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !contents.is_null() {
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
            b"contents != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    *contents = ::core::ptr::null_mut::<gchar>();
    if !length.is_null() {
        *length = 0 as gsize;
    }
    return safe_c2rust_get_contents_posix(filename, contents, length, error);
}
unsafe extern "C" fn safe_c2rust_rename_file(
    mut old_name: *const ::core::ffi::c_char,
    mut new_name: *const ::core::ffi::c_char,
    mut do_fsync: gboolean,
    mut err: *mut *mut GError,
) -> gboolean {
    *__errno_location() = 0 as ::core::ffi::c_int;
    if rename(old_name, new_name) == -(1 as ::core::ffi::c_int) {
        let mut save_errno: ::core::ffi::c_int = *__errno_location();
        let mut display_old_name: *mut gchar = g_filename_display_name(old_name as *const gchar);
        let mut display_new_name: *mut gchar = g_filename_display_name(new_name as *const gchar);
        g_set_error(
            err,
            safe_c2rust_g_file_error_quark(),
            safe_c2rust_g_file_error_from_errno(save_errno as gint) as gint,
            glib_gettext(
                b"Failed to rename file \xE2\x80\x9C%s\xE2\x80\x9D to \xE2\x80\x9C%s\xE2\x80\x9D: g_rename() failed: %s\0"
                    as *const u8 as *const gchar,
            ),
            display_old_name,
            display_new_name,
            g_strerror(save_errno as gint),
        );
        g_free(display_old_name as gpointer);
        g_free(display_new_name as gpointer);
        return FALSE;
    }
    if do_fsync != 0 {
        let mut dir: *mut gchar = safe_c2rust_g_path_get_dirname(new_name as *const gchar);
        let mut dir_fd: ::core::ffi::c_int =
            open(dir, O_RDONLY | O_CLOEXEC, 0 as ::core::ffi::c_int);
        if dir_fd >= 0 as ::core::ffi::c_int {
            fsync(dir_fd);
            g_close(dir_fd as gint, ::core::ptr::null_mut::<*mut GError>());
        }
        g_free(dir as gpointer);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_fd_should_be_fsynced(
    mut fd: ::core::ffi::c_int,
    mut test_file: *const gchar,
    mut flags: GFileSetContentsFlags,
) -> gboolean {
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
    if flags as ::core::ffi::c_uint
        & (G_FILE_SET_CONTENTS_CONSISTENT as ::core::ffi::c_int
            | G_FILE_SET_CONTENTS_DURABLE as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
        && flags as ::core::ffi::c_uint
            & G_FILE_SET_CONTENTS_ONLY_EXISTING as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
    {
        *__errno_location() = 0 as ::core::ffi::c_int;
        if lstat(test_file as *const ::core::ffi::c_char, &raw mut statbuf)
            == 0 as ::core::ffi::c_int
        {
            return (statbuf.st_size > 0 as __off_t) as ::core::ffi::c_int;
        } else if *__errno_location() == ENOENT {
            return FALSE;
        } else {
            return TRUE;
        }
    } else {
        return (flags as ::core::ffi::c_uint
            & (G_FILE_SET_CONTENTS_CONSISTENT as ::core::ffi::c_int
                | G_FILE_SET_CONTENTS_DURABLE as ::core::ffi::c_int)
                as ::core::ffi::c_uint) as gboolean;
    };
}
unsafe extern "C" fn safe_c2rust_truncate_file(
    mut fd: ::core::ffi::c_int,
    mut length: off_t,
    mut dest_file: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> gboolean {
    while ftruncate(fd, length as __off64_t) < 0 as ::core::ffi::c_int {
        let mut saved_errno: ::core::ffi::c_int = *__errno_location();
        if saved_errno == EINTR {
            continue;
        }
        if !error.is_null() {
            safe_c2rust_set_file_error(
                error,
                dest_file as *const gchar,
                glib_gettext(
                    b"Failed to write file \xE2\x80\x9C%s\xE2\x80\x9D: ftruncate() failed: %s\0"
                        as *const u8 as *const gchar,
                ),
                saved_errno,
            );
        }
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_write_to_file(
    mut contents: *const gchar,
    mut length: gsize,
    mut fd: ::core::ffi::c_int,
    mut dest_file: *const gchar,
    mut do_fsync: gboolean,
    mut err: *mut *mut GError,
) -> gboolean {
    if length > 0 as gsize {
        fallocate(
            fd,
            0 as ::core::ffi::c_int,
            0 as __off64_t,
            length as __off64_t,
        );
    }
    while length > 0 as gsize {
        let mut s: gssize = 0;
        s = write(
            fd,
            contents as *const ::core::ffi::c_void,
            if length < 9223372036854775807 as ::core::ffi::c_long as gsize {
                length as size_t
            } else {
                9223372036854775807 as ::core::ffi::c_long as size_t
            },
        ) as gssize;
        if s < 0 as gssize {
            let mut saved_errno: ::core::ffi::c_int = *__errno_location();
            if saved_errno == EINTR {
                continue;
            }
            if !err.is_null() {
                safe_c2rust_set_file_error(
                    err,
                    dest_file,
                    glib_gettext(
                        b"Failed to write file \xE2\x80\x9C%s\xE2\x80\x9D: write() failed: %s\0"
                            as *const u8 as *const gchar,
                    ),
                    saved_errno,
                );
            }
            close(fd);
            return FALSE;
        } else {
            if ({
                let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                if s as gsize <= length {
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
                    b"../original/glib/gfileutils.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1201 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"(gsize) s <= length\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            contents = contents.offset(s as isize);
            length = length.wrapping_sub(s as gsize);
        }
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    if do_fsync != 0 && fsync(fd) != 0 as ::core::ffi::c_int {
        let mut saved_errno_0: ::core::ffi::c_int = *__errno_location();
        if !err.is_null() {
            safe_c2rust_set_file_error(
                err,
                dest_file,
                glib_gettext(
                    b"Failed to write file \xE2\x80\x9C%s\xE2\x80\x9D: fsync() failed: %s\0"
                        as *const u8 as *const gchar,
                ),
                saved_errno_0,
            );
        }
        close(fd);
        return FALSE;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    if g_close(fd as gint, err) == 0 {
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_contents(
    mut filename: *const gchar,
    mut contents: *const gchar,
    mut length: gssize,
    mut error: *mut *mut GError,
) -> gboolean {
    return safe_c2rust_g_file_set_contents_full(
        filename,
        contents,
        length,
        (G_FILE_SET_CONTENTS_CONSISTENT as ::core::ffi::c_int
            | G_FILE_SET_CONTENTS_ONLY_EXISTING as ::core::ffi::c_int)
            as GFileSetContentsFlags,
        0o666 as ::core::ffi::c_int,
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_set_contents_full(
    mut filename: *const gchar,
    mut contents: *const gchar,
    mut length: gssize,
    mut flags: GFileSetContentsFlags,
    mut mode: ::core::ffi::c_int,
    mut error: *mut *mut GError,
) -> gboolean {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !filename.is_null() {
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
            b"filename != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
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
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !contents.is_null() || length == 0 as gssize {
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
            b"contents != NULL || length == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if length >= -(1 as ::core::ffi::c_int) as gssize {
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
            b"length >= -1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if length < 0 as gssize {
        length = strlen(contents as *const ::core::ffi::c_char) as gssize;
    }
    if flags as ::core::ffi::c_uint
        & G_FILE_SET_CONTENTS_CONSISTENT as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        let mut tmp_filename: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut rename_error: *mut GError = ::core::ptr::null_mut::<GError>();
        let mut retval: gboolean = 0;
        let mut fd: ::core::ffi::c_int = 0;
        let mut do_fsync: gboolean = 0;
        tmp_filename = g_strdup_printf(b"%s.XXXXXX\0" as *const u8 as *const gchar, filename);
        *__errno_location() = 0 as ::core::ffi::c_int;
        fd = safe_c2rust_g_mkstemp_full(tmp_filename, O_RDWR | O_BINARY | O_CLOEXEC, mode as gint)
            as ::core::ffi::c_int;
        if fd == -(1 as ::core::ffi::c_int) {
            let mut saved_errno: ::core::ffi::c_int = *__errno_location();
            if !error.is_null() {
                safe_c2rust_set_file_error(
                    error,
                    tmp_filename,
                    glib_gettext(
                        b"Failed to create file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                            as *const gchar,
                    ),
                    saved_errno,
                );
            }
            retval = FALSE as gboolean;
        } else {
            do_fsync = safe_c2rust_fd_should_be_fsynced(fd, filename, flags);
            if safe_c2rust_write_to_file(
                contents,
                length as gsize,
                safe_c2rust_g_steal_fd(&raw mut fd),
                tmp_filename,
                do_fsync,
                error,
            ) == 0
            {
                g_unlink(tmp_filename);
                retval = FALSE as gboolean;
            } else if safe_c2rust_rename_file(
                tmp_filename,
                filename as *const ::core::ffi::c_char,
                do_fsync,
                &raw mut rename_error,
            ) == 0
            {
                g_unlink(tmp_filename);
                g_propagate_error(error, rename_error);
                retval = FALSE as gboolean;
            } else {
                retval = TRUE as gboolean;
            }
        }
        g_free(tmp_filename as gpointer);
        return retval;
    } else {
        let mut direct_fd: ::core::ffi::c_int = 0;
        let mut open_flags: ::core::ffi::c_int = 0;
        let mut do_fsync_0: gboolean = 0;
        open_flags = O_RDWR | O_BINARY | O_CREAT | O_CLOEXEC;
        open_flags |= O_NOFOLLOW;
        *__errno_location() = 0 as ::core::ffi::c_int;
        direct_fd = open(filename as *const ::core::ffi::c_char, open_flags, mode);
        if direct_fd < 0 as ::core::ffi::c_int {
            let mut saved_errno_0: ::core::ffi::c_int = *__errno_location();
            if saved_errno_0 == ELOOP {
                return safe_c2rust_g_file_set_contents_full(
                    filename,
                    contents,
                    length,
                    (flags as ::core::ffi::c_uint
                        | G_FILE_SET_CONTENTS_CONSISTENT as ::core::ffi::c_int
                            as ::core::ffi::c_uint) as GFileSetContentsFlags,
                    mode,
                    error,
                );
            }
            if !error.is_null() {
                safe_c2rust_set_file_error(
                    error,
                    filename,
                    glib_gettext(
                        b"Failed to open file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                            as *const gchar,
                    ),
                    saved_errno_0,
                );
            }
            return FALSE;
        }
        do_fsync_0 = safe_c2rust_fd_should_be_fsynced(direct_fd, filename, flags);
        if safe_c2rust_truncate_file(
            direct_fd,
            0 as off_t,
            filename as *const ::core::ffi::c_char,
            error,
        ) == 0
        {
            return FALSE;
        }
        if safe_c2rust_write_to_file(
            contents,
            length as gsize,
            safe_c2rust_g_steal_fd(&raw mut direct_fd),
            filename,
            do_fsync_0,
            error,
        ) == 0
        {
            return FALSE;
        }
        return TRUE;
    };
}
unsafe extern "C" fn safe_c2rust_get_tmp_file(
    mut tmpl: *mut gchar,
    mut f: GTmpFileCallback,
    mut flags: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> gint {
    let mut XXXXXX: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut count: ::core::ffi::c_int = 0;
    let mut fd: ::core::ffi::c_int = 0;
    static mut safe_c2rust_letters: [::core::ffi::c_char; 37] = unsafe {
        ::core::mem::transmute::<[u8; 37], [::core::ffi::c_char; 37]>(
            *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789\0",
        )
    };
    let mut value: guint64 = 0;
    let mut now_us: guint64 = 0;
    static mut safe_c2rust_counter: guint = 0 as guint;
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !tmpl.is_null() {
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
            b"tmpl != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    XXXXXX = g_strrstr(tmpl, b"XXXXXX\0" as *const u8 as *const gchar) as *mut ::core::ffi::c_char;
    if XXXXXX.is_null()
        || strncmp(
            XXXXXX,
            b"XXXXXX\0" as *const u8 as *const ::core::ffi::c_char,
            6 as size_t,
        ) != 0
    {
        *__errno_location() = EINVAL;
        return -(1 as gint);
    }
    now_us = g_get_real_time() as guint64;
    let fresh0 = safe_c2rust_counter;
    safe_c2rust_counter = safe_c2rust_counter.wrapping_add(1);
    value = (now_us.wrapping_rem(G_USEC_PER_SEC as guint64)
        ^ now_us.wrapping_div(G_USEC_PER_SEC as guint64))
    .wrapping_add(fresh0 as guint64);
    count = 0 as ::core::ffi::c_int;
    while count < 100 as ::core::ffi::c_int {
        let mut v: guint64 = value;
        *XXXXXX.offset(0 as ::core::ffi::c_int as isize) =
            safe_c2rust_letters[v.wrapping_rem(safe_c2rust_NLETTERS as guint64) as usize];
        v = v.wrapping_div(safe_c2rust_NLETTERS as guint64);
        *XXXXXX.offset(1 as ::core::ffi::c_int as isize) =
            safe_c2rust_letters[v.wrapping_rem(safe_c2rust_NLETTERS as guint64) as usize];
        v = v.wrapping_div(safe_c2rust_NLETTERS as guint64);
        *XXXXXX.offset(2 as ::core::ffi::c_int as isize) =
            safe_c2rust_letters[v.wrapping_rem(safe_c2rust_NLETTERS as guint64) as usize];
        v = v.wrapping_div(safe_c2rust_NLETTERS as guint64);
        *XXXXXX.offset(3 as ::core::ffi::c_int as isize) =
            safe_c2rust_letters[v.wrapping_rem(safe_c2rust_NLETTERS as guint64) as usize];
        v = v.wrapping_div(safe_c2rust_NLETTERS as guint64);
        *XXXXXX.offset(4 as ::core::ffi::c_int as isize) =
            safe_c2rust_letters[v.wrapping_rem(safe_c2rust_NLETTERS as guint64) as usize];
        v = v.wrapping_div(safe_c2rust_NLETTERS as guint64);
        *XXXXXX.offset(5 as ::core::ffi::c_int as isize) =
            safe_c2rust_letters[v.wrapping_rem(safe_c2rust_NLETTERS as guint64) as usize];
        fd = f.expect("non-null function pointer")(tmpl, flags as gint, mode as gint)
            as ::core::ffi::c_int;
        if fd >= 0 as ::core::ffi::c_int {
            return fd as gint;
        } else if *__errno_location() != EEXIST {
            return -(1 as gint);
        }
        value = value.wrapping_add(7777 as guint64);
        count += 1;
    }
    *__errno_location() = EEXIST;
    return -(1 as gint);
}
unsafe extern "C" fn safe_c2rust_wrap_g_mkdir(
    mut filename: *const gchar,
    mut flags: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> gint {
    return mkdir(filename as *const ::core::ffi::c_char, mode as __mode_t) as gint;
}
unsafe extern "C" fn safe_c2rust_wrap_g_open(
    mut filename: *const gchar,
    mut flags: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> gint {
    return open(filename as *const ::core::ffi::c_char, flags, mode) as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mkdtemp_full(
    mut tmpl: *mut gchar,
    mut mode: gint,
) -> *mut gchar {
    if safe_c2rust_get_tmp_file(
        tmpl,
        Some(
            safe_c2rust_wrap_g_mkdir
                as unsafe extern "C" fn(
                    *const gchar,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> gint,
        ),
        0 as ::core::ffi::c_int,
        mode as ::core::ffi::c_int,
    ) == -(1 as ::core::ffi::c_int)
    {
        return ::core::ptr::null_mut::<gchar>();
    } else {
        return tmpl;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mkdtemp(mut tmpl: *mut gchar) -> *mut gchar {
    return safe_c2rust_g_mkdtemp_full(tmpl, 0o700 as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mkstemp_full(
    mut tmpl: *mut gchar,
    mut flags: gint,
    mut mode: gint,
) -> gint {
    return safe_c2rust_get_tmp_file(
        tmpl,
        Some(
            safe_c2rust_wrap_g_open
                as unsafe extern "C" fn(
                    *const gchar,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> gint,
        ),
        flags as ::core::ffi::c_int | O_CREAT | O_EXCL,
        mode as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mkstemp(mut tmpl: *mut gchar) -> gint {
    return safe_c2rust_g_mkstemp_full(tmpl, O_RDWR | O_BINARY | O_CLOEXEC, 0o600 as gint);
}
unsafe extern "C" fn safe_c2rust_g_get_tmp_name(
    mut tmpl: *const gchar,
    mut name_used: *mut *mut gchar,
    mut f: GTmpFileCallback,
    mut flags: gint,
    mut mode: gint,
    mut error: *mut *mut GError,
) -> gint {
    let mut retval: ::core::ffi::c_int = 0;
    let mut tmpdir: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut sep: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut fulltemplate: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut slash: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if tmpl.is_null() {
        tmpl = b".XXXXXX\0" as *const u8 as *const ::core::ffi::c_char as *const gchar;
    }
    slash = strchr(tmpl as *const ::core::ffi::c_char, G_DIR_SEPARATOR);
    if !slash.is_null() {
        let mut display_tmpl: *mut gchar = g_filename_display_name(tmpl);
        let mut c: [::core::ffi::c_char; 2] = [0; 2];
        c[0 as ::core::ffi::c_int as usize] = *slash;
        c[1 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
        g_set_error(
            error,
            safe_c2rust_g_file_error_quark(),
            G_FILE_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Template \xE2\x80\x9C%s\xE2\x80\x9D invalid, should not contain a \xE2\x80\x9C%s\xE2\x80\x9D\0"
                    as *const u8 as *const gchar,
            ),
            display_tmpl,
            &raw mut c as *mut ::core::ffi::c_char,
        );
        g_free(display_tmpl as gpointer);
        return -(1 as gint);
    }
    if strstr(
        tmpl as *const ::core::ffi::c_char,
        b"XXXXXX\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
    {
        let mut display_tmpl_0: *mut gchar = g_filename_display_name(tmpl);
        g_set_error(
            error,
            safe_c2rust_g_file_error_quark(),
            G_FILE_ERROR_FAILED as ::core::ffi::c_int as gint,
            glib_gettext(
                b"Template \xE2\x80\x9C%s\xE2\x80\x9D doesn\xE2\x80\x99t contain XXXXXX\0"
                    as *const u8 as *const gchar,
            ),
            display_tmpl_0,
        );
        g_free(display_tmpl_0 as gpointer);
        return -(1 as gint);
    }
    tmpdir = g_get_tmp_dir() as *const ::core::ffi::c_char;
    if *tmpdir.offset(strlen(tmpdir).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        == G_DIR_SEPARATOR
    {
        sep = b"\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        sep = G_DIR_SEPARATOR_S.as_ptr();
    }
    fulltemplate = g_strconcat(tmpdir as *const gchar, sep, tmpl, NULL) as *mut ::core::ffi::c_char;
    retval = safe_c2rust_get_tmp_file(
        fulltemplate as *mut gchar,
        f,
        flags as ::core::ffi::c_int,
        mode as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if retval == -(1 as ::core::ffi::c_int) {
        let mut saved_errno: ::core::ffi::c_int = *__errno_location();
        if !error.is_null() {
            safe_c2rust_set_file_error(
                error,
                fulltemplate,
                glib_gettext(
                    b"Failed to create file \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                        as *const gchar,
                ),
                saved_errno,
            );
        }
        g_free(fulltemplate as gpointer);
        return -(1 as gint);
    }
    *name_used = fulltemplate as *mut gchar;
    return retval as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_open_tmp(
    mut tmpl: *const gchar,
    mut name_used: *mut *mut gchar,
    mut error: *mut *mut GError,
) -> gint {
    let mut fulltemplate: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut result: gint = 0;
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    result = safe_c2rust_g_get_tmp_name(
        tmpl,
        &raw mut fulltemplate,
        Some(
            safe_c2rust_wrap_g_open
                as unsafe extern "C" fn(
                    *const gchar,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> gint,
        ),
        O_CREAT | O_EXCL | O_RDWR | O_BINARY | O_CLOEXEC,
        0o600 as gint,
        error,
    );
    if result != -(1 as ::core::ffi::c_int) {
        if !name_used.is_null() {
            *name_used = fulltemplate;
        } else {
            g_free(fulltemplate as gpointer);
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dir_make_tmp(
    mut tmpl: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut fulltemplate: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if safe_c2rust_g_get_tmp_name(
        tmpl,
        &raw mut fulltemplate,
        Some(
            safe_c2rust_wrap_g_mkdir
                as unsafe extern "C" fn(
                    *const gchar,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> gint,
        ),
        0 as gint,
        0o700 as gint,
        error,
    ) == -(1 as ::core::ffi::c_int)
    {
        return ::core::ptr::null_mut::<gchar>();
    } else {
        return fulltemplate;
    };
}
unsafe extern "C" fn safe_c2rust_g_build_path_va(
    mut separator: *const gchar,
    mut first_element: *const gchar,
    mut args: *mut ::core::ffi::VaList,
    mut str_array: *mut *mut gchar,
) -> *mut gchar {
    let mut result: *mut GString = ::core::ptr::null_mut::<GString>();
    let mut separator_len: gint = strlen(separator as *const ::core::ffi::c_char) as gint;
    let mut is_first: gboolean = TRUE;
    let mut have_leading: gboolean = FALSE;
    let mut single_element: *const gchar = ::core::ptr::null::<gchar>();
    let mut next_element: *const gchar = ::core::ptr::null::<gchar>();
    let mut last_trailing: *const gchar = ::core::ptr::null::<gchar>();
    let mut i: gint = 0 as gint;
    result = g_string_new(::core::ptr::null::<gchar>());
    if !str_array.is_null() {
        let fresh1 = i;
        i = i + 1;
        next_element = *str_array.offset(fresh1 as isize);
    } else {
        next_element = first_element;
    }
    while FALSE == 0 {
        let mut element: *const gchar = ::core::ptr::null::<gchar>();
        let mut start: *const gchar = ::core::ptr::null::<gchar>();
        let mut end: *const gchar = ::core::ptr::null::<gchar>();
        if next_element.is_null() {
            break;
        }
        element = next_element;
        if !str_array.is_null() {
            let fresh2 = i;
            i = i + 1;
            next_element = *str_array.offset(fresh2 as isize);
        } else {
            next_element = (*args).arg::<*mut gchar>();
        }
        if *element == 0 {
            continue;
        }
        start = element;
        if separator_len != 0 {
            while strncmp(
                start as *const ::core::ffi::c_char,
                separator as *const ::core::ffi::c_char,
                separator_len as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                start = start.offset(separator_len as isize);
            }
        }
        end = start.offset(strlen(start as *const ::core::ffi::c_char) as isize);
        if separator_len != 0 {
            while end >= start.offset(separator_len as isize)
                && strncmp(
                    end.offset(-(separator_len as isize)),
                    separator as *const ::core::ffi::c_char,
                    separator_len as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                end = end.offset(-(separator_len as isize));
            }
            last_trailing = end;
            while last_trailing >= element.offset(separator_len as isize)
                && strncmp(
                    last_trailing.offset(-(separator_len as isize)),
                    separator as *const ::core::ffi::c_char,
                    separator_len as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                last_trailing = last_trailing.offset(-(separator_len as isize));
            }
            if have_leading == 0 {
                if last_trailing <= start {
                    single_element = element;
                }
                safe_c2rust_g_string_append_len_inline(
                    result,
                    element as *const ::core::ffi::c_char,
                    start.offset_from(element) as gssize,
                );
                have_leading = TRUE as gboolean;
            } else {
                single_element = ::core::ptr::null::<gchar>();
            }
        }
        if end == start {
            continue;
        }
        if is_first == 0 {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char = separator as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        result,
                        __val,
                        if ({
                            let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_22 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_22 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_22
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
                    result,
                    separator as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        }
        safe_c2rust_g_string_append_len_inline(
            result,
            start as *const ::core::ffi::c_char,
            end.offset_from(start) as gssize,
        );
        is_first = FALSE as gboolean;
    }
    if !single_element.is_null() {
        if 0 != 0 {
            if 0 as ::core::ffi::c_int == 0 {
                g_string_free(result, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
            } else {
                g_string_free_and_steal(result);
            };
        } else {
            g_string_free(result, (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int);
        };
        return safe_c2rust_g_strdup_inline(single_element as *const ::core::ffi::c_char)
            as *mut gchar;
    } else {
        if !last_trailing.is_null() {
            if 0 != 0 {
                ({
                    let __val: *const ::core::ffi::c_char =
                        last_trailing as *const ::core::ffi::c_char;
                    safe_c2rust_g_string_append_len_inline(
                        result,
                        __val,
                        if ({
                            let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
                            if !__val.is_null() {
                                _g_boolean_var_23 = 1 as ::core::ffi::c_int;
                            } else {
                                _g_boolean_var_23 = 0 as ::core::ffi::c_int;
                            }
                            _g_boolean_var_23
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
                    result,
                    last_trailing as *const ::core::ffi::c_char,
                    -(1 as ::core::ffi::c_int) as gssize,
                );
            };
        }
        return if 0 != 0 {
            if 0 as ::core::ffi::c_int != 0 {
                g_string_free(result, 0 as gboolean)
            } else {
                g_string_free_and_steal(result)
            }
        } else {
            g_string_free(result, 0 as gboolean)
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_build_pathv(
    mut separator: *const gchar,
    mut args: *mut *mut gchar,
) -> *mut gchar {
    if args.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    return safe_c2rust_g_build_path_va(
        separator,
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<::core::ffi::VaList>(),
        args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_build_path(
    mut separator: *const gchar,
    mut first_element: *const gchar,
    mut args: ...
) -> *mut gchar {
    let mut str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut args_0: ::core::ffi::VaList;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !separator.is_null() {
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
            b"separator != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    args_0 = args.clone();
    str = safe_c2rust_g_build_path_va(
        separator,
        first_element,
        &raw mut args_0,
        ::core::ptr::null_mut::<*mut gchar>(),
    );
    return str;
}
unsafe extern "C" fn safe_c2rust_g_build_filename_va(
    mut first_argument: *const gchar,
    mut args: *mut ::core::ffi::VaList,
    mut str_array: *mut *mut gchar,
) -> *mut gchar {
    let mut str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    str = safe_c2rust_g_build_path_va(
        G_DIR_SEPARATOR_S.as_ptr() as *const gchar,
        first_argument,
        args,
        str_array,
    );
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_build_filename_valist(
    mut first_element: *const gchar,
    mut args: *mut ::core::ffi::VaList,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !first_element.is_null() {
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
            b"first_element != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return safe_c2rust_g_build_filename_va(
        first_element,
        args,
        ::core::ptr::null_mut::<*mut gchar>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_build_filenamev(mut args: *mut *mut gchar) -> *mut gchar {
    return safe_c2rust_g_build_filename_va(
        ::core::ptr::null::<gchar>(),
        ::core::ptr::null_mut::<::core::ffi::VaList>(),
        args,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_build_filename(
    mut first_element: *const gchar,
    mut args: ...
) -> *mut gchar {
    let mut str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut args_0: ::core::ffi::VaList;
    args_0 = args.clone();
    str = safe_c2rust_g_build_filename_va(
        first_element,
        &raw mut args_0,
        ::core::ptr::null_mut::<*mut gchar>(),
    );
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_file_read_link(
    mut filename: *const gchar,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut size: size_t = 0;
    let mut read_size: gssize = 0;
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !filename.is_null() {
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
            b"filename != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
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
            b"error == NULL || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    size = 256 as size_t;
    buffer = g_malloc(size as gsize) as *mut gchar;
    while FALSE == 0 {
        read_size = readlink(
            filename as *const ::core::ffi::c_char,
            buffer as *mut ::core::ffi::c_char,
            size,
        ) as gssize;
        if read_size < 0 as gssize {
            let mut saved_errno: ::core::ffi::c_int = *__errno_location();
            if !error.is_null() {
                safe_c2rust_set_file_error(
                    error,
                    filename,
                    glib_gettext(
                        b"Failed to read the symbolic link \xE2\x80\x9C%s\xE2\x80\x9D: %s\0"
                            as *const u8 as *const gchar,
                    ),
                    saved_errno,
                );
            }
            g_free(buffer as gpointer);
            return ::core::ptr::null_mut::<gchar>();
        }
        if (read_size as size_t) < size {
            *buffer.offset(read_size as isize) = 0 as gchar;
            return buffer;
        }
        size = size.wrapping_mul(2 as size_t);
        buffer = g_realloc(buffer as gpointer, size as gsize) as *mut gchar;
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_is_absolute(mut file_name: *const gchar) -> gboolean {
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !file_name.is_null() {
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
            b"file_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if *file_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == G_DIR_SEPARATOR
    {
        return TRUE;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_skip_root(mut file_name: *const gchar) -> *const gchar {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !file_name.is_null() {
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
            b"file_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    if *file_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == G_DIR_SEPARATOR
    {
        while *file_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == G_DIR_SEPARATOR
        {
            file_name = file_name.offset(1);
        }
        return file_name as *mut gchar;
    }
    return ::core::ptr::null::<gchar>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_basename(mut file_name: *const gchar) -> *const gchar {
    let mut base: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !file_name.is_null() {
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
            b"file_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    base = strrchr(file_name as *const ::core::ffi::c_char, G_DIR_SEPARATOR) as *mut gchar;
    if !base.is_null() {
        return base.offset(1 as ::core::ffi::c_int as isize);
    }
    return file_name as *mut gchar;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_get_basename(
    mut file_name: *const gchar,
) -> *mut gchar {
    let mut base: gssize = 0;
    let mut last_nonslash: gssize = 0;
    let mut len: gsize = 0;
    let mut retval: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !file_name.is_null() {
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
            b"file_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if *file_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
        return safe_c2rust_g_strdup_inline(b".\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
    }
    last_nonslash =
        strlen(file_name as *const ::core::ffi::c_char).wrapping_sub(1 as size_t) as gssize;
    while last_nonslash >= 0 as gssize
        && *file_name.offset(last_nonslash as isize) as ::core::ffi::c_int == G_DIR_SEPARATOR
    {
        last_nonslash -= 1;
    }
    if last_nonslash == -(1 as ::core::ffi::c_int) as gssize {
        return safe_c2rust_g_strdup_inline(b"/\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
    }
    base = last_nonslash;
    while base >= 0 as gssize
        && !(*file_name.offset(base as isize) as ::core::ffi::c_int == G_DIR_SEPARATOR)
    {
        base -= 1;
    }
    len = (last_nonslash - base) as gsize;
    retval = g_malloc(len.wrapping_add(1 as gsize)) as *mut gchar;
    memcpy(
        retval as *mut ::core::ffi::c_void,
        file_name.offset((base + 1 as gssize) as isize) as *const ::core::ffi::c_void,
        len as size_t,
    );
    *retval.offset(len as isize) = '\0' as i32 as gchar;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_path_get_dirname(mut file_name: *const gchar) -> *mut gchar {
    let mut base: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut len: gsize = 0;
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !file_name.is_null() {
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
            b"file_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    base = strrchr(file_name as *const ::core::ffi::c_char, G_DIR_SEPARATOR) as *mut gchar;
    if base.is_null() {
        return safe_c2rust_g_strdup_inline(b".\0" as *const u8 as *const ::core::ffi::c_char)
            as *mut gchar;
    }
    while base > file_name as *mut gchar && *base as ::core::ffi::c_int == G_DIR_SEPARATOR {
        base = base.offset(-1);
    }
    len = base
        .offset(1 as ::core::ffi::c_int as guint as isize)
        .offset_from(file_name) as ::core::ffi::c_long as gsize;
    base = ({
        let mut __n: gsize = len.wrapping_add(1 as gsize);
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
    memmove(
        base as *mut ::core::ffi::c_void,
        file_name as *const ::core::ffi::c_void,
        len as size_t,
    );
    *base.offset(len as isize) = 0 as gchar;
    return base;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_canonicalize_filename(
    mut filename: *const gchar,
    mut relative_to: *const gchar,
) -> *mut gchar {
    let mut canon: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut input: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut output: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut after_root: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut output_start: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if relative_to.is_null() || safe_c2rust_g_path_is_absolute(relative_to) != 0 {
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
            b"relative_to == NULL || g_path_is_absolute (relative_to)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if safe_c2rust_g_path_is_absolute(filename) == 0 {
        let mut cwd_allocated: *mut gchar = ::core::ptr::null_mut::<gchar>();
        let mut cwd: *const gchar = ::core::ptr::null::<gchar>();
        if !relative_to.is_null() {
            cwd = relative_to;
        } else {
            cwd_allocated = safe_c2rust_g_get_current_dir();
            cwd = cwd_allocated;
        }
        canon = safe_c2rust_g_build_filename(cwd, filename, NULL);
        g_free(cwd_allocated as gpointer);
    } else {
        canon = safe_c2rust_g_strdup_inline(filename as *const ::core::ffi::c_char) as *mut gchar;
    }
    after_root = safe_c2rust_g_path_skip_root(canon) as *mut ::core::ffi::c_char as *mut gchar;
    if after_root.is_null() {
        g_free(canon as gpointer);
        return safe_c2rust_g_build_filename(
            G_DIR_SEPARATOR_S.as_ptr() as *const gchar,
            filename,
            NULL,
        );
    }
    output = after_root.offset(-(1 as ::core::ffi::c_int as isize));
    while output >= canon && *output as ::core::ffi::c_int == G_DIR_SEPARATOR {
        *output = G_DIR_SEPARATOR as gchar;
        output = output.offset(-1);
    }
    output = output.offset(1);
    if *output as ::core::ffi::c_int == G_DIR_SEPARATOR {
        output = output.offset(1);
    }
    if after_root.offset_from(output) as ::core::ffi::c_long == 1 as ::core::ffi::c_long {
        output = output.offset(1);
    }
    input = after_root;
    output_start = output;
    while *input != 0 {
        if ({
            let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
            if input > canon
                && *input.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == '/' as i32
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gfileutils.c\0" as *const u8 as *const ::core::ffi::c_char,
                2854 as ::core::ffi::c_int,
                G_STRFUNC,
                b"input > canon && G_IS_DIR_SEPARATOR (input[-1])\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if ({
            let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
            if output > canon
                && *output.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == '/' as i32
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
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gfileutils.c\0" as *const u8 as *const ::core::ffi::c_char,
                2855 as ::core::ffi::c_int,
                G_STRFUNC,
                b"output > canon && G_IS_DIR_SEPARATOR (output[-1])\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if ({
            let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
            if input >= output {
                _g_boolean_var_36 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_36 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_36
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gfileutils.c\0" as *const u8 as *const ::core::ffi::c_char,
                2856 as ::core::ffi::c_int,
                G_STRFUNC,
                b"input >= output\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        while *input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == G_DIR_SEPARATOR
        {
            input = input.offset(1);
        }
        if *input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32
            && (*input.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                || *input.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == G_DIR_SEPARATOR)
        {
            if *input.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                break;
            }
            input = input.offset(2 as ::core::ffi::c_int as isize);
        } else if *input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '.' as i32
            && *input.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32
            && (*input.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                || *input.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == G_DIR_SEPARATOR)
        {
            if output > output_start {
                loop {
                    output = output.offset(-1);
                    if !(!(*output.offset(-(1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == G_DIR_SEPARATOR)
                        && output > output_start)
                    {
                        break;
                    }
                }
            }
            if *input.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                break;
            }
            input = input.offset(3 as ::core::ffi::c_int as isize);
        } else {
            while *input as ::core::ffi::c_int != 0
                && !(*input as ::core::ffi::c_int == G_DIR_SEPARATOR)
            {
                let fresh4 = input;
                input = input.offset(1);
                let fresh5 = output;
                output = output.offset(1);
                *fresh5 = *fresh4;
            }
            if *input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                break;
            }
            input = input.offset(1);
            let fresh6 = output;
            output = output.offset(1);
            *fresh6 = G_DIR_SEPARATOR as gchar;
        }
    }
    if output > output_start
        && *output.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == G_DIR_SEPARATOR
    {
        output = output.offset(-1);
    }
    *output = '\0' as i32 as gchar;
    return canon;
}
pub const G_PATH_LENGTH: ::core::ffi::c_int = PATH_MAX;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_get_current_dir() -> *mut gchar {
    let mut pwd: *const gchar = ::core::ptr::null::<gchar>();
    let mut buffer: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut dir: *mut gchar = ::core::ptr::null_mut::<gchar>();
    static mut safe_c2rust_buffer_size: gsize = 0 as gsize;
    let mut pwdbuf: stat = stat {
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
    let mut dotbuf: stat = stat {
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
    pwd = g_getenv(b"PWD\0" as *const u8 as *const gchar);
    if !pwd.is_null()
        && stat(
            b".\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut dotbuf,
        ) == 0 as ::core::ffi::c_int
        && stat(pwd as *const ::core::ffi::c_char, &raw mut pwdbuf) == 0 as ::core::ffi::c_int
        && dotbuf.st_dev == pwdbuf.st_dev
        && dotbuf.st_ino == pwdbuf.st_ino
    {
        return safe_c2rust_g_strdup_inline(pwd as *const ::core::ffi::c_char) as *mut gchar;
    }
    if safe_c2rust_buffer_size == 0 as gsize {
        safe_c2rust_buffer_size = (if G_PATH_LENGTH == -(1 as ::core::ffi::c_int) {
            2048 as ::core::ffi::c_int
        } else {
            G_PATH_LENGTH
        }) as gsize;
    }
    while safe_c2rust_buffer_size < G_MAXSIZE.wrapping_div(2 as ::core::ffi::c_ulong) {
        g_free(buffer as gpointer);
        buffer = ({
            let mut __n: gsize = safe_c2rust_buffer_size;
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
        *buffer = 0 as gchar;
        dir = getcwd(
            buffer as *mut ::core::ffi::c_char,
            safe_c2rust_buffer_size as size_t,
        ) as *mut gchar;
        if !dir.is_null() || *__errno_location() != ERANGE {
            break;
        }
        safe_c2rust_buffer_size = safe_c2rust_buffer_size.wrapping_mul(2 as gsize);
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if dir.is_null()
            || strnlen(dir, safe_c2rust_buffer_size as size_t) < safe_c2rust_buffer_size as size_t
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
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gfileutils.c\0" as *const u8 as *const ::core::ffi::c_char,
            2988 as ::core::ffi::c_int,
            G_STRFUNC,
            b"dir == NULL || strnlen (dir, buffer_size) < buffer_size\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if dir.is_null() || *buffer == 0 {
        if ({
            let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
            if safe_c2rust_buffer_size >= 2 as gsize {
                _g_boolean_var_38 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_38 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_38
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gfileutils.c\0" as *const u8 as *const ::core::ffi::c_char,
                2993 as ::core::ffi::c_int,
                G_STRFUNC,
                b"buffer_size >= 2\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        *buffer.offset(0 as ::core::ffi::c_int as isize) = G_DIR_SEPARATOR as gchar;
        *buffer.offset(1 as ::core::ffi::c_int as isize) = 0 as gchar;
    }
    dir = safe_c2rust_g_strdup_inline(buffer) as *mut gchar;
    g_free(buffer as gpointer);
    return dir;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_file_test\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
unsafe extern "C" fn run_static_initializers() {
    safe_c2rust_NLETTERS = (::core::mem::size_of::<[::core::ffi::c_char; 37]>() as usize)
        .wrapping_sub(1 as usize) as ::core::ffi::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
