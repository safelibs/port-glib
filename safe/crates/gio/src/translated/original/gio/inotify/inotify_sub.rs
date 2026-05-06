extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_path_get_basename(file_name: *const gchar) -> *mut gchar;
    fn g_path_get_dirname(file_name: *const gchar) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
}
pub type size_t = usize;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gpointer = *mut ::core::ffi::c_void;
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
pub struct inotify_sub {
    pub dirname: *mut gchar,
    pub filename: *mut gchar,
    pub cancelled: gboolean,
    pub user_data: gpointer,
    pub pair_moves: gboolean,
    pub hardlinks: gboolean,
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
static mut safe_c2rust_is_debug_enabled: gboolean = FALSE;
unsafe extern "C" fn safe_c2rust_dup_dirname(mut dirname: *const gchar) -> *mut gchar {
    let mut d_dirname: *mut gchar =
        safe_c2rust_g_strdup_inline(dirname as *const ::core::ffi::c_char) as *mut gchar;
    let mut len: size_t = strlen(d_dirname);
    if len > 1 as size_t
        && *d_dirname.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            == '/' as i32
    {
        *d_dirname.offset(len.wrapping_sub(1 as size_t) as isize) = '\0' as i32 as gchar;
    }
    return d_dirname;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__ih_sub_new(
    mut dirname: *const gchar,
    mut basename: *const gchar,
    mut filename: *const gchar,
    mut user_data: gpointer,
) -> *mut inotify_sub {
    let mut sub: *mut inotify_sub = ::core::ptr::null_mut::<inotify_sub>();
    sub = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<inotify_sub>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut inotify_sub;
    if !filename.is_null() {
        (*sub).dirname = g_path_get_dirname(filename);
        (*sub).filename = g_path_get_basename(filename);
        (*sub).hardlinks = TRUE as gboolean;
    } else {
        (*sub).dirname = safe_c2rust_dup_dirname(dirname);
        (*sub).filename =
            safe_c2rust_g_strdup_inline(basename as *const ::core::ffi::c_char) as *mut gchar;
        (*sub).hardlinks = FALSE as gboolean;
    }
    (*sub).user_data = user_data;
    if safe_c2rust_is_debug_enabled != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"new subscription for %s being setup\n\0" as *const u8 as *const gchar,
            (*sub).dirname,
        );
    }
    return sub;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__ih_sub_free(mut sub: *mut inotify_sub) {
    g_free((*sub).dirname as gpointer);
    g_free((*sub).filename as gpointer);
    g_free(sub as gpointer);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
