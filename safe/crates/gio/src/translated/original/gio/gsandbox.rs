extern "C" {
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
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_clear_error(err: *mut *mut GError);
    fn g_getenv(variable: *const gchar) -> *const gchar;
    fn g_file_test(filename: *const gchar, test: GFileTest) -> gboolean;
    fn g_file_get_contents(
        filename: *const gchar,
        contents: *mut *mut gchar,
        length: *mut gsize,
        error: *mut *mut GError,
    ) -> gboolean;
    fn g_build_filename(first_element: *const gchar, ...) -> *mut gchar;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
    fn g_strchug(string: *mut gchar) -> *mut gchar;
    fn g_strchomp(string: *mut gchar) -> *mut gchar;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_strcmp0(
        str1: *const ::core::ffi::c_char,
        str2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
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
pub type GFileTest = ::core::ffi::c_uint;
pub const G_FILE_TEST_EXISTS: GFileTest = 16;
pub const G_FILE_TEST_IS_EXECUTABLE: GFileTest = 8;
pub const G_FILE_TEST_IS_DIR: GFileTest = 4;
pub const G_FILE_TEST_IS_SYMLINK: GFileTest = 2;
pub const G_FILE_TEST_IS_REGULAR: GFileTest = 1;
pub type GSandboxType = ::core::ffi::c_uint;
pub const G_SANDBOX_TYPE_SNAP: GSandboxType = 2;
pub const G_SANDBOX_TYPE_FLATPAK: GSandboxType = 1;
pub const G_SANDBOX_TYPE_UNKNOWN: GSandboxType = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
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
pub const SNAP_CONFINEMENT_PREFIX: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"confinement:\0") };
unsafe extern "C" fn safe_c2rust_is_flatpak() -> gboolean {
    let mut flatpak_info: *const ::core::ffi::c_char =
        b"/.flatpak-info\0" as *const u8 as *const ::core::ffi::c_char;
    let mut found: gboolean = 0;
    found = g_file_test(flatpak_info as *const gchar, G_FILE_TEST_EXISTS);
    return found;
}
unsafe extern "C" fn safe_c2rust_get_snap_confinement(
    mut snap_yaml: *const ::core::ffi::c_char,
    mut error: *mut *mut GError,
) -> *mut gchar {
    let mut confinement: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut yaml_contents: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    if g_file_get_contents(
        snap_yaml as *const gchar,
        &raw mut yaml_contents,
        ::core::ptr::null_mut::<gsize>(),
        error,
    ) != 0
    {
        let mut line: *const ::core::ffi::c_char = yaml_contents;
        while !(if 0 != 0 {
            ({
                let __str: *const ::core::ffi::c_char = line;
                let __prefix: *const ::core::ffi::c_char =
                    b"confinement:\0" as *const u8 as *const ::core::ffi::c_char;
                let mut __result: gboolean = FALSE;
                if ({
                    let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
                    if __str.is_null() || __prefix.is_null() {
                        _g_boolean_var_10 = 1 as ::core::ffi::c_int;
                    } else {
                        _g_boolean_var_10 = 0 as ::core::ffi::c_int;
                    }
                    _g_boolean_var_10
                }) as ::core::ffi::c_long
                    != 0
                {
                    __result = g_str_has_prefix(__str as *const gchar, __prefix as *const gchar);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as ::core::ffi::c_int as isize))
                            as size_t;
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
                line as *const gchar,
                b"confinement:\0" as *const u8 as *const gchar,
            )
        } != 0)
        {
            line = strchr(line, '\n' as i32);
            if !line.is_null() {
                line = line.offset(1 as ::core::ffi::c_int as isize);
            }
            if line.is_null() {
                break;
            }
        }
        if !line.is_null() {
            let mut start: *const ::core::ffi::c_char =
                line.offset(strlen(SNAP_CONFINEMENT_PREFIX.as_ptr()) as isize);
            let mut end: *const ::core::ffi::c_char = strchr(start, '\n' as i32);
            confinement = g_strchomp(g_strchug(if !end.is_null() {
                g_strndup(
                    start as *const gchar,
                    end.offset_from(start) as ::core::ffi::c_long as gsize,
                )
            } else {
                safe_c2rust_g_strdup_inline(start) as *mut gchar
            })) as *mut ::core::ffi::c_char;
        }
        g_free(yaml_contents as gpointer);
    }
    return safe_c2rust_g_steal_pointer(&raw mut confinement as gpointer) as *mut gchar;
}
unsafe extern "C" fn safe_c2rust_is_snap() -> gboolean {
    let mut error: *mut GError = ::core::ptr::null_mut::<GError>();
    let mut snap_path: *const gchar = ::core::ptr::null::<gchar>();
    let mut yaml_path: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut confinement: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut result: gboolean = 0;
    snap_path = g_getenv(b"SNAP\0" as *const u8 as *const gchar);
    if snap_path.is_null() {
        return FALSE;
    }
    result = FALSE as gboolean;
    yaml_path = g_build_filename(
        snap_path,
        b"meta\0" as *const u8 as *const ::core::ffi::c_char,
        b"snap.yaml\0" as *const u8 as *const ::core::ffi::c_char,
        NULL_0,
    );
    confinement =
        safe_c2rust_get_snap_confinement(yaml_path, &raw mut error) as *mut ::core::ffi::c_char;
    g_free(yaml_path as gpointer);
    if error.is_null()
        && g_strcmp0(
            confinement,
            b"classic\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        result = TRUE as gboolean;
    }
    g_clear_error(&raw mut error);
    g_free(confinement as gpointer);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_glib_get_sandbox_type() -> GSandboxType {
    if safe_c2rust_is_flatpak() != 0 {
        return G_SANDBOX_TYPE_FLATPAK;
    } else if safe_c2rust_is_snap() != 0 {
        return G_SANDBOX_TYPE_SNAP;
    } else {
        return G_SANDBOX_TYPE_UNKNOWN;
    };
}
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
