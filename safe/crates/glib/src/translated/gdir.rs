extern "C" {
    pub type __dirstream;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn rewinddir(__dirp: *mut DIR);
    fn g_set_error(err: *mut *mut GError, domain: GQuark, code: gint, format: *const gchar, ...);
    fn g_filename_to_utf8(
        opsysstring: *const gchar,
        len: gssize,
        bytes_read: *mut gsize,
        bytes_written: *mut gsize,
        error: *mut *mut GError,
    ) -> *mut gchar;
    fn g_file_error_quark() -> GQuark;
    fn g_file_error_from_errno(err_no: gint) -> GFileError;
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_strerror(errnum: gint) -> *const gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn glib_gettext(str: *const gchar) -> *const gchar;
    fn g_atomic_ref_count_init(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_inc(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_dec(arc: *mut gatomicrefcount) -> gboolean;
}
pub type __ino64_t = ::core::ffi::c_ulong;
pub type __off64_t = ::core::ffi::c_long;
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
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gatomicrefcount = gint;
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
pub struct _GDir {
    pub ref_count: gatomicrefcount,
    pub dirp: *mut DIR,
}
pub type GDir = _GDir;
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
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_dir_new_from_dirp\0" as *const u8 as *const ::core::ffi::c_char;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dir_open_with_errno(
    mut path: *const gchar,
    mut flags: guint,
) -> *mut GDir {
    let mut dirp: *mut DIR = ::core::ptr::null_mut::<DIR>();
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !path.is_null() {
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
            b"path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDir>();
    }
    dirp = opendir(path as *const ::core::ffi::c_char);
    if dirp.is_null() {
        return ::core::ptr::null_mut::<GDir>();
    }
    return safe_c2rust_g_dir_new_from_dirp(dirp as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dir_open(
    mut path: *const gchar,
    mut flags: guint,
    mut error: *mut *mut GError,
) -> *mut GDir {
    let mut saved_errno: gint = 0;
    let mut dir: *mut GDir = ::core::ptr::null_mut::<GDir>();
    dir = safe_c2rust_g_dir_open_with_errno(path, flags);
    if dir.is_null() {
        let mut utf8_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
        saved_errno = *__errno_location() as gint;
        utf8_path = g_filename_to_utf8(
            path,
            -(1 as ::core::ffi::c_int) as gssize,
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<gsize>(),
            ::core::ptr::null_mut::<*mut GError>(),
        );
        g_set_error(
            error,
            g_file_error_quark(),
            g_file_error_from_errno(saved_errno) as gint,
            glib_gettext(
                b"Error opening directory \xE2\x80\x9C%s\xE2\x80\x9D: %s\0" as *const u8
                    as *const gchar,
            ),
            utf8_path,
            g_strerror(saved_errno),
        );
        g_free(utf8_path as gpointer);
    }
    return dir;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dir_new_from_dirp(mut dirp: gpointer) -> *mut GDir {
    let mut dir: *mut GDir = ::core::ptr::null_mut::<GDir>();
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !dirp.is_null() {
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
            b"dirp != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDir>();
    }
    dir = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GDir>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut GDir;
    g_atomic_ref_count_init(&raw mut (*dir).ref_count);
    (*dir).dirp = dirp as *mut DIR;
    return dir;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dir_read_name(mut dir: *mut GDir) -> *const gchar {
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !dir.is_null() {
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
            b"dir != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    entry = readdir((*dir).dirp);
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
        entry = readdir((*dir).dirp);
    }
    if !entry.is_null() {
        return &raw mut (*entry).d_name as *mut ::core::ffi::c_char;
    } else {
        return ::core::ptr::null::<gchar>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dir_rewind(mut dir: *mut GDir) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !dir.is_null() {
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
            b"dir != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    rewinddir((*dir).dirp);
}
unsafe extern "C" fn safe_c2rust_g_dir_actually_close(mut dir: *mut GDir) {
    let mut _pp: *mut *mut DIR = &raw mut (*dir).dirp;
    let mut _ptr: *mut DIR = *_pp;
    *_pp = ::core::ptr::null_mut::<DIR>();
    if !_ptr.is_null() {
        closedir(_ptr as *mut DIR);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dir_close(mut dir: *mut GDir) {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !dir.is_null() {
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
            b"dir != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_dir_actually_close(dir);
    safe_c2rust_g_dir_unref(dir);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dir_ref(mut dir: *mut GDir) -> *mut GDir {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !dir.is_null() {
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
            b"dir != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GDir>();
    }
    g_atomic_ref_count_inc(&raw mut (*dir).ref_count);
    return dir;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dir_unref(mut dir: *mut GDir) {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !dir.is_null() {
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
            b"dir != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if g_atomic_ref_count_dec(&raw mut (*dir).ref_count) != 0 {
        safe_c2rust_g_dir_actually_close(dir);
        g_free(dir as gpointer);
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
