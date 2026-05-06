extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_slice_free_chain_with_offset(block_size: gsize, mem_chain: gpointer, next_offset: gsize);
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
}
pub type size_t = usize;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
pub type GCompareDataFunc =
    Option<unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GCopyFunc = Option<unsafe extern "C" fn(gconstpointer, gpointer) -> gpointer>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_alloc() -> *mut GList {
    return ({
        let mut __s: gsize = ::core::mem::size_of::<GList>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GList;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_free(mut list: *mut GList) {
    g_slice_free_chain_with_offset(
        ::core::mem::size_of::<GList>() as gsize,
        list as gpointer,
        8 as ::core::ffi::c_ulong as glong as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_free_1(mut list: *mut GList) {
    g_slice_free1(::core::mem::size_of::<GList>() as gsize, list as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_free_full(
    mut list: *mut GList,
    mut free_func: GDestroyNotify,
) {
    safe_c2rust_g_list_foreach(
        list,
        ::core::mem::transmute::<GDestroyNotify, GFunc>(free_func),
        NULL,
    );
    safe_c2rust_g_list_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_append(
    mut list: *mut GList,
    mut data: gpointer,
) -> *mut GList {
    let mut new_list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut last: *mut GList = ::core::ptr::null_mut::<GList>();
    new_list = g_slice_alloc(::core::mem::size_of::<GList>() as gsize) as *mut GList;
    (*new_list).data = data;
    (*new_list).next = ::core::ptr::null_mut::<GList>();
    if !list.is_null() {
        last = safe_c2rust_g_list_last(list);
        (*last).next = new_list;
        (*new_list).prev = last;
        return list;
    } else {
        (*new_list).prev = ::core::ptr::null_mut::<GList>();
        return new_list;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_prepend(
    mut list: *mut GList,
    mut data: gpointer,
) -> *mut GList {
    let mut new_list: *mut GList = ::core::ptr::null_mut::<GList>();
    new_list = g_slice_alloc(::core::mem::size_of::<GList>() as gsize) as *mut GList;
    (*new_list).data = data;
    (*new_list).next = list;
    if !list.is_null() {
        (*new_list).prev = (*list).prev;
        if !(*list).prev.is_null() {
            (*(*list).prev).next = new_list;
        }
        (*list).prev = new_list;
    } else {
        (*new_list).prev = ::core::ptr::null_mut::<GList>();
    }
    return new_list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_insert(
    mut list: *mut GList,
    mut data: gpointer,
    mut position: gint,
) -> *mut GList {
    let mut new_list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut tmp_list: *mut GList = ::core::ptr::null_mut::<GList>();
    if position < 0 as ::core::ffi::c_int {
        return safe_c2rust_g_list_append(list, data);
    } else if position == 0 as ::core::ffi::c_int {
        return safe_c2rust_g_list_prepend(list, data);
    }
    tmp_list = safe_c2rust_g_list_nth(list, position as guint);
    if tmp_list.is_null() {
        return safe_c2rust_g_list_append(list, data);
    }
    new_list = g_slice_alloc(::core::mem::size_of::<GList>() as gsize) as *mut GList;
    (*new_list).data = data;
    (*new_list).prev = (*tmp_list).prev;
    (*(*tmp_list).prev).next = new_list;
    (*new_list).next = tmp_list;
    (*tmp_list).prev = new_list;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_insert_before_link(
    mut list: *mut GList,
    mut sibling: *mut GList,
    mut link_: *mut GList,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !link_.is_null() {
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
            b"link_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return list;
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if (*link_).prev.is_null() {
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
            b"link_->prev == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return list;
    }
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if (*link_).next.is_null() {
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
            b"link_->next == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return list;
    }
    if list.is_null() {
        if ({
            let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
            if sibling.is_null() {
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
                b"sibling == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return list;
        }
        return link_;
    } else if !sibling.is_null() {
        (*link_).prev = (*sibling).prev;
        (*link_).next = sibling;
        (*sibling).prev = link_;
        if !(*link_).prev.is_null() {
            (*(*link_).prev).next = link_;
            return list;
        } else {
            if ({
                let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
                if sibling == list {
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
                    b"sibling == list\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return link_;
            }
            return link_;
        }
    } else {
        let mut last: *mut GList = ::core::ptr::null_mut::<GList>();
        last = list;
        while !(*last).next.is_null() {
            last = (*last).next;
        }
        (*last).next = link_;
        (*(*last).next).prev = last;
        (*(*last).next).next = ::core::ptr::null_mut::<GList>();
        return list;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_insert_before(
    mut list: *mut GList,
    mut sibling: *mut GList,
    mut data: gpointer,
) -> *mut GList {
    if list.is_null() {
        list = safe_c2rust_g_list_alloc();
        (*list).data = data;
        if ({
            let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
            if sibling.is_null() {
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
                b"sibling == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return list;
        }
        return list;
    } else if !sibling.is_null() {
        let mut node: *mut GList = ::core::ptr::null_mut::<GList>();
        node = g_slice_alloc(::core::mem::size_of::<GList>() as gsize) as *mut GList;
        (*node).data = data;
        (*node).prev = (*sibling).prev;
        (*node).next = sibling;
        (*sibling).prev = node;
        if !(*node).prev.is_null() {
            (*(*node).prev).next = node;
            return list;
        } else {
            if ({
                let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
                if sibling == list {
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
                    b"sibling == list\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return node;
            }
            return node;
        }
    } else {
        let mut last: *mut GList = ::core::ptr::null_mut::<GList>();
        last = list;
        while !(*last).next.is_null() {
            last = (*last).next;
        }
        (*last).next = g_slice_alloc(::core::mem::size_of::<GList>() as gsize) as *mut GList;
        (*(*last).next).data = data;
        (*(*last).next).prev = last;
        (*(*last).next).next = ::core::ptr::null_mut::<GList>();
        return list;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_concat(
    mut list1: *mut GList,
    mut list2: *mut GList,
) -> *mut GList {
    let mut tmp_list: *mut GList = ::core::ptr::null_mut::<GList>();
    if !list2.is_null() {
        tmp_list = safe_c2rust_g_list_last(list1);
        if !tmp_list.is_null() {
            (*tmp_list).next = list2;
        } else {
            list1 = list2;
        }
        (*list2).prev = tmp_list;
    }
    return list1;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_list_remove_link(
    mut list: *mut GList,
    mut link: *mut GList,
) -> *mut GList {
    if link.is_null() {
        return list;
    }
    if !(*link).prev.is_null() {
        if (*(*link).prev).next == link {
            (*(*link).prev).next = (*link).next;
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"corrupted double-linked list detected\0" as *const u8 as *const gchar,
            );
        }
    }
    if !(*link).next.is_null() {
        if (*(*link).next).prev == link {
            (*(*link).next).prev = (*link).prev;
        } else {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"corrupted double-linked list detected\0" as *const u8 as *const gchar,
            );
        }
    }
    if link == list {
        list = (*list).next;
    }
    (*link).next = ::core::ptr::null_mut::<GList>();
    (*link).prev = ::core::ptr::null_mut::<GList>();
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_remove(
    mut list: *mut GList,
    mut data: gconstpointer,
) -> *mut GList {
    let mut tmp: *mut GList = ::core::ptr::null_mut::<GList>();
    tmp = list;
    while !tmp.is_null() {
        if (*tmp).data != data as gpointer {
            tmp = (*tmp).next;
        } else {
            list = safe_c2rust__g_list_remove_link(list, tmp);
            g_slice_free1(::core::mem::size_of::<GList>() as gsize, tmp as gpointer);
            break;
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_remove_all(
    mut list: *mut GList,
    mut data: gconstpointer,
) -> *mut GList {
    let mut tmp: *mut GList = list;
    while !tmp.is_null() {
        if (*tmp).data != data as gpointer {
            tmp = (*tmp).next;
        } else {
            let mut next: *mut GList = (*tmp).next;
            if !(*tmp).prev.is_null() {
                (*(*tmp).prev).next = next;
            } else {
                list = next;
            }
            if !next.is_null() {
                (*next).prev = (*tmp).prev;
            }
            g_slice_free1(::core::mem::size_of::<GList>() as gsize, tmp as gpointer);
            tmp = next;
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_remove_link(
    mut list: *mut GList,
    mut llink: *mut GList,
) -> *mut GList {
    return safe_c2rust__g_list_remove_link(list, llink);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_delete_link(
    mut list: *mut GList,
    mut link_: *mut GList,
) -> *mut GList {
    list = safe_c2rust__g_list_remove_link(list, link_);
    g_slice_free1(::core::mem::size_of::<GList>() as gsize, link_ as gpointer);
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_copy(mut list: *mut GList) -> *mut GList {
    return safe_c2rust_g_list_copy_deep(list, None, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_copy_deep(
    mut list: *mut GList,
    mut func: GCopyFunc,
    mut user_data: gpointer,
) -> *mut GList {
    let mut new_list: *mut GList = ::core::ptr::null_mut::<GList>();
    if !list.is_null() {
        let mut last: *mut GList = ::core::ptr::null_mut::<GList>();
        new_list = g_slice_alloc(::core::mem::size_of::<GList>() as gsize) as *mut GList;
        if func.is_some() {
            (*new_list).data =
                func.expect("non-null function pointer")((*list).data as gconstpointer, user_data);
        } else {
            (*new_list).data = (*list).data;
        }
        (*new_list).prev = ::core::ptr::null_mut::<GList>();
        last = new_list;
        list = (*list).next;
        while !list.is_null() {
            (*last).next = g_slice_alloc(::core::mem::size_of::<GList>() as gsize) as *mut GList;
            (*(*last).next).prev = last;
            last = (*last).next;
            if func.is_some() {
                (*last).data = func.expect("non-null function pointer")(
                    (*list).data as gconstpointer,
                    user_data,
                );
            } else {
                (*last).data = (*list).data;
            }
            list = (*list).next;
        }
        (*last).next = ::core::ptr::null_mut::<GList>();
    }
    return new_list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_reverse(mut list: *mut GList) -> *mut GList {
    let mut last: *mut GList = ::core::ptr::null_mut::<GList>();
    last = ::core::ptr::null_mut::<GList>();
    while !list.is_null() {
        last = list;
        list = (*last).next;
        (*last).next = (*last).prev;
        (*last).prev = list;
    }
    return last;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_nth(mut list: *mut GList, mut n: guint) -> *mut GList {
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 > 0 as guint && !list.is_null()) {
            break;
        }
        list = (*list).next;
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_nth_prev(
    mut list: *mut GList,
    mut n: guint,
) -> *mut GList {
    loop {
        let fresh1 = n;
        n = n.wrapping_sub(1);
        if !(fresh1 > 0 as guint && !list.is_null()) {
            break;
        }
        list = (*list).prev;
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_nth_data(
    mut list: *mut GList,
    mut n: guint,
) -> gpointer {
    loop {
        let fresh2 = n;
        n = n.wrapping_sub(1);
        if !(fresh2 > 0 as guint && !list.is_null()) {
            break;
        }
        list = (*list).next;
    }
    return if !list.is_null() { (*list).data } else { NULL };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_find(
    mut list: *mut GList,
    mut data: gconstpointer,
) -> *mut GList {
    while !list.is_null() {
        if (*list).data == data as gpointer {
            break;
        }
        list = (*list).next;
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_find_custom(
    mut list: *mut GList,
    mut data: gconstpointer,
    mut func: GCompareFunc,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return list;
    }
    while !list.is_null() {
        if func.expect("non-null function pointer")((*list).data as gconstpointer, data) == 0 {
            return list;
        }
        list = (*list).next;
    }
    return ::core::ptr::null_mut::<GList>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_position(
    mut list: *mut GList,
    mut llink: *mut GList,
) -> gint {
    let mut i: gint = 0;
    i = 0 as ::core::ffi::c_int as gint;
    while !list.is_null() {
        if list == llink {
            return i;
        }
        i += 1;
        list = (*list).next;
    }
    return -(1 as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_index(
    mut list: *mut GList,
    mut data: gconstpointer,
) -> gint {
    let mut i: gint = 0;
    i = 0 as ::core::ffi::c_int as gint;
    while !list.is_null() {
        if (*list).data == data as gpointer {
            return i;
        }
        i += 1;
        list = (*list).next;
    }
    return -(1 as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_last(mut list: *mut GList) -> *mut GList {
    if !list.is_null() {
        while !(*list).next.is_null() {
            list = (*list).next;
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_first(mut list: *mut GList) -> *mut GList {
    if !list.is_null() {
        while !(*list).prev.is_null() {
            list = (*list).prev;
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_length(mut list: *mut GList) -> guint {
    let mut length: guint = 0;
    length = 0 as guint;
    while !list.is_null() {
        length = length.wrapping_add(1);
        list = (*list).next;
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_foreach(
    mut list: *mut GList,
    mut func: GFunc,
    mut user_data: gpointer,
) {
    while !list.is_null() {
        let mut next: *mut GList = (*list).next;
        Some(func.expect("non-null function pointer")).expect("non-null function pointer")(
            (*list).data,
            user_data,
        );
        list = next;
    }
}
unsafe extern "C" fn safe_c2rust_g_list_insert_sorted_real(
    mut list: *mut GList,
    mut data: gpointer,
    mut func: GFunc,
    mut user_data: gpointer,
) -> *mut GList {
    let mut tmp_list: *mut GList = list;
    let mut new_list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut cmp: gint = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return list;
    }
    if list.is_null() {
        new_list = ({
            let mut __s: gsize = ::core::mem::size_of::<GList>() as gsize;
            let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            __p = g_slice_alloc(__s);
            memset(
                __p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                __s as size_t,
            );
            __p
        }) as *mut GList;
        (*new_list).data = data;
        return new_list;
    }
    cmp = ::core::mem::transmute::<GFunc, GCompareDataFunc>(func)
        .expect("non-null function pointer")(
        data as gconstpointer,
        (*tmp_list).data as gconstpointer,
        user_data,
    );
    while !(*tmp_list).next.is_null() && cmp > 0 as ::core::ffi::c_int {
        tmp_list = (*tmp_list).next;
        cmp = ::core::mem::transmute::<GFunc, GCompareDataFunc>(func)
            .expect("non-null function pointer")(
            data as gconstpointer,
            (*tmp_list).data as gconstpointer,
            user_data,
        );
    }
    new_list = ({
        let mut __s: gsize = ::core::mem::size_of::<GList>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GList;
    (*new_list).data = data;
    if (*tmp_list).next.is_null() && cmp > 0 as ::core::ffi::c_int {
        (*tmp_list).next = new_list;
        (*new_list).prev = tmp_list;
        return list;
    }
    if !(*tmp_list).prev.is_null() {
        (*(*tmp_list).prev).next = new_list;
        (*new_list).prev = (*tmp_list).prev;
    }
    (*new_list).next = tmp_list;
    (*tmp_list).prev = new_list;
    if tmp_list == list {
        return new_list;
    } else {
        return list;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_insert_sorted(
    mut list: *mut GList,
    mut data: gpointer,
    mut func: GCompareFunc,
) -> *mut GList {
    return safe_c2rust_g_list_insert_sorted_real(
        list,
        data,
        ::core::mem::transmute::<GCompareFunc, GFunc>(func),
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_insert_sorted_with_data(
    mut list: *mut GList,
    mut data: gpointer,
    mut func: GCompareDataFunc,
    mut user_data: gpointer,
) -> *mut GList {
    return safe_c2rust_g_list_insert_sorted_real(
        list,
        data,
        ::core::mem::transmute::<GCompareDataFunc, GFunc>(func),
        user_data,
    );
}
unsafe extern "C" fn safe_c2rust_g_list_sort_merge(
    mut l1: *mut GList,
    mut l2: *mut GList,
    mut compare_func: GFunc,
    mut user_data: gpointer,
) -> *mut GList {
    let mut list: GList = _GList {
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        next: ::core::ptr::null_mut::<GList>(),
        prev: ::core::ptr::null_mut::<GList>(),
    };
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut lprev: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut cmp: gint = 0;
    l = &raw mut list;
    lprev = ::core::ptr::null_mut::<GList>();
    while !l1.is_null() && !l2.is_null() {
        cmp = ::core::mem::transmute::<GFunc, GCompareDataFunc>(compare_func)
            .expect("non-null function pointer")(
            (*l1).data as gconstpointer,
            (*l2).data as gconstpointer,
            user_data,
        );
        if cmp <= 0 as ::core::ffi::c_int {
            (*l).next = l1;
            l1 = (*l1).next;
        } else {
            (*l).next = l2;
            l2 = (*l2).next;
        }
        l = (*l).next;
        (*l).prev = lprev;
        lprev = l;
    }
    (*l).next = if !l1.is_null() { l1 } else { l2 };
    (*(*l).next).prev = l;
    return list.next;
}
unsafe extern "C" fn safe_c2rust_g_list_sort_real(
    mut list: *mut GList,
    mut compare_func: GFunc,
    mut user_data: gpointer,
) -> *mut GList {
    let mut l1: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut l2: *mut GList = ::core::ptr::null_mut::<GList>();
    if list.is_null() {
        return ::core::ptr::null_mut::<GList>();
    }
    if (*list).next.is_null() {
        return list;
    }
    l1 = list;
    l2 = (*list).next;
    loop {
        l2 = (*l2).next;
        if l2.is_null() {
            break;
        }
        l2 = (*l2).next;
        if l2.is_null() {
            break;
        }
        l1 = (*l1).next;
    }
    l2 = (*l1).next;
    (*l1).next = ::core::ptr::null_mut::<GList>();
    return safe_c2rust_g_list_sort_merge(
        safe_c2rust_g_list_sort_real(list, compare_func, user_data),
        safe_c2rust_g_list_sort_real(l2, compare_func, user_data),
        compare_func,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_sort(
    mut list: *mut GList,
    mut compare_func: GCompareFunc,
) -> *mut GList {
    return safe_c2rust_g_list_sort_real(
        list,
        ::core::mem::transmute::<GCompareFunc, GFunc>(compare_func),
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_list_sort_with_data(
    mut list: *mut GList,
    mut compare_func: GCompareDataFunc,
    mut user_data: gpointer,
) -> *mut GList {
    return safe_c2rust_g_list_sort_real(
        list,
        ::core::mem::transmute::<GCompareDataFunc, GFunc>(compare_func),
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_clear_list(
    mut list_ptr: *mut *mut GList,
    mut destroy: GDestroyNotify,
) {
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    list = *list_ptr;
    if !list.is_null() {
        *list_ptr = ::core::ptr::null_mut::<GList>();
        if destroy.is_some() {
            safe_c2rust_g_list_free_full(list, destroy);
        } else {
            safe_c2rust_g_list_free(list);
        }
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_list_insert_sorted_real\0" as *const u8 as *const ::core::ffi::c_char;
