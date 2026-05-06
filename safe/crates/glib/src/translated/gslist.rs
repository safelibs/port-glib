extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_slice_free_chain_with_offset(block_size: gsize, mem_chain: gpointer, next_offset: gsize);
}
pub type size_t = usize;
pub type gsize = ::core::ffi::c_ulong;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
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
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_alloc() -> *mut GSList {
    return ({
        let mut __s: gsize = ::core::mem::size_of::<GSList>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GSList;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_free(mut list: *mut GSList) {
    g_slice_free_chain_with_offset(
        ::core::mem::size_of::<GSList>() as gsize,
        list as gpointer,
        8 as ::core::ffi::c_ulong as glong as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_free_1(mut list: *mut GSList) {
    g_slice_free1(::core::mem::size_of::<GSList>() as gsize, list as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_free_full(
    mut list: *mut GSList,
    mut free_func: GDestroyNotify,
) {
    safe_c2rust_g_slist_foreach(
        list,
        ::core::mem::transmute::<GDestroyNotify, GFunc>(free_func),
        NULL,
    );
    safe_c2rust_g_slist_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_append(
    mut list: *mut GSList,
    mut data: gpointer,
) -> *mut GSList {
    let mut new_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut last: *mut GSList = ::core::ptr::null_mut::<GSList>();
    new_list = g_slice_alloc(::core::mem::size_of::<GSList>() as gsize) as *mut GSList;
    (*new_list).data = data;
    (*new_list).next = ::core::ptr::null_mut::<GSList>();
    if !list.is_null() {
        last = safe_c2rust_g_slist_last(list);
        (*last).next = new_list;
        return list;
    } else {
        return new_list;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_prepend(
    mut list: *mut GSList,
    mut data: gpointer,
) -> *mut GSList {
    let mut new_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    new_list = g_slice_alloc(::core::mem::size_of::<GSList>() as gsize) as *mut GSList;
    (*new_list).data = data;
    (*new_list).next = list;
    return new_list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_insert(
    mut list: *mut GSList,
    mut data: gpointer,
    mut position: gint,
) -> *mut GSList {
    let mut prev_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut tmp_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut new_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if position < 0 as ::core::ffi::c_int {
        return safe_c2rust_g_slist_append(list, data);
    } else if position == 0 as ::core::ffi::c_int {
        return safe_c2rust_g_slist_prepend(list, data);
    }
    new_list = g_slice_alloc(::core::mem::size_of::<GSList>() as gsize) as *mut GSList;
    (*new_list).data = data;
    if list.is_null() {
        (*new_list).next = ::core::ptr::null_mut::<GSList>();
        return new_list;
    }
    prev_list = ::core::ptr::null_mut::<GSList>();
    tmp_list = list;
    loop {
        let fresh0 = position;
        position = position - 1;
        if !(fresh0 > 0 as ::core::ffi::c_int && !tmp_list.is_null()) {
            break;
        }
        prev_list = tmp_list;
        tmp_list = (*tmp_list).next;
    }
    (*new_list).next = (*prev_list).next;
    (*prev_list).next = new_list;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_insert_before(
    mut slist: *mut GSList,
    mut sibling: *mut GSList,
    mut data: gpointer,
) -> *mut GSList {
    if slist.is_null() {
        slist = g_slice_alloc(::core::mem::size_of::<GSList>() as gsize) as *mut GSList;
        (*slist).data = data;
        (*slist).next = ::core::ptr::null_mut::<GSList>();
        if ({
            let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
            if sibling.is_null() {
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
                b"sibling == NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return slist;
        }
        return slist;
    } else {
        let mut node: *mut GSList = ::core::ptr::null_mut::<GSList>();
        let mut last: *mut GSList = ::core::ptr::null_mut::<GSList>();
        node = slist;
        while !node.is_null() {
            if node == sibling {
                break;
            }
            last = node;
            node = (*last).next;
        }
        if last.is_null() {
            node = g_slice_alloc(::core::mem::size_of::<GSList>() as gsize) as *mut GSList;
            (*node).data = data;
            (*node).next = slist;
            return node;
        } else {
            node = g_slice_alloc(::core::mem::size_of::<GSList>() as gsize) as *mut GSList;
            (*node).data = data;
            (*node).next = (*last).next;
            (*last).next = node;
            return slist;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_concat(
    mut list1: *mut GSList,
    mut list2: *mut GSList,
) -> *mut GSList {
    if !list2.is_null() {
        if !list1.is_null() {
            let ref mut fresh1 = (*safe_c2rust_g_slist_last(list1)).next;
            *fresh1 = list2;
        } else {
            list1 = list2;
        }
    }
    return list1;
}
unsafe extern "C" fn safe_c2rust__g_slist_remove_data(
    mut list: *mut GSList,
    mut data: gconstpointer,
    mut all: gboolean,
) -> *mut GSList {
    let mut tmp: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut previous_ptr: *mut *mut GSList = &raw mut list;
    while !(*previous_ptr).is_null() {
        tmp = *previous_ptr;
        if (*tmp).data == data as gpointer {
            *previous_ptr = (*tmp).next;
            safe_c2rust_g_slist_free_1(tmp);
            if all == 0 {
                break;
            }
        } else {
            previous_ptr = &raw mut (*tmp).next;
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_remove(
    mut list: *mut GSList,
    mut data: gconstpointer,
) -> *mut GSList {
    return safe_c2rust__g_slist_remove_data(list, data, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_remove_all(
    mut list: *mut GSList,
    mut data: gconstpointer,
) -> *mut GSList {
    return safe_c2rust__g_slist_remove_data(list, data, TRUE);
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_slist_remove_link(
    mut list: *mut GSList,
    mut link: *mut GSList,
) -> *mut GSList {
    let mut tmp: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut previous_ptr: *mut *mut GSList = &raw mut list;
    while !(*previous_ptr).is_null() {
        tmp = *previous_ptr;
        if tmp == link {
            *previous_ptr = (*tmp).next;
            (*tmp).next = ::core::ptr::null_mut::<GSList>();
            break;
        } else {
            previous_ptr = &raw mut (*tmp).next;
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_remove_link(
    mut list: *mut GSList,
    mut link_: *mut GSList,
) -> *mut GSList {
    return safe_c2rust__g_slist_remove_link(list, link_);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_delete_link(
    mut list: *mut GSList,
    mut link_: *mut GSList,
) -> *mut GSList {
    list = safe_c2rust__g_slist_remove_link(list, link_);
    g_slice_free1(::core::mem::size_of::<GSList>() as gsize, link_ as gpointer);
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_copy(mut list: *mut GSList) -> *mut GSList {
    return safe_c2rust_g_slist_copy_deep(list, None, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_copy_deep(
    mut list: *mut GSList,
    mut func: GCopyFunc,
    mut user_data: gpointer,
) -> *mut GSList {
    let mut new_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if !list.is_null() {
        let mut last: *mut GSList = ::core::ptr::null_mut::<GSList>();
        new_list = g_slice_alloc(::core::mem::size_of::<GSList>() as gsize) as *mut GSList;
        if func.is_some() {
            (*new_list).data =
                func.expect("non-null function pointer")((*list).data as gconstpointer, user_data);
        } else {
            (*new_list).data = (*list).data;
        }
        last = new_list;
        list = (*list).next;
        while !list.is_null() {
            (*last).next = g_slice_alloc(::core::mem::size_of::<GSList>() as gsize) as *mut GSList;
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
        (*last).next = ::core::ptr::null_mut::<GSList>();
    }
    return new_list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_reverse(mut list: *mut GSList) -> *mut GSList {
    let mut prev: *mut GSList = ::core::ptr::null_mut::<GSList>();
    while !list.is_null() {
        let mut next: *mut GSList = (*list).next;
        (*list).next = prev;
        prev = list;
        list = next;
    }
    return prev;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_nth(
    mut list: *mut GSList,
    mut n: guint,
) -> *mut GSList {
    loop {
        let fresh2 = n;
        n = n.wrapping_sub(1);
        if !(fresh2 > 0 as guint && !list.is_null()) {
            break;
        }
        list = (*list).next;
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_nth_data(
    mut list: *mut GSList,
    mut n: guint,
) -> gpointer {
    loop {
        let fresh3 = n;
        n = n.wrapping_sub(1);
        if !(fresh3 > 0 as guint && !list.is_null()) {
            break;
        }
        list = (*list).next;
    }
    return if !list.is_null() { (*list).data } else { NULL };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_find(
    mut list: *mut GSList,
    mut data: gconstpointer,
) -> *mut GSList {
    while !list.is_null() {
        if (*list).data == data as gpointer {
            break;
        }
        list = (*list).next;
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_find_custom(
    mut list: *mut GSList,
    mut data: gconstpointer,
    mut func: GCompareFunc,
) -> *mut GSList {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if func.is_some() {
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
    return ::core::ptr::null_mut::<GSList>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_position(
    mut list: *mut GSList,
    mut llink: *mut GSList,
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
pub unsafe extern "C" fn safe_c2rust_g_slist_index(
    mut list: *mut GSList,
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
pub unsafe extern "C" fn safe_c2rust_g_slist_last(mut list: *mut GSList) -> *mut GSList {
    if !list.is_null() {
        while !(*list).next.is_null() {
            list = (*list).next;
        }
    }
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_length(mut list: *mut GSList) -> guint {
    let mut length: guint = 0;
    length = 0 as guint;
    while !list.is_null() {
        length = length.wrapping_add(1);
        list = (*list).next;
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_foreach(
    mut list: *mut GSList,
    mut func: GFunc,
    mut user_data: gpointer,
) {
    while !list.is_null() {
        let mut next: *mut GSList = (*list).next;
        Some(func.expect("non-null function pointer")).expect("non-null function pointer")(
            (*list).data,
            user_data,
        );
        list = next;
    }
}
unsafe extern "C" fn safe_c2rust_g_slist_insert_sorted_real(
    mut list: *mut GSList,
    mut data: gpointer,
    mut func: GFunc,
    mut user_data: gpointer,
) -> *mut GSList {
    let mut tmp_list: *mut GSList = list;
    let mut prev_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut new_list: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut cmp: gint = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return list;
    }
    if list.is_null() {
        new_list = g_slice_alloc(::core::mem::size_of::<GSList>() as gsize) as *mut GSList;
        (*new_list).data = data;
        (*new_list).next = ::core::ptr::null_mut::<GSList>();
        return new_list;
    }
    cmp = ::core::mem::transmute::<GFunc, GCompareDataFunc>(func)
        .expect("non-null function pointer")(
        data as gconstpointer,
        (*tmp_list).data as gconstpointer,
        user_data,
    );
    while !(*tmp_list).next.is_null() && cmp > 0 as ::core::ffi::c_int {
        prev_list = tmp_list;
        tmp_list = (*tmp_list).next;
        cmp = ::core::mem::transmute::<GFunc, GCompareDataFunc>(func)
            .expect("non-null function pointer")(
            data as gconstpointer,
            (*tmp_list).data as gconstpointer,
            user_data,
        );
    }
    new_list = g_slice_alloc(::core::mem::size_of::<GSList>() as gsize) as *mut GSList;
    (*new_list).data = data;
    if (*tmp_list).next.is_null() && cmp > 0 as ::core::ffi::c_int {
        (*tmp_list).next = new_list;
        (*new_list).next = ::core::ptr::null_mut::<GSList>();
        return list;
    }
    if !prev_list.is_null() {
        (*prev_list).next = new_list;
        (*new_list).next = tmp_list;
        return list;
    } else {
        (*new_list).next = list;
        return new_list;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_insert_sorted(
    mut list: *mut GSList,
    mut data: gpointer,
    mut func: GCompareFunc,
) -> *mut GSList {
    return safe_c2rust_g_slist_insert_sorted_real(
        list,
        data,
        ::core::mem::transmute::<GCompareFunc, GFunc>(func),
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_insert_sorted_with_data(
    mut list: *mut GSList,
    mut data: gpointer,
    mut func: GCompareDataFunc,
    mut user_data: gpointer,
) -> *mut GSList {
    return safe_c2rust_g_slist_insert_sorted_real(
        list,
        data,
        ::core::mem::transmute::<GCompareDataFunc, GFunc>(func),
        user_data,
    );
}
unsafe extern "C" fn safe_c2rust_g_slist_sort_merge(
    mut l1: *mut GSList,
    mut l2: *mut GSList,
    mut compare_func: GFunc,
    mut user_data: gpointer,
) -> *mut GSList {
    let mut list: GSList = _GSList {
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        next: ::core::ptr::null_mut::<GSList>(),
    };
    let mut l: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut cmp: gint = 0;
    l = &raw mut list;
    while !l1.is_null() && !l2.is_null() {
        cmp = ::core::mem::transmute::<GFunc, GCompareDataFunc>(compare_func)
            .expect("non-null function pointer")(
            (*l1).data as gconstpointer,
            (*l2).data as gconstpointer,
            user_data,
        );
        if cmp <= 0 as ::core::ffi::c_int {
            (*l).next = l1;
            l = (*l).next;
            l1 = (*l1).next;
        } else {
            (*l).next = l2;
            l = (*l).next;
            l2 = (*l2).next;
        }
    }
    (*l).next = if !l1.is_null() { l1 } else { l2 };
    return list.next;
}
unsafe extern "C" fn safe_c2rust_g_slist_sort_real(
    mut list: *mut GSList,
    mut compare_func: GFunc,
    mut user_data: gpointer,
) -> *mut GSList {
    let mut l1: *mut GSList = ::core::ptr::null_mut::<GSList>();
    let mut l2: *mut GSList = ::core::ptr::null_mut::<GSList>();
    if list.is_null() {
        return ::core::ptr::null_mut::<GSList>();
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
    (*l1).next = ::core::ptr::null_mut::<GSList>();
    return safe_c2rust_g_slist_sort_merge(
        safe_c2rust_g_slist_sort_real(list, compare_func, user_data),
        safe_c2rust_g_slist_sort_real(l2, compare_func, user_data),
        compare_func,
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_sort(
    mut list: *mut GSList,
    mut compare_func: GCompareFunc,
) -> *mut GSList {
    return safe_c2rust_g_slist_sort_real(
        list,
        ::core::mem::transmute::<GCompareFunc, GFunc>(compare_func),
        NULL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_slist_sort_with_data(
    mut list: *mut GSList,
    mut compare_func: GCompareDataFunc,
    mut user_data: gpointer,
) -> *mut GSList {
    return safe_c2rust_g_slist_sort_real(
        list,
        ::core::mem::transmute::<GCompareDataFunc, GFunc>(compare_func),
        user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_clear_slist(
    mut slist_ptr: *mut *mut GSList,
    mut destroy: GDestroyNotify,
) {
    let mut slist: *mut GSList = ::core::ptr::null_mut::<GSList>();
    slist = *slist_ptr;
    if !slist.is_null() {
        *slist_ptr = ::core::ptr::null_mut::<GSList>();
        if destroy.is_some() {
            safe_c2rust_g_slist_free_full(slist, destroy);
        } else {
            safe_c2rust_g_slist_free(slist);
        }
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_slist_insert_sorted_real\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
