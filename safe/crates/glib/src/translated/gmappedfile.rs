extern "C" {
    pub type _GBytes;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
    ) -> *mut ::core::ffi::c_void;
    fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_filename_display_name(filename: *const gchar) -> *mut gchar;
    fn g_file_error_quark() -> GQuark;
    fn g_file_error_from_errno(err_no: gint) -> GFileError;
    fn g_bytes_new_with_free_func(
        data: gconstpointer,
        size: gsize,
        free_func: GDestroyNotify,
        user_data: gpointer,
    ) -> *mut GBytes;
    fn g_free(mem: gpointer);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn glib_gettext(str: *const gchar) -> *const gchar;
}
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
pub type off_t = __off64_t;
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
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
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
pub type GBytes = _GBytes;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GMappedFile {
    pub contents: *mut gchar,
    pub length: gsize,
    pub free_func: gpointer,
    pub ref_count: ::core::ffi::c_int,
}
pub type GMappedFile = _GMappedFile;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_PRIVATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_mapped_file_new\0" as *const u8 as *const ::core::ffi::c_char;
pub const MAP_FAILED: *mut ::core::ffi::c_void =
    -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_void;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const _O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_g_mapped_file_destroy(mut file: *mut GMappedFile) {
    if (*file).length != 0 {
        munmap(
            (*file).contents as *mut ::core::ffi::c_void,
            (*file).length as size_t,
        );
    }
    g_slice_free1(
        ::core::mem::size_of::<GMappedFile>() as gsize,
        file as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_mapped_file_new_from_fd(
    mut fd: ::core::ffi::c_int,
    mut writable: gboolean,
    mut filename: *const gchar,
    mut error: *mut *mut GError,
) -> *mut GMappedFile {
    let mut file: *mut GMappedFile = ::core::ptr::null_mut::<GMappedFile>();
    let mut st: stat = stat {
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
    file = ({
        let mut __s: gsize = ::core::mem::size_of::<GMappedFile>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GMappedFile;
    (*file).ref_count = 1 as ::core::ffi::c_int;
    (*file).free_func =
        ::core::mem::transmute::<Option<unsafe extern "C" fn(*mut GMappedFile) -> ()>, gpointer>(
            Some(safe_c2rust_g_mapped_file_destroy as unsafe extern "C" fn(*mut GMappedFile) -> ()),
        );
    if fstat(fd, &raw mut st) == -(1 as ::core::ffi::c_int) {
        let mut save_errno: ::core::ffi::c_int = *__errno_location();
        let mut display_filename: *mut gchar = if !filename.is_null() {
            g_filename_display_name(filename)
        } else {
            ::core::ptr::null_mut::<gchar>()
        };
        g_set_error(
            error,
            g_file_error_quark(),
            g_file_error_from_errno(save_errno as gint) as gint,
            glib_gettext(
                b"Failed to get attributes of file \xE2\x80\x9C%s%s%s%s\xE2\x80\x9D: fstat() failed: %s\0"
                    as *const u8 as *const gchar,
            ),
            if !display_filename.is_null() {
                display_filename as *const gchar
            } else {
                b"fd\0" as *const u8 as *const gchar
            },
            if !display_filename.is_null() {
                b"' \0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            if !display_filename.is_null() {
                display_filename as *const gchar
            } else {
                b"\0" as *const u8 as *const gchar
            },
            if !display_filename.is_null() {
                b"'\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            g_strerror(save_errno as gint),
        );
        g_free(display_filename as gpointer);
    } else {
        if st.st_size == 0 as __off_t && st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t {
            (*file).length = 0 as gsize;
            (*file).contents = ::core::ptr::null_mut::<gchar>();
            return file;
        }
        (*file).contents = MAP_FAILED as *mut gchar;
        if ::core::mem::size_of::<__off_t>() as usize > ::core::mem::size_of::<gsize>() as usize
            && st.st_size > G_MAXSIZE as off_t
        {
            *__errno_location() = EINVAL;
        } else {
            (*file).length = st.st_size as gsize;
            (*file).contents = mmap(
                NULL,
                (*file).length as size_t,
                if writable != 0 {
                    PROT_READ | PROT_WRITE
                } else {
                    PROT_READ
                },
                MAP_PRIVATE,
                fd,
                0 as __off64_t,
            ) as *mut gchar;
        }
        if (*file).contents == MAP_FAILED as *mut gchar {
            let mut save_errno_0: ::core::ffi::c_int = *__errno_location();
            let mut display_filename_0: *mut gchar = if !filename.is_null() {
                g_filename_display_name(filename)
            } else {
                ::core::ptr::null_mut::<gchar>()
            };
            g_set_error(
                error,
                g_file_error_quark(),
                g_file_error_from_errno(save_errno_0 as gint) as gint,
                glib_gettext(
                    b"Failed to map %s%s%s%s: mmap() failed: %s\0" as *const u8 as *const gchar,
                ),
                if !display_filename_0.is_null() {
                    display_filename_0 as *const gchar
                } else {
                    b"fd\0" as *const u8 as *const gchar
                },
                if !display_filename_0.is_null() {
                    b"' \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                if !display_filename_0.is_null() {
                    display_filename_0 as *const gchar
                } else {
                    b"\0" as *const u8 as *const gchar
                },
                if !display_filename_0.is_null() {
                    b"'\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"\0" as *const u8 as *const ::core::ffi::c_char
                },
                g_strerror(save_errno_0 as gint),
            );
            g_free(display_filename_0 as gpointer);
        } else {
            return file;
        }
    }
    g_slice_free1(
        ::core::mem::size_of::<GMappedFile>() as gsize,
        file as gpointer,
    );
    return ::core::ptr::null_mut::<GMappedFile>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mapped_file_new(
    mut filename: *const gchar,
    mut writable: gboolean,
    mut error: *mut *mut GError,
) -> *mut GMappedFile {
    let mut file: *mut GMappedFile = ::core::ptr::null_mut::<GMappedFile>();
    let mut fd: ::core::ffi::c_int = 0;
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
        return ::core::ptr::null_mut::<GMappedFile>();
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if error.is_null() || (*error).is_null() {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"!error || *error == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GMappedFile>();
    }
    fd = open(
        filename as *const ::core::ffi::c_char,
        (if writable != 0 { O_RDWR } else { O_RDONLY }) | _O_BINARY | O_CLOEXEC,
        0 as ::core::ffi::c_int,
    );
    if fd == -(1 as ::core::ffi::c_int) {
        let mut save_errno: ::core::ffi::c_int = *__errno_location();
        let mut display_filename: *mut gchar = g_filename_display_name(filename);
        g_set_error(
            error,
            g_file_error_quark(),
            g_file_error_from_errno(save_errno as gint) as gint,
            glib_gettext(
                b"Failed to open file \xE2\x80\x9C%s\xE2\x80\x9D: open() failed: %s\0" as *const u8
                    as *const gchar,
            ),
            display_filename,
            g_strerror(save_errno as gint),
        );
        g_free(display_filename as gpointer);
        return ::core::ptr::null_mut::<GMappedFile>();
    }
    file = safe_c2rust_mapped_file_new_from_fd(fd, writable, filename, error);
    close(fd);
    return file;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mapped_file_new_from_fd(
    mut fd: gint,
    mut writable: gboolean,
    mut error: *mut *mut GError,
) -> *mut GMappedFile {
    return safe_c2rust_mapped_file_new_from_fd(
        fd as ::core::ffi::c_int,
        writable,
        ::core::ptr::null::<gchar>(),
        error,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mapped_file_get_length(mut file: *mut GMappedFile) -> gsize {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !file.is_null() {
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
            b"file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gsize;
    }
    return (*file).length;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mapped_file_get_contents(
    mut file: *mut GMappedFile,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !file.is_null() {
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
            b"file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    return (*file).contents;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mapped_file_free(mut file: *mut GMappedFile) {
    safe_c2rust_g_mapped_file_unref(file);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mapped_file_ref(
    mut file: *mut GMappedFile,
) -> *mut GMappedFile {
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
        return ::core::ptr::null_mut::<GMappedFile>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*file).ref_count;
        (*file).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*file).ref_count, 1 as ::core::ffi::c_int);
    return file;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mapped_file_unref(mut file: *mut GMappedFile) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !file.is_null() {
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
            b"file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*file).ref_count;
            (*file).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*file).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        safe_c2rust_g_mapped_file_destroy(file);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_mapped_file_get_bytes(
    mut file: *mut GMappedFile,
) -> *mut GBytes {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !file.is_null() {
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
            b"file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GBytes>();
    }
    return g_bytes_new_with_free_func(
        (*file).contents as gconstpointer,
        (*file).length,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GMappedFile) -> ()>,
            GDestroyNotify,
        >(Some(
            safe_c2rust_g_mapped_file_unref as unsafe extern "C" fn(*mut GMappedFile) -> (),
        )),
        safe_c2rust_g_mapped_file_ref(file) as gpointer,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
