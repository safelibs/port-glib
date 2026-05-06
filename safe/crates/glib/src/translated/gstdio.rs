extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn chmod(__file: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn mkdir(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn creat(__file: *const ::core::ffi::c_char, __mode: mode_t) -> ::core::ffi::c_int;
    fn access(__name: *const ::core::ffi::c_char, __type: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn chdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn rmdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn utime(
        __file: *const ::core::ffi::c_char,
        __file_times: *const utimbuf,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn g_set_error_literal(
        err: *mut *mut GError,
        domain: GQuark,
        code: gint,
        message: *const gchar,
    );
    fn g_file_error_quark() -> GQuark;
    fn g_file_error_from_errno(err_no: gint) -> GFileError;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn remove(__filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn freopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
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
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type mode_t = __mode_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
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
pub type GStatBuf = stat;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EBADF: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_access(
    mut filename: *const gchar,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return access(filename as *const ::core::ffi::c_char, mode);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_chmod(
    mut filename: *const gchar,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return chmod(filename as *const ::core::ffi::c_char, mode as __mode_t);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_open(
    mut filename: *const gchar,
    mut flags: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    loop {
        fd = open(filename as *const ::core::ffi::c_char, flags, mode);
        if !(({
            let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
            if fd == -(1 as ::core::ffi::c_int) && *__errno_location() == 4 as ::core::ffi::c_int {
                _g_boolean_var_8 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_8 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_8
        }) as ::core::ffi::c_long
            != 0)
        {
            break;
        }
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_creat(
    mut filename: *const gchar,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return creat(filename as *const ::core::ffi::c_char, mode as mode_t);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rename(
    mut oldfilename: *const gchar,
    mut newfilename: *const gchar,
) -> ::core::ffi::c_int {
    return rename(
        oldfilename as *const ::core::ffi::c_char,
        newfilename as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mkdir(
    mut filename: *const gchar,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return mkdir(filename as *const ::core::ffi::c_char, mode as __mode_t);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_chdir(mut path: *const gchar) -> ::core::ffi::c_int {
    return chdir(path as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_stat(
    mut filename: *const gchar,
    mut buf: *mut GStatBuf,
) -> ::core::ffi::c_int {
    return stat(filename as *const ::core::ffi::c_char, buf as *mut stat);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_lstat(
    mut filename: *const gchar,
    mut buf: *mut GStatBuf,
) -> ::core::ffi::c_int {
    return lstat(filename as *const ::core::ffi::c_char, buf as *mut stat);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_unlink(mut filename: *const gchar) -> ::core::ffi::c_int {
    return unlink(filename as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_remove(mut filename: *const gchar) -> ::core::ffi::c_int {
    return remove(filename as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_rmdir(mut filename: *const gchar) -> ::core::ffi::c_int {
    return rmdir(filename as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_fopen(
    mut filename: *const gchar,
    mut mode: *const gchar,
) -> *mut FILE {
    return fopen(
        filename as *const ::core::ffi::c_char,
        mode as *const ::core::ffi::c_char,
    ) as *mut FILE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_freopen(
    mut filename: *const gchar,
    mut mode: *const gchar,
    mut stream: *mut FILE,
) -> *mut FILE {
    return freopen(
        filename as *const ::core::ffi::c_char,
        mode as *const ::core::ffi::c_char,
        stream,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_fsync(mut fd: gint) -> gint {
    let mut retval: ::core::ffi::c_int = 0;
    loop {
        retval = fsync(fd as ::core::ffi::c_int);
        if !(({
            let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
            if retval < 0 as ::core::ffi::c_int && *__errno_location() == 4 as ::core::ffi::c_int {
                _g_boolean_var_9 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_9 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_9
        }) as ::core::ffi::c_long
            != 0)
        {
            break;
        }
    }
    return retval as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_utime(
    mut filename: *const gchar,
    mut utb: *mut utimbuf,
) -> ::core::ffi::c_int {
    return utime(filename as *const ::core::ffi::c_char, utb);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_close(
    mut fd: gint,
    mut error: *mut *mut GError,
) -> gboolean {
    let mut res: ::core::ffi::c_int = 0;
    res = close(fd as ::core::ffi::c_int);
    if res == -(1 as ::core::ffi::c_int) {
        let mut errsv: ::core::ffi::c_int = *__errno_location();
        if errsv == EINTR {
            return TRUE;
        }
        if !error.is_null() {
            g_set_error_literal(
                error,
                g_file_error_quark(),
                g_file_error_from_errno(errsv as gint) as gint,
                g_strerror(errsv as gint),
            );
        }
        if errsv == EBADF {
            if fd >= 0 as ::core::ffi::c_int {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"g_close(fd:%d) failed with EBADF. The tracking of file descriptors got messed up\0"
                        as *const u8 as *const gchar,
                    fd,
                );
            } else {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_CRITICAL,
                    b"g_close(fd:%d) failed with EBADF. This is not a valid file descriptor\0"
                        as *const u8 as *const gchar,
                    fd,
                );
            }
        }
        *__errno_location() = errsv;
        return FALSE;
    }
    return TRUE;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
