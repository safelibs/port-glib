extern "C" {
    fn g_list_free(list: *mut GList);
    fn g_list_free_1(list: *mut GList);
    fn g_list_append(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_insert_before(list: *mut GList, sibling: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_insert_before_link(
        list: *mut GList,
        sibling: *mut GList,
        link_: *mut GList,
    ) -> *mut GList;
    fn g_list_remove_link(list: *mut GList, llink: *mut GList) -> *mut GList;
    fn g_list_reverse(list: *mut GList) -> *mut GList;
    fn g_list_find(list: *mut GList, data: gconstpointer) -> *mut GList;
    fn g_list_find_custom(list: *mut GList, data: gconstpointer, func: GCompareFunc) -> *mut GList;
    fn g_list_position(list: *mut GList, llink: *mut GList) -> gint;
    fn g_list_index(list: *mut GList, data: gconstpointer) -> gint;
    fn g_list_last(list: *mut GList) -> *mut GList;
    fn g_list_sort_with_data(
        list: *mut GList,
        compare_func: GCompareDataFunc,
        user_data: gpointer,
    ) -> *mut GList;
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
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
}
pub type size_t = usize;
pub type gsize = ::core::ffi::c_ulong;
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
pub struct _GQueue {
    pub head: *mut GList,
    pub tail: *mut GList,
    pub length: guint,
}
pub type GQueue = _GQueue;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_new() -> *mut GQueue {
    return ({
        let mut __s: gsize = ::core::mem::size_of::<GQueue>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GQueue;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_free(mut queue: *mut GQueue) {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_list_free((*queue).head);
    g_slice_free1(::core::mem::size_of::<GQueue>() as gsize, queue as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_free_full(
    mut queue: *mut GQueue,
    mut free_func: GDestroyNotify,
) {
    safe_c2rust_g_queue_foreach(
        queue,
        ::core::mem::transmute::<GDestroyNotify, GFunc>(free_func),
        NULL,
    );
    safe_c2rust_g_queue_free(queue);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_init(mut queue: *mut GQueue) {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*queue).tail = ::core::ptr::null_mut::<GList>();
    (*queue).head = (*queue).tail;
    (*queue).length = 0 as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_clear(mut queue: *mut GQueue) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_list_free((*queue).head);
    safe_c2rust_g_queue_init(queue);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_clear_full(
    mut queue: *mut GQueue,
    mut free_func: GDestroyNotify,
) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if free_func.is_some() {
        safe_c2rust_g_queue_foreach(
            queue,
            ::core::mem::transmute::<GDestroyNotify, GFunc>(free_func),
            NULL,
        );
    }
    safe_c2rust_g_queue_clear(queue);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_is_empty(mut queue: *mut GQueue) -> gboolean {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    return ((*queue).head == NULL as *mut GList) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_get_length(mut queue: *mut GQueue) -> guint {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    return (*queue).length;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_reverse(mut queue: *mut GQueue) {
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*queue).tail = (*queue).head;
    (*queue).head = g_list_reverse((*queue).head);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_copy(mut queue: *mut GQueue) -> *mut GQueue {
    let mut result: *mut GQueue = ::core::ptr::null_mut::<GQueue>();
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GQueue>();
    }
    result = safe_c2rust_g_queue_new();
    list = (*queue).head;
    while !list.is_null() {
        safe_c2rust_g_queue_push_tail(result, (*list).data);
        list = (*list).next;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_foreach(
    mut queue: *mut GQueue,
    mut func: GFunc,
    mut user_data: gpointer,
) {
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    list = (*queue).head;
    while !list.is_null() {
        let mut next: *mut GList = (*list).next;
        func.expect("non-null function pointer")((*list).data, user_data);
        list = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_find(
    mut queue: *mut GQueue,
    mut data: gconstpointer,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    return g_list_find((*queue).head, data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_find_custom(
    mut queue: *mut GQueue,
    mut data: gconstpointer,
    mut func: GCompareFunc,
) -> *mut GList {
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    return g_list_find_custom((*queue).head, data, func);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_sort(
    mut queue: *mut GQueue,
    mut compare_func: GCompareDataFunc,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if compare_func.is_some() {
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"compare_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*queue).head = g_list_sort_with_data((*queue).head, compare_func, user_data);
    (*queue).tail = g_list_last((*queue).head);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_push_head(mut queue: *mut GQueue, mut data: gpointer) {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_23 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_23 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_23
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*queue).head = g_list_prepend((*queue).head, data);
    if (*queue).tail.is_null() {
        (*queue).tail = (*queue).head;
    }
    (*queue).length = (*queue).length.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_push_nth(
    mut queue: *mut GQueue,
    mut data: gpointer,
    mut n: gint,
) {
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if n < 0 as ::core::ffi::c_int || n as guint >= (*queue).length {
        safe_c2rust_g_queue_push_tail(queue, data);
        return;
    }
    safe_c2rust_g_queue_insert_before(
        queue,
        safe_c2rust_g_queue_peek_nth_link(queue, n as guint),
        data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_push_head_link(
    mut queue: *mut GQueue,
    mut link: *mut GList,
) {
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !link.is_null() {
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
            b"link != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if (*link).prev.is_null() {
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
            b"link->prev == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if (*link).next.is_null() {
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
            b"link->next == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*link).next = (*queue).head;
    if !(*queue).head.is_null() {
        (*(*queue).head).prev = link;
    } else {
        (*queue).tail = link;
    }
    (*queue).head = link;
    (*queue).length = (*queue).length.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_push_tail(mut queue: *mut GQueue, mut data: gpointer) {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*queue).tail = g_list_append((*queue).tail, data);
    if !(*(*queue).tail).next.is_null() {
        (*queue).tail = (*(*queue).tail).next;
    } else {
        (*queue).head = (*queue).tail;
    }
    (*queue).length = (*queue).length.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_push_tail_link(
    mut queue: *mut GQueue,
    mut link: *mut GList,
) {
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !queue.is_null() {
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
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !link.is_null() {
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
            b"link != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if (*link).prev.is_null() {
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
            b"link->prev == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if (*link).next.is_null() {
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
            b"link->next == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*link).prev = (*queue).tail;
    if !(*queue).tail.is_null() {
        (*(*queue).tail).next = link;
    } else {
        (*queue).head = link;
    }
    (*queue).tail = link;
    (*queue).length = (*queue).length.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_push_nth_link(
    mut queue: *mut GQueue,
    mut n: gint,
    mut link_: *mut GList,
) {
    let mut next: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut prev: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_34 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_34 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_34
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !link_.is_null() {
            _g_boolean_var_35 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_35 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_35
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"link_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if n < 0 as ::core::ffi::c_int || n as guint >= (*queue).length {
        safe_c2rust_g_queue_push_tail_link(queue, link_);
        return;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !(*queue).head.is_null() {
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
            b"../original/glib/gqueue.c\0" as *const u8 as *const ::core::ffi::c_char,
            480 as ::core::ffi::c_int,
            G_STRFUNC,
            b"queue->head\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !(*queue).tail.is_null() {
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
            b"../original/glib/gqueue.c\0" as *const u8 as *const ::core::ffi::c_char,
            481 as ::core::ffi::c_int,
            G_STRFUNC,
            b"queue->tail\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    next = safe_c2rust_g_queue_peek_nth_link(queue, n as guint);
    prev = (*next).prev;
    if !prev.is_null() {
        (*prev).next = link_;
    }
    (*next).prev = link_;
    (*link_).next = next;
    (*link_).prev = prev;
    if !(*(*queue).head).prev.is_null() {
        (*queue).head = (*(*queue).head).prev;
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if (*(*queue).tail).next.is_null() {
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
            b"../original/glib/gqueue.c\0" as *const u8 as *const ::core::ffi::c_char,
            499 as ::core::ffi::c_int,
            G_STRFUNC,
            b"queue->tail->next == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*queue).length = (*queue).length.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_pop_head(mut queue: *mut GQueue) -> gpointer {
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_39 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_39 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_39
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if !(*queue).head.is_null() {
        let mut node: *mut GList = (*queue).head;
        let mut data: gpointer = (*node).data;
        (*queue).head = (*node).next;
        if !(*queue).head.is_null() {
            (*(*queue).head).prev = ::core::ptr::null_mut::<GList>();
        } else {
            (*queue).tail = ::core::ptr::null_mut::<GList>();
        }
        g_list_free_1(node);
        (*queue).length = (*queue).length.wrapping_sub(1);
        return data;
    }
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_pop_head_link(mut queue: *mut GQueue) -> *mut GList {
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_40 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_40 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_40
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if !(*queue).head.is_null() {
        let mut node: *mut GList = (*queue).head;
        (*queue).head = (*node).next;
        if !(*queue).head.is_null() {
            (*(*queue).head).prev = ::core::ptr::null_mut::<GList>();
            (*node).next = ::core::ptr::null_mut::<GList>();
        } else {
            (*queue).tail = ::core::ptr::null_mut::<GList>();
        }
        (*queue).length = (*queue).length.wrapping_sub(1);
        return node;
    }
    return ::core::ptr::null_mut::<GList>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_peek_head_link(mut queue: *mut GQueue) -> *mut GList {
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_41 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_41 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_41
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    return (*queue).head;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_peek_tail_link(mut queue: *mut GQueue) -> *mut GList {
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_42 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_42 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_42
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    return (*queue).tail;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_pop_tail(mut queue: *mut GQueue) -> gpointer {
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_43 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_43 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_43
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if !(*queue).tail.is_null() {
        let mut node: *mut GList = (*queue).tail;
        let mut data: gpointer = (*node).data;
        (*queue).tail = (*node).prev;
        if !(*queue).tail.is_null() {
            (*(*queue).tail).next = ::core::ptr::null_mut::<GList>();
        } else {
            (*queue).head = ::core::ptr::null_mut::<GList>();
        }
        (*queue).length = (*queue).length.wrapping_sub(1);
        g_list_free_1(node);
        return data;
    }
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_pop_nth(
    mut queue: *mut GQueue,
    mut n: guint,
) -> gpointer {
    let mut nth_link: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut result: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_44 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_44 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_44
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if n >= (*queue).length {
        return NULL;
    }
    nth_link = safe_c2rust_g_queue_peek_nth_link(queue, n);
    result = (*nth_link).data;
    safe_c2rust_g_queue_delete_link(queue, nth_link);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_pop_tail_link(mut queue: *mut GQueue) -> *mut GList {
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_45 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_45 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_45
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if !(*queue).tail.is_null() {
        let mut node: *mut GList = (*queue).tail;
        (*queue).tail = (*node).prev;
        if !(*queue).tail.is_null() {
            (*(*queue).tail).next = ::core::ptr::null_mut::<GList>();
            (*node).prev = ::core::ptr::null_mut::<GList>();
        } else {
            (*queue).head = ::core::ptr::null_mut::<GList>();
        }
        (*queue).length = (*queue).length.wrapping_sub(1);
        return node;
    }
    return ::core::ptr::null_mut::<GList>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_pop_nth_link(
    mut queue: *mut GQueue,
    mut n: guint,
) -> *mut GList {
    let mut link: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_46 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_46 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_46
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if n >= (*queue).length {
        return ::core::ptr::null_mut::<GList>();
    }
    link = safe_c2rust_g_queue_peek_nth_link(queue, n);
    safe_c2rust_g_queue_unlink(queue, link);
    return link;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_peek_nth_link(
    mut queue: *mut GQueue,
    mut n: guint,
) -> *mut GList {
    let mut link: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut i: guint = 0;
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_47 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_47 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_47
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GList>();
    }
    if n >= (*queue).length {
        return ::core::ptr::null_mut::<GList>();
    }
    if n > (*queue).length.wrapping_div(2 as guint) {
        n = (*queue).length.wrapping_sub(n).wrapping_sub(1 as guint);
        link = (*queue).tail;
        i = 0 as guint;
        while i < n {
            link = (*link).prev;
            i = i.wrapping_add(1);
        }
    } else {
        link = (*queue).head;
        i = 0 as guint;
        while i < n {
            link = (*link).next;
            i = i.wrapping_add(1);
        }
    }
    return link;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_link_index(
    mut queue: *mut GQueue,
    mut link_: *mut GList,
) -> gint {
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_48 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_48 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_48
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    return g_list_position((*queue).head, link_);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_unlink(mut queue: *mut GQueue, mut link_: *mut GList) {
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_49 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_49 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_49
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if !link_.is_null() {
            _g_boolean_var_50 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_50 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_50
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"link_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if link_ == (*queue).tail {
        (*queue).tail = (*(*queue).tail).prev;
    }
    (*queue).head = g_list_remove_link((*queue).head, link_);
    (*queue).length = (*queue).length.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_delete_link(
    mut queue: *mut GQueue,
    mut link_: *mut GList,
) {
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_51 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_51 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_51
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if !link_.is_null() {
            _g_boolean_var_52 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_52 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_52
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"link_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_queue_unlink(queue, link_);
    g_list_free(link_);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_peek_head(mut queue: *mut GQueue) -> gpointer {
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_53 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_53 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_53
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return if !(*queue).head.is_null() {
        (*(*queue).head).data
    } else {
        NULL
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_peek_tail(mut queue: *mut GQueue) -> gpointer {
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_54 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_54 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_54
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return if !(*queue).tail.is_null() {
        (*(*queue).tail).data
    } else {
        NULL
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_peek_nth(
    mut queue: *mut GQueue,
    mut n: guint,
) -> gpointer {
    let mut link: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_55 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_55 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_55
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    link = safe_c2rust_g_queue_peek_nth_link(queue, n);
    if !link.is_null() {
        return (*link).data;
    }
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_index(
    mut queue: *mut GQueue,
    mut data: gconstpointer,
) -> gint {
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_56 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_56 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_56
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    return g_list_index((*queue).head, data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_remove(
    mut queue: *mut GQueue,
    mut data: gconstpointer,
) -> gboolean {
    let mut link: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_57 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_57 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_57
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    link = g_list_find((*queue).head, data);
    if !link.is_null() {
        safe_c2rust_g_queue_delete_link(queue, link);
    }
    return (link != NULL as *mut GList) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_remove_all(
    mut queue: *mut GQueue,
    mut data: gconstpointer,
) -> guint {
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut old_length: guint = 0;
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_58 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_58 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_58
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    old_length = (*queue).length;
    list = (*queue).head;
    while !list.is_null() {
        let mut next: *mut GList = (*list).next;
        if (*list).data == data as gpointer {
            safe_c2rust_g_queue_delete_link(queue, list);
        }
        list = next;
    }
    return old_length.wrapping_sub((*queue).length);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_insert_before(
    mut queue: *mut GQueue,
    mut sibling: *mut GList,
    mut data: gpointer,
) {
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_59 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_59 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_59
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if sibling.is_null() {
        safe_c2rust_g_queue_push_tail(queue, data);
    } else {
        (*queue).head = g_list_insert_before((*queue).head, sibling, data);
        (*queue).length = (*queue).length.wrapping_add(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_insert_before_link(
    mut queue: *mut GQueue,
    mut sibling: *mut GList,
    mut link_: *mut GList,
) {
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_60 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_60 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_60
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if !link_.is_null() {
            _g_boolean_var_61 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_61 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_61
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"link_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if (*link_).prev.is_null() {
            _g_boolean_var_62 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_62 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_62
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"link_->prev == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if (*link_).next.is_null() {
            _g_boolean_var_63 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_63 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_63
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"link_->next == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if sibling.is_null() {
            _g_boolean_var_64 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_64 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_64
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_queue_push_tail_link(queue, link_);
    } else {
        (*queue).head = g_list_insert_before_link((*queue).head, sibling, link_);
        (*queue).length = (*queue).length.wrapping_add(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_insert_after(
    mut queue: *mut GQueue,
    mut sibling: *mut GList,
    mut data: gpointer,
) {
    if ({
        let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_65 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_65 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_65
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if sibling.is_null() {
        safe_c2rust_g_queue_push_head(queue, data);
    } else {
        safe_c2rust_g_queue_insert_before(queue, (*sibling).next, data);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_insert_after_link(
    mut queue: *mut GQueue,
    mut sibling: *mut GList,
    mut link_: *mut GList,
) {
    if ({
        let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_66 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_66 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_66
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_67: ::core::ffi::c_int = 0;
        if !link_.is_null() {
            _g_boolean_var_67 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_67 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_67
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"link_ != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_68: ::core::ffi::c_int = 0;
        if (*link_).prev.is_null() {
            _g_boolean_var_68 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_68 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_68
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"link_->prev == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_69: ::core::ffi::c_int = 0;
        if (*link_).next.is_null() {
            _g_boolean_var_69 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_69 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_69
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"link_->next == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_70: ::core::ffi::c_int = 0;
        if sibling.is_null() {
            _g_boolean_var_70 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_70 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_70
    }) as ::core::ffi::c_long
        != 0
    {
        safe_c2rust_g_queue_push_head_link(queue, link_);
    } else {
        safe_c2rust_g_queue_insert_before_link(queue, (*sibling).next, link_);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_queue_insert_sorted(
    mut queue: *mut GQueue,
    mut data: gpointer,
    mut func: GCompareDataFunc,
    mut user_data: gpointer,
) {
    let mut list: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_71: ::core::ffi::c_int = 0;
        if !queue.is_null() {
            _g_boolean_var_71 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_71 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_71
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"queue != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    list = (*queue).head;
    while !list.is_null()
        && func.expect("non-null function pointer")(
            (*list).data as gconstpointer,
            data as gconstpointer,
            user_data,
        ) < 0 as ::core::ffi::c_int
    {
        list = (*list).next;
    }
    safe_c2rust_g_queue_insert_before(queue, list, data);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_queue_free\0" as *const u8 as *const ::core::ffi::c_char;
