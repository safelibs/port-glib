extern "C" {
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_remove(list: *mut GList, data: gconstpointer) -> *mut GList;
    fn g_list_delete_link(list: *mut GList, link_: *mut GList) -> *mut GList;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_utf8_get_char_validated(p: *const gchar, max_len: gssize) -> gunichar;
    fn g_utf8_find_prev_char(str: *const gchar, p: *const gchar) -> *mut gchar;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCompletion {
    pub items: *mut GList,
    pub func: GCompletionFunc,
    pub prefix: *mut gchar,
    pub cache: *mut GList,
    pub strncmp_func: GCompletionStrncmpFunc,
}
pub type GCompletionStrncmpFunc =
    Option<unsafe extern "C" fn(*const gchar, *const gchar, gsize) -> gint>;
pub type GCompletionFunc = Option<unsafe extern "C" fn(gpointer) -> *mut gchar>;
pub type GCompletion = _GCompletion;
pub type gunichar = guint32;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn safe_c2rust_strncmp_bridge(left: *const ::core::ffi::c_char, right: *const ::core::ffi::c_char, n: gsize) -> ::core::ffi::c_int {
    strncmp(left, right, n as size_t)
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_completion_new(
    mut func: GCompletionFunc,
) -> *mut GCompletion {
    let mut gcomp: *mut GCompletion = ::core::ptr::null_mut::<GCompletion>();
    gcomp = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GCompletion>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GCompletion;
    (*gcomp).items = ::core::ptr::null_mut::<GList>();
    (*gcomp).cache = ::core::ptr::null_mut::<GList>();
    (*gcomp).prefix = ::core::ptr::null_mut::<gchar>();
    (*gcomp).func = func;
    (*gcomp).strncmp_func = Some(safe_c2rust_strncmp_bridge);
    return gcomp;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_completion_add_items(
    mut cmp: *mut GCompletion,
    mut items: *mut GList,
) {
    let mut it: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !cmp.is_null() {
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
            b"cmp != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*cmp).cache.is_null() {
        g_list_free((*cmp).cache);
        (*cmp).cache = ::core::ptr::null_mut::<GList>();
    }
    if !(*cmp).prefix.is_null() {
        g_free((*cmp).prefix as gpointer);
        (*cmp).prefix = ::core::ptr::null_mut::<gchar>();
    }
    it = items;
    while !it.is_null() {
        (*cmp).items = g_list_prepend((*cmp).items, (*it).data);
        it = (*it).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_completion_remove_items(
    mut cmp: *mut GCompletion,
    mut items: *mut GList,
) {
    let mut it: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !cmp.is_null() {
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
            b"cmp != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    it = items;
    while !(*cmp).items.is_null() && !it.is_null() {
        (*cmp).items = g_list_remove((*cmp).items, (*it).data as gconstpointer);
        it = (*it).next;
    }
    it = items;
    while !(*cmp).cache.is_null() && !it.is_null() {
        (*cmp).cache = g_list_remove((*cmp).cache, (*it).data as gconstpointer);
        it = (*it).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_completion_clear_items(mut cmp: *mut GCompletion) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !cmp.is_null() {
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
            b"cmp != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_list_free((*cmp).items);
    (*cmp).items = ::core::ptr::null_mut::<GList>();
    g_list_free((*cmp).cache);
    (*cmp).cache = ::core::ptr::null_mut::<GList>();
    g_free((*cmp).prefix as gpointer);
    (*cmp).prefix = ::core::ptr::null_mut::<gchar>();
}
unsafe extern "C" fn safe_c2rust_completion_check_cache(
    mut cmp: *mut GCompletion,
    mut new_prefix: *mut *mut gchar,
) {
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut len: gsize = 0;
    let mut i: gsize = 0;
    let mut plen: gsize = 0;
    let mut postfix: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut s: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if new_prefix.is_null() {
        return;
    }
    if (*cmp).cache.is_null() {
        *new_prefix = ::core::ptr::null_mut::<gchar>();
        return;
    }
    len = strlen((*cmp).prefix) as gsize;
    list = (*cmp).cache;
    s = if (*cmp).func.is_some() {
        (*cmp).func.expect("non-null function pointer")((*list).data)
    } else {
        (*list).data as *mut gchar
    };
    postfix = s.offset(len as isize);
    plen = strlen(postfix) as gsize;
    list = (*list).next;
    while !list.is_null() && plen != 0 {
        s = if (*cmp).func.is_some() {
            (*cmp).func.expect("non-null function pointer")((*list).data)
        } else {
            (*list).data as *mut gchar
        };
        s = s.offset(len as isize);
        i = 0 as gsize;
        while i < plen {
            if *postfix.offset(i as isize) as ::core::ffi::c_int
                != *s.offset(i as isize) as ::core::ffi::c_int
            {
                break;
            }
            i = i.wrapping_add(1);
        }
        plen = i;
        list = (*list).next;
    }
    *new_prefix = ({
        let mut __n: gsize = len.wrapping_add(plen).wrapping_add(1 as gsize);
        let mut __s: gsize = ::core::mem::size_of::<gchar>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut gchar;
    strncpy(*new_prefix, (*cmp).prefix, len as size_t);
    strncpy((*new_prefix).offset(len as isize), postfix, plen as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_completion_complete_utf8(
    mut cmp: *mut GCompletion,
    mut prefix: *const gchar,
    mut new_prefix: *mut *mut gchar,
) -> *mut GList {
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut p: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut q: *mut gchar = ::core::ptr::null_mut::<gchar>();
    list = safe_c2rust_g_completion_complete(cmp, prefix, new_prefix);
    if !new_prefix.is_null() && !(*new_prefix).is_null() {
        p = (*new_prefix).offset(strlen(*new_prefix) as isize);
        q = g_utf8_find_prev_char(*new_prefix, p);
        match g_utf8_get_char_validated(q, p.offset_from(q) as gssize) {
            4294967294 | 4294967295 => {
                *q = 0 as gchar;
            }
            _ => {}
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_completion_complete(
    mut cmp: *mut GCompletion,
    mut prefix: *const gchar,
    mut new_prefix: *mut *mut gchar,
) -> *mut GList {
    let mut plen: gsize = 0;
    let mut len: gsize = 0;
    let mut done: gboolean = FALSE;
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !cmp.is_null() {
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
            b"cmp != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !prefix.is_null() {
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
            b"prefix != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    len = strlen(prefix as *const ::core::ffi::c_char) as gsize;
    if !(*cmp).prefix.is_null() && !(*cmp).cache.is_null() {
        plen = strlen((*cmp).prefix) as gsize;
        if plen <= len
            && (*cmp).strncmp_func.expect("non-null function pointer")(prefix, (*cmp).prefix, plen)
                == 0
        {
            list = (*cmp).cache;
            while !list.is_null() {
                let mut next: *mut GList = (*list).next;
                if (*cmp).strncmp_func.expect("non-null function pointer")(
                    prefix,
                    if (*cmp).func.is_some() {
                        (*cmp).func.expect("non-null function pointer")((*list).data)
                    } else {
                        (*list).data as *mut gchar
                    },
                    len,
                ) != 0
                {
                    (*cmp).cache = g_list_delete_link((*cmp).cache, list);
                }
                list = next;
            }
            done = TRUE as gboolean;
        }
    }
    if done == 0 {
        g_list_free((*cmp).cache);
        (*cmp).cache = ::core::ptr::null_mut::<GList>();
        list = (*cmp).items;
        while *prefix as ::core::ffi::c_int != 0 && !list.is_null() {
            if (*cmp).strncmp_func.expect("non-null function pointer")(
                prefix,
                if (*cmp).func.is_some() {
                    (*cmp).func.expect("non-null function pointer")((*list).data)
                } else {
                    (*list).data as *mut gchar
                },
                len,
            ) == 0
            {
                (*cmp).cache = g_list_prepend((*cmp).cache, (*list).data);
            }
            list = (*list).next;
        }
    }
    if !(*cmp).prefix.is_null() {
        g_free((*cmp).prefix as gpointer);
        (*cmp).prefix = ::core::ptr::null_mut::<gchar>();
    }
    if !(*cmp).cache.is_null() {
        (*cmp).prefix =
            safe_c2rust_g_strdup_inline(prefix as *const ::core::ffi::c_char) as *mut gchar;
    }
    safe_c2rust_completion_check_cache(cmp, new_prefix);
    return if *prefix as ::core::ffi::c_int != 0 {
        (*cmp).cache
    } else {
        (*cmp).items
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_completion_free(mut cmp: *mut GCompletion) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !cmp.is_null() {
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
            b"cmp != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_completion_clear_items(cmp);
    g_free(cmp as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_completion_set_compare(
    mut cmp: *mut GCompletion,
    mut strncmp_func: GCompletionStrncmpFunc,
) {
    (*cmp).strncmp_func = strncmp_func;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_completion_add_items\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
