extern "C" {
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
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
pub type guint32 = ::core::ffi::c_uint;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareDataFunc =
    Option<unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSequence {
    pub end_node: *mut GSequenceNode,
    pub data_destroy_notify: GDestroyNotify,
    pub access_prohibited: gboolean,
    pub real_sequence: *mut GSequence,
}
pub type GSequence = _GSequence;
pub type GSequenceNode = _GSequenceNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSequenceNode {
    pub n_nodes: gint,
    pub priority: guint32,
    pub parent: *mut GSequenceNode,
    pub left: *mut GSequenceNode,
    pub right: *mut GSequenceNode,
    pub data: gpointer,
}
pub type GSequenceIter = _GSequenceNode;
pub type GSequenceIterCompareFunc =
    Option<unsafe extern "C" fn(*mut GSequenceIter, *mut GSequenceIter, gpointer) -> gint>;
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
pub struct SortInfo {
    pub cmp_func: GCompareDataFunc,
    pub cmp_data: gpointer,
    pub end_node: *mut GSequenceNode,
}
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn safe_c2rust_check_seq_access(mut seq: *mut GSequence) {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if (*seq).access_prohibited != 0 {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Accessing a sequence while it is being sorted or searched is not allowed\0"
                as *const u8 as *const gchar,
        );
    }
}
unsafe extern "C" fn safe_c2rust_get_sequence(mut node: *mut GSequenceNode) -> *mut GSequence {
    return (*safe_c2rust_node_get_last(node)).data as *mut GSequence;
}
unsafe extern "C" fn safe_c2rust_seq_is_end(
    mut seq: *mut GSequence,
    mut iter: *mut GSequenceIter,
) -> gboolean {
    return ((*seq).end_node == iter) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_is_end(mut iter: *mut GSequenceIter) -> gboolean {
    let mut parent: *mut GSequenceIter = (*iter).parent as *mut GSequenceIter;
    if !(*iter).right.is_null() {
        return FALSE;
    }
    if parent.is_null() {
        return TRUE;
    }
    while (*parent).right == iter {
        iter = parent;
        parent = (*iter).parent as *mut GSequenceIter;
        if parent.is_null() {
            return TRUE;
        }
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_iter_compare(
    mut node1: *mut GSequenceIter,
    mut node2: *mut GSequenceIter,
    mut data: gpointer,
) -> gint {
    let mut info: *const SortInfo = data as *const SortInfo;
    let mut retval: gint = 0;
    if node1 == (*info).end_node {
        return 1 as gint;
    }
    if node2 == (*info).end_node {
        return -(1 as gint);
    }
    retval = (*info).cmp_func.expect("non-null function pointer")(
        (*node1).data as gconstpointer,
        (*node2).data as gconstpointer,
        (*info).cmp_data,
    );
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_new(
    mut data_destroy: GDestroyNotify,
) -> *mut GSequence {
    let mut seq: *mut GSequence = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<GSequence>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut GSequence;
    (*seq).data_destroy_notify = data_destroy;
    (*seq).end_node = safe_c2rust_node_new(seq as gpointer);
    (*seq).access_prohibited = FALSE as gboolean;
    (*seq).real_sequence = seq;
    return seq;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_free(mut seq: *mut GSequence) {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !seq.is_null() {
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
            b"seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_check_seq_access(seq);
    safe_c2rust_node_free((*seq).end_node, seq);
    g_free(seq as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_foreach_range(
    mut begin: *mut GSequenceIter,
    mut end: *mut GSequenceIter,
    mut func: GFunc,
    mut user_data: gpointer,
) {
    let mut seq: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    let mut iter: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
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
        return;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !begin.is_null() {
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
            b"begin != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !end.is_null() {
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
            b"end != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    seq = safe_c2rust_get_sequence(begin as *mut GSequenceNode);
    (*seq).access_prohibited = TRUE as gboolean;
    iter = begin;
    while iter != end {
        let mut next: *mut GSequenceIter =
            safe_c2rust_node_get_next(iter as *mut GSequenceNode) as *mut GSequenceIter;
        func.expect("non-null function pointer")((*iter).data, user_data);
        iter = next;
    }
    (*seq).access_prohibited = FALSE as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_foreach(
    mut seq: *mut GSequence,
    mut func: GFunc,
    mut user_data: gpointer,
) {
    let mut begin: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    let mut end: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    safe_c2rust_check_seq_access(seq);
    begin = safe_c2rust_g_sequence_get_begin_iter(seq);
    end = safe_c2rust_g_sequence_get_end_iter(seq);
    safe_c2rust_g_sequence_foreach_range(begin, end, func, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_range_get_midpoint(
    mut begin: *mut GSequenceIter,
    mut end: *mut GSequenceIter,
) -> *mut GSequenceIter {
    let mut begin_pos: ::core::ffi::c_int = 0;
    let mut end_pos: ::core::ffi::c_int = 0;
    let mut mid_pos: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !begin.is_null() {
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
            b"begin != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !end.is_null() {
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
            b"end != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if safe_c2rust_get_sequence(begin as *mut GSequenceNode)
            == safe_c2rust_get_sequence(end as *mut GSequenceNode)
        {
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
            b"get_sequence (begin) == get_sequence (end)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    begin_pos = safe_c2rust_node_get_pos(begin as *mut GSequenceNode) as ::core::ffi::c_int;
    end_pos = safe_c2rust_node_get_pos(end as *mut GSequenceNode) as ::core::ffi::c_int;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if end_pos >= begin_pos {
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
            b"end_pos >= begin_pos\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    mid_pos = begin_pos + (end_pos - begin_pos) / 2 as ::core::ffi::c_int;
    return safe_c2rust_node_get_by_pos(begin as *mut GSequenceNode, mid_pos as gint)
        as *mut GSequenceIter;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_iter_compare(
    mut a: *mut GSequenceIter,
    mut b: *mut GSequenceIter,
) -> gint {
    let mut a_pos: gint = 0;
    let mut b_pos: gint = 0;
    let mut seq_a: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    let mut seq_b: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !a.is_null() {
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
            b"a != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !b.is_null() {
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
            b"b != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    seq_a = safe_c2rust_get_sequence(a as *mut GSequenceNode);
    seq_b = safe_c2rust_get_sequence(b as *mut GSequenceNode);
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if seq_a == seq_b {
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
            b"seq_a == seq_b\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    safe_c2rust_check_seq_access(seq_a);
    safe_c2rust_check_seq_access(seq_b);
    a_pos = safe_c2rust_node_get_pos(a as *mut GSequenceNode);
    b_pos = safe_c2rust_node_get_pos(b as *mut GSequenceNode);
    if a_pos == b_pos {
        return 0 as gint;
    } else if a_pos > b_pos {
        return 1 as gint;
    } else {
        return -(1 as gint);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_append(
    mut seq: *mut GSequence,
    mut data: gpointer,
) -> *mut GSequenceIter {
    let mut node: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !seq.is_null() {
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
            b"seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    safe_c2rust_check_seq_access(seq);
    node = safe_c2rust_node_new(data);
    safe_c2rust_node_insert_before((*seq).end_node, node);
    return node as *mut GSequenceIter;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_prepend(
    mut seq: *mut GSequence,
    mut data: gpointer,
) -> *mut GSequenceIter {
    let mut node: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    let mut first: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !seq.is_null() {
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
            b"seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    safe_c2rust_check_seq_access(seq);
    node = safe_c2rust_node_new(data);
    first = safe_c2rust_node_get_first((*seq).end_node);
    safe_c2rust_node_insert_before(first, node);
    return node as *mut GSequenceIter;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_insert_before(
    mut iter: *mut GSequenceIter,
    mut data: gpointer,
) -> *mut GSequenceIter {
    let mut seq: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    let mut node: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    seq = safe_c2rust_get_sequence(iter as *mut GSequenceNode);
    safe_c2rust_check_seq_access(seq);
    node = safe_c2rust_node_new(data);
    safe_c2rust_node_insert_before(iter as *mut GSequenceNode, node);
    return node as *mut GSequenceIter;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_remove(mut iter: *mut GSequenceIter) {
    let mut seq: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    seq = safe_c2rust_get_sequence(iter as *mut GSequenceNode);
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if safe_c2rust_seq_is_end(seq, iter) == 0 {
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
            b"!seq_is_end (seq, iter)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_check_seq_access(seq);
    safe_c2rust_node_unlink(iter as *mut GSequenceNode);
    safe_c2rust_node_free(iter as *mut GSequenceNode, seq);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_remove_range(
    mut begin: *mut GSequenceIter,
    mut end: *mut GSequenceIter,
) {
    let mut seq_begin: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    let mut seq_end: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    seq_begin = safe_c2rust_get_sequence(begin as *mut GSequenceNode);
    seq_end = safe_c2rust_get_sequence(end as *mut GSequenceNode);
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if seq_begin == seq_end {
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
            b"seq_begin == seq_end\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_sequence_move_range(::core::ptr::null_mut::<GSequenceIter>(), begin, end);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_move_range(
    mut dest: *mut GSequenceIter,
    mut begin: *mut GSequenceIter,
    mut end: *mut GSequenceIter,
) {
    let mut src_seq: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    let mut end_seq: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    let mut dest_seq: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    let mut first: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !begin.is_null() {
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
            b"begin != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !end.is_null() {
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
            b"end != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    src_seq = safe_c2rust_get_sequence(begin as *mut GSequenceNode);
    safe_c2rust_check_seq_access(src_seq);
    end_seq = safe_c2rust_get_sequence(end as *mut GSequenceNode);
    safe_c2rust_check_seq_access(end_seq);
    if !dest.is_null() {
        dest_seq = safe_c2rust_get_sequence(dest as *mut GSequenceNode);
        safe_c2rust_check_seq_access(dest_seq);
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if src_seq == end_seq {
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
            b"src_seq == end_seq\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if dest == begin || dest == end {
        return;
    }
    if safe_c2rust_g_sequence_iter_compare(begin, end) >= 0 as ::core::ffi::c_int {
        return;
    }
    if !dest.is_null()
        && dest_seq == src_seq
        && safe_c2rust_g_sequence_iter_compare(dest, begin) > 0 as ::core::ffi::c_int
        && safe_c2rust_g_sequence_iter_compare(dest, end) < 0 as ::core::ffi::c_int
    {
        return;
    }
    first = safe_c2rust_node_get_first(begin as *mut GSequenceNode);
    safe_c2rust_node_cut(begin as *mut GSequenceNode);
    safe_c2rust_node_cut(end as *mut GSequenceNode);
    if first != begin {
        safe_c2rust_node_join(first, end as *mut GSequenceNode);
    }
    if !dest.is_null() {
        first = safe_c2rust_node_get_first(dest as *mut GSequenceNode);
        safe_c2rust_node_cut(dest as *mut GSequenceNode);
        safe_c2rust_node_join(begin as *mut GSequenceNode, dest as *mut GSequenceNode);
        if dest != first {
            safe_c2rust_node_join(first, begin as *mut GSequenceNode);
        }
    } else {
        safe_c2rust_node_free(begin as *mut GSequenceNode, src_seq);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_sort(
    mut seq: *mut GSequence,
    mut cmp_func: GCompareDataFunc,
    mut cmp_data: gpointer,
) {
    let mut info: SortInfo = SortInfo {
        cmp_func: None,
        cmp_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        end_node: ::core::ptr::null_mut::<GSequenceNode>(),
    };
    info.cmp_func = cmp_func;
    info.cmp_data = cmp_data;
    info.end_node = (*seq).end_node;
    safe_c2rust_check_seq_access(seq);
    safe_c2rust_g_sequence_sort_iter(
        seq,
        Some(
            safe_c2rust_iter_compare
                as unsafe extern "C" fn(*mut GSequenceIter, *mut GSequenceIter, gpointer) -> gint,
        ),
        &raw mut info as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_insert_sorted(
    mut seq: *mut GSequence,
    mut data: gpointer,
    mut cmp_func: GCompareDataFunc,
    mut cmp_data: gpointer,
) -> *mut GSequenceIter {
    let mut info: SortInfo = SortInfo {
        cmp_func: None,
        cmp_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        end_node: ::core::ptr::null_mut::<GSequenceNode>(),
    };
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !seq.is_null() {
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
            b"seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if cmp_func.is_some() {
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
            b"cmp_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    info.cmp_func = cmp_func;
    info.cmp_data = cmp_data;
    info.end_node = (*seq).end_node;
    safe_c2rust_check_seq_access(seq);
    return safe_c2rust_g_sequence_insert_sorted_iter(
        seq,
        data,
        Some(
            safe_c2rust_iter_compare
                as unsafe extern "C" fn(*mut GSequenceIter, *mut GSequenceIter, gpointer) -> gint,
        ),
        &raw mut info as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_sort_changed(
    mut iter: *mut GSequenceIter,
    mut cmp_func: GCompareDataFunc,
    mut cmp_data: gpointer,
) {
    let mut seq: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    let mut info: SortInfo = SortInfo {
        cmp_func: None,
        cmp_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        end_node: ::core::ptr::null_mut::<GSequenceNode>(),
    };
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    seq = safe_c2rust_get_sequence(iter as *mut GSequenceNode);
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if safe_c2rust_seq_is_end(seq, iter) == 0 {
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
            b"!seq_is_end (seq, iter)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    info.cmp_func = cmp_func;
    info.cmp_data = cmp_data;
    info.end_node = (*seq).end_node;
    safe_c2rust_g_sequence_sort_changed_iter(
        iter,
        Some(
            safe_c2rust_iter_compare
                as unsafe extern "C" fn(*mut GSequenceIter, *mut GSequenceIter, gpointer) -> gint,
        ),
        &raw mut info as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_search(
    mut seq: *mut GSequence,
    mut data: gpointer,
    mut cmp_func: GCompareDataFunc,
    mut cmp_data: gpointer,
) -> *mut GSequenceIter {
    let mut info: SortInfo = SortInfo {
        cmp_func: None,
        cmp_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        end_node: ::core::ptr::null_mut::<GSequenceNode>(),
    };
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !seq.is_null() {
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
            b"seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    info.cmp_func = cmp_func;
    info.cmp_data = cmp_data;
    info.end_node = (*seq).end_node;
    safe_c2rust_check_seq_access(seq);
    return safe_c2rust_g_sequence_search_iter(
        seq,
        data,
        Some(
            safe_c2rust_iter_compare
                as unsafe extern "C" fn(*mut GSequenceIter, *mut GSequenceIter, gpointer) -> gint,
        ),
        &raw mut info as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_lookup(
    mut seq: *mut GSequence,
    mut data: gpointer,
    mut cmp_func: GCompareDataFunc,
    mut cmp_data: gpointer,
) -> *mut GSequenceIter {
    let mut info: SortInfo = SortInfo {
        cmp_func: None,
        cmp_data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        end_node: ::core::ptr::null_mut::<GSequenceNode>(),
    };
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !seq.is_null() {
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
            b"seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    info.cmp_func = cmp_func;
    info.cmp_data = cmp_data;
    info.end_node = (*seq).end_node;
    safe_c2rust_check_seq_access(seq);
    return safe_c2rust_g_sequence_lookup_iter(
        seq,
        data,
        Some(
            safe_c2rust_iter_compare
                as unsafe extern "C" fn(*mut GSequenceIter, *mut GSequenceIter, gpointer) -> gint,
        ),
        &raw mut info as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_sort_iter(
    mut seq: *mut GSequence,
    mut cmp_func: GSequenceIterCompareFunc,
    mut cmp_data: gpointer,
) {
    let mut tmp: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    let mut begin: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    let mut end: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !seq.is_null() {
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
            b"seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if cmp_func.is_some() {
            _g_boolean_var_36 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_36 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_36
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"cmp_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_check_seq_access(seq);
    begin = safe_c2rust_g_sequence_get_begin_iter(seq) as *mut GSequenceNode;
    end = safe_c2rust_g_sequence_get_end_iter(seq) as *mut GSequenceNode;
    tmp = safe_c2rust_g_sequence_new(None);
    (*tmp).real_sequence = seq;
    safe_c2rust_g_sequence_move_range(
        safe_c2rust_g_sequence_get_begin_iter(tmp),
        begin as *mut GSequenceIter,
        end as *mut GSequenceIter,
    );
    (*seq).access_prohibited = TRUE as gboolean;
    (*tmp).access_prohibited = TRUE as gboolean;
    while safe_c2rust_g_sequence_is_empty(tmp) == 0 {
        let mut node: *mut GSequenceNode =
            safe_c2rust_g_sequence_get_begin_iter(tmp) as *mut GSequenceNode;
        safe_c2rust_node_insert_sorted((*seq).end_node, node, (*seq).end_node, cmp_func, cmp_data);
    }
    (*tmp).access_prohibited = FALSE as gboolean;
    (*seq).access_prohibited = FALSE as gboolean;
    safe_c2rust_g_sequence_free(tmp);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_sort_changed_iter(
    mut iter: *mut GSequenceIter,
    mut iter_cmp: GSequenceIterCompareFunc,
    mut cmp_data: gpointer,
) {
    let mut seq: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    let mut tmp_seq: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    let mut next: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    let mut prev: *mut GSequenceIter = ::core::ptr::null_mut::<GSequenceIter>();
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !iter.is_null() {
            _g_boolean_var_37 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_37 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_37
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if iter_cmp.is_some() {
            _g_boolean_var_38 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_38 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_38
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"iter_cmp != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    seq = safe_c2rust_get_sequence(iter as *mut GSequenceNode);
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if safe_c2rust_seq_is_end(seq, iter) == 0 {
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
            b"!seq_is_end (seq, iter)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_check_seq_access(seq);
    next = safe_c2rust_node_get_next(iter as *mut GSequenceNode) as *mut GSequenceIter;
    prev = safe_c2rust_node_get_prev(iter as *mut GSequenceNode) as *mut GSequenceIter;
    if prev != iter
        && iter_cmp.expect("non-null function pointer")(prev, iter, cmp_data)
            == 0 as ::core::ffi::c_int
    {
        return;
    }
    if safe_c2rust_is_end(next) == 0
        && iter_cmp.expect("non-null function pointer")(next, iter, cmp_data)
            == 0 as ::core::ffi::c_int
    {
        return;
    }
    (*seq).access_prohibited = TRUE as gboolean;
    tmp_seq = safe_c2rust_g_sequence_new(None);
    (*tmp_seq).real_sequence = seq;
    safe_c2rust_node_unlink(iter as *mut GSequenceNode);
    safe_c2rust_node_insert_before((*tmp_seq).end_node, iter as *mut GSequenceNode);
    safe_c2rust_node_insert_sorted(
        (*seq).end_node,
        iter as *mut GSequenceNode,
        (*seq).end_node,
        iter_cmp,
        cmp_data,
    );
    safe_c2rust_g_sequence_free(tmp_seq);
    (*seq).access_prohibited = FALSE as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_insert_sorted_iter(
    mut seq: *mut GSequence,
    mut data: gpointer,
    mut iter_cmp: GSequenceIterCompareFunc,
    mut cmp_data: gpointer,
) -> *mut GSequenceIter {
    let mut new_node: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    let mut tmp_seq: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !seq.is_null() {
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
            b"seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if iter_cmp.is_some() {
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
            b"iter_cmp != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    safe_c2rust_check_seq_access(seq);
    (*seq).access_prohibited = TRUE as gboolean;
    tmp_seq = safe_c2rust_g_sequence_new(None);
    (*tmp_seq).real_sequence = seq;
    new_node = safe_c2rust_g_sequence_append(tmp_seq, data) as *mut GSequenceNode;
    safe_c2rust_node_insert_sorted(
        (*seq).end_node,
        new_node,
        (*seq).end_node,
        iter_cmp,
        cmp_data,
    );
    safe_c2rust_g_sequence_free(tmp_seq);
    (*seq).access_prohibited = FALSE as gboolean;
    return new_node as *mut GSequenceIter;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_search_iter(
    mut seq: *mut GSequence,
    mut data: gpointer,
    mut iter_cmp: GSequenceIterCompareFunc,
    mut cmp_data: gpointer,
) -> *mut GSequenceIter {
    let mut node: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    let mut dummy: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    let mut tmp_seq: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !seq.is_null() {
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
            b"seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    safe_c2rust_check_seq_access(seq);
    (*seq).access_prohibited = TRUE as gboolean;
    tmp_seq = safe_c2rust_g_sequence_new(None);
    (*tmp_seq).real_sequence = seq;
    dummy = safe_c2rust_g_sequence_append(tmp_seq, data) as *mut GSequenceNode;
    node =
        safe_c2rust_node_find_closest((*seq).end_node, dummy, (*seq).end_node, iter_cmp, cmp_data);
    safe_c2rust_g_sequence_free(tmp_seq);
    (*seq).access_prohibited = FALSE as gboolean;
    return node as *mut GSequenceIter;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_lookup_iter(
    mut seq: *mut GSequence,
    mut data: gpointer,
    mut iter_cmp: GSequenceIterCompareFunc,
    mut cmp_data: gpointer,
) -> *mut GSequenceIter {
    let mut node: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    let mut dummy: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    let mut tmp_seq: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !seq.is_null() {
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
            b"seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    safe_c2rust_check_seq_access(seq);
    (*seq).access_prohibited = TRUE as gboolean;
    tmp_seq = safe_c2rust_g_sequence_new(None);
    (*tmp_seq).real_sequence = seq;
    dummy = safe_c2rust_g_sequence_append(tmp_seq, data) as *mut GSequenceNode;
    node = safe_c2rust_node_find((*seq).end_node, dummy, (*seq).end_node, iter_cmp, cmp_data);
    safe_c2rust_g_sequence_free(tmp_seq);
    (*seq).access_prohibited = FALSE as gboolean;
    return node as *mut GSequenceIter;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_iter_get_sequence(
    mut iter: *mut GSequenceIter,
) -> *mut GSequence {
    let mut seq: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequence>();
    }
    seq = safe_c2rust_get_sequence(iter as *mut GSequenceNode);
    return (*seq).real_sequence;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_get(mut iter: *mut GSequenceIter) -> gpointer {
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if safe_c2rust_is_end(iter) == 0 {
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
            b"!is_end (iter)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*iter).data;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_set(
    mut iter: *mut GSequenceIter,
    mut data: gpointer,
) {
    let mut seq: *mut GSequence = ::core::ptr::null_mut::<GSequence>();
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    seq = safe_c2rust_get_sequence(iter as *mut GSequenceNode);
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if safe_c2rust_seq_is_end(seq, iter) == 0 {
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
            b"!seq_is_end (seq, iter)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*seq).data_destroy_notify.is_some() {
        (*seq)
            .data_destroy_notify
            .expect("non-null function pointer")((*iter).data);
    }
    (*iter).data = data;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_get_length(mut seq: *mut GSequence) -> gint {
    return safe_c2rust_node_get_length((*seq).end_node) - 1 as gint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_is_empty(mut seq: *mut GSequence) -> gboolean {
    return ((*(*seq).end_node).parent.is_null() && (*(*seq).end_node).left.is_null())
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_get_end_iter(
    mut seq: *mut GSequence,
) -> *mut GSequenceIter {
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if !seq.is_null() {
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
            b"seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    return (*seq).end_node as *mut GSequenceIter;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_get_begin_iter(
    mut seq: *mut GSequence,
) -> *mut GSequenceIter {
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if !seq.is_null() {
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
            b"seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    return safe_c2rust_node_get_first((*seq).end_node) as *mut GSequenceIter;
}
unsafe extern "C" fn safe_c2rust_clamp_position(
    mut seq: *mut GSequence,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut len: gint = safe_c2rust_g_sequence_get_length(seq);
    if pos > len || pos < 0 as ::core::ffi::c_int {
        pos = len as ::core::ffi::c_int;
    }
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_get_iter_at_pos(
    mut seq: *mut GSequence,
    mut pos: gint,
) -> *mut GSequenceIter {
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if !seq.is_null() {
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
            b"seq != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    pos = safe_c2rust_clamp_position(seq, pos as ::core::ffi::c_int) as gint;
    return safe_c2rust_node_get_by_pos((*seq).end_node, pos) as *mut GSequenceIter;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_move(
    mut src: *mut GSequenceIter,
    mut dest: *mut GSequenceIter,
) {
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if !src.is_null() {
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
            b"src != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if !dest.is_null() {
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
            b"dest != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if safe_c2rust_is_end(src) == 0 {
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
            b"!is_end (src)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if src == dest {
        return;
    }
    safe_c2rust_node_unlink(src as *mut GSequenceNode);
    safe_c2rust_node_insert_before(dest as *mut GSequenceNode, src as *mut GSequenceNode);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_iter_is_end(
    mut iter: *mut GSequenceIter,
) -> gboolean {
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return safe_c2rust_is_end(iter);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_iter_is_begin(
    mut iter: *mut GSequenceIter,
) -> gboolean {
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (safe_c2rust_node_get_prev(iter as *mut GSequenceNode) == iter) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_iter_get_position(
    mut iter: *mut GSequenceIter,
) -> gint {
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    return safe_c2rust_node_get_pos(iter as *mut GSequenceNode);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_iter_next(
    mut iter: *mut GSequenceIter,
) -> *mut GSequenceIter {
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    return safe_c2rust_node_get_next(iter as *mut GSequenceNode) as *mut GSequenceIter;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_iter_prev(
    mut iter: *mut GSequenceIter,
) -> *mut GSequenceIter {
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    return safe_c2rust_node_get_prev(iter as *mut GSequenceNode) as *mut GSequenceIter;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_iter_move(
    mut iter: *mut GSequenceIter,
    mut delta: gint,
) -> *mut GSequenceIter {
    let mut new_pos: gint = 0;
    let mut len: gint = 0;
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if !iter.is_null() {
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
            b"iter != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GSequenceIter>();
    }
    len = safe_c2rust_g_sequence_get_length(safe_c2rust_get_sequence(iter as *mut GSequenceNode));
    new_pos = safe_c2rust_node_get_pos(iter as *mut GSequenceNode) + delta;
    if new_pos < 0 as ::core::ffi::c_int {
        new_pos = 0 as ::core::ffi::c_int as gint;
    } else if new_pos > len {
        new_pos = len;
    }
    return safe_c2rust_node_get_by_pos(iter as *mut GSequenceNode, new_pos) as *mut GSequenceIter;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_sequence_swap(
    mut a: *mut GSequenceIter,
    mut b: *mut GSequenceIter,
) {
    let mut leftmost: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    let mut rightmost: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    let mut rightmost_next: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    let mut a_pos: ::core::ffi::c_int = 0;
    let mut b_pos: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if safe_c2rust_g_sequence_iter_is_end(a) == 0 {
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
            b"!g_sequence_iter_is_end (a)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if safe_c2rust_g_sequence_iter_is_end(b) == 0 {
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
            b"!g_sequence_iter_is_end (b)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if a == b {
        return;
    }
    a_pos = safe_c2rust_g_sequence_iter_get_position(a) as ::core::ffi::c_int;
    b_pos = safe_c2rust_g_sequence_iter_get_position(b) as ::core::ffi::c_int;
    if a_pos > b_pos {
        leftmost = b as *mut GSequenceNode;
        rightmost = a as *mut GSequenceNode;
    } else {
        leftmost = a as *mut GSequenceNode;
        rightmost = b as *mut GSequenceNode;
    }
    rightmost_next = safe_c2rust_node_get_next(rightmost);
    safe_c2rust_g_sequence_move(
        rightmost as *mut GSequenceIter,
        leftmost as *mut GSequenceIter,
    );
    safe_c2rust_g_sequence_move(
        leftmost as *mut GSequenceIter,
        rightmost_next as *mut GSequenceIter,
    );
}
unsafe extern "C" fn safe_c2rust_hash_uint32(mut key: guint32) -> guint32 {
    key = (key << 15 as ::core::ffi::c_int)
        .wrapping_sub(key)
        .wrapping_sub(1 as guint32);
    key = key ^ key >> 12 as ::core::ffi::c_int;
    key = key.wrapping_add(key << 2 as ::core::ffi::c_int);
    key = key ^ key >> 4 as ::core::ffi::c_int;
    key = key
        .wrapping_add(key << 3 as ::core::ffi::c_int)
        .wrapping_add(key << 11 as ::core::ffi::c_int);
    key = key ^ key >> 16 as ::core::ffi::c_int;
    return key;
}
#[inline]
unsafe extern "C" fn safe_c2rust_get_priority(mut node: *mut GSequenceNode) -> guint {
    return (*node).priority as guint;
}
unsafe extern "C" fn safe_c2rust_make_priority(mut key: guint32) -> guint {
    key = safe_c2rust_hash_uint32(key);
    return if key != 0 { key as guint } else { 1 as guint };
}
unsafe extern "C" fn safe_c2rust_find_root(mut node: *mut GSequenceNode) -> *mut GSequenceNode {
    while !(*node).parent.is_null() {
        node = (*node).parent;
    }
    return node;
}
unsafe extern "C" fn safe_c2rust_node_new(mut data: gpointer) -> *mut GSequenceNode {
    let mut node: *mut GSequenceNode = ({
        let mut __s: gsize = ::core::mem::size_of::<GSequenceNode>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GSequenceNode;
    static mut safe_c2rust_counter: guint64 = 0 as guint64;
    let mut hash_key: guint32 = node as gulong as guint;
    hash_key ^= safe_c2rust_counter as guint32;
    safe_c2rust_counter = safe_c2rust_counter.wrapping_add(1);
    (*node).n_nodes = 1 as ::core::ffi::c_int as gint;
    (*node).priority = safe_c2rust_make_priority(hash_key) as guint32;
    (*node).data = data;
    (*node).left = ::core::ptr::null_mut::<GSequenceNode>();
    (*node).right = ::core::ptr::null_mut::<GSequenceNode>();
    (*node).parent = ::core::ptr::null_mut::<GSequenceNode>();
    return node;
}
unsafe extern "C" fn safe_c2rust_node_get_first(
    mut node: *mut GSequenceNode,
) -> *mut GSequenceNode {
    node = safe_c2rust_find_root(node);
    while !(*node).left.is_null() {
        node = (*node).left;
    }
    return node;
}
unsafe extern "C" fn safe_c2rust_node_get_last(mut node: *mut GSequenceNode) -> *mut GSequenceNode {
    node = safe_c2rust_find_root(node);
    while !(*node).right.is_null() {
        node = (*node).right;
    }
    return node;
}
unsafe extern "C" fn safe_c2rust_node_get_next(mut node: *mut GSequenceNode) -> *mut GSequenceNode {
    let mut n: *mut GSequenceNode = node;
    if !(*n).right.is_null() {
        n = (*n).right;
        while !(*n).left.is_null() {
            n = (*n).left;
        }
    } else {
        while !(*n).parent.is_null() && (*(*n).parent).right == n {
            n = (*n).parent;
        }
        if !(*n).parent.is_null() {
            n = (*n).parent;
        } else {
            n = node;
        }
    }
    return n;
}
unsafe extern "C" fn safe_c2rust_node_get_prev(mut node: *mut GSequenceNode) -> *mut GSequenceNode {
    let mut n: *mut GSequenceNode = node;
    if !(*n).left.is_null() {
        n = (*n).left;
        while !(*n).right.is_null() {
            n = (*n).right;
        }
    } else {
        while !(*n).parent.is_null() && (*(*n).parent).left == n {
            n = (*n).parent;
        }
        if !(*n).parent.is_null() {
            n = (*n).parent;
        } else {
            n = node;
        }
    }
    return n;
}
unsafe extern "C" fn safe_c2rust_node_get_pos(mut node: *mut GSequenceNode) -> gint {
    let mut n_smaller: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*node).left.is_null() {
        n_smaller = (*(*node).left).n_nodes as ::core::ffi::c_int;
    }
    while !node.is_null() {
        if !(*node).parent.is_null() && (*(*node).parent).right == node {
            n_smaller += (if !(*(*node).parent).left.is_null() {
                (*(*(*node).parent).left).n_nodes as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) + 1 as ::core::ffi::c_int;
        }
        node = (*node).parent;
    }
    return n_smaller as gint;
}
unsafe extern "C" fn safe_c2rust_node_get_by_pos(
    mut node: *mut GSequenceNode,
    mut pos: gint,
) -> *mut GSequenceNode {
    let mut i: ::core::ffi::c_int = 0;
    node = safe_c2rust_find_root(node);
    loop {
        i = (if !(*node).left.is_null() {
            (*(*node).left).n_nodes as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
        if !(i != pos) {
            break;
        }
        if i < pos {
            node = (*node).right;
            pos -= i + 1 as ::core::ffi::c_int;
        } else {
            node = (*node).left;
        }
    }
    return node;
}
unsafe extern "C" fn safe_c2rust_node_find(
    mut haystack: *mut GSequenceNode,
    mut needle: *mut GSequenceNode,
    mut end: *mut GSequenceNode,
    mut iter_cmp: GSequenceIterCompareFunc,
    mut cmp_data: gpointer,
) -> *mut GSequenceNode {
    let mut c: gint = 0;
    haystack = safe_c2rust_find_root(haystack);
    loop {
        if haystack == end {
            c = 1 as ::core::ffi::c_int as gint;
        } else {
            c = iter_cmp.expect("non-null function pointer")(
                haystack as *mut GSequenceIter,
                needle as *mut GSequenceIter,
                cmp_data,
            );
        }
        if c == 0 as ::core::ffi::c_int {
            break;
        }
        if c > 0 as ::core::ffi::c_int {
            haystack = (*haystack).left;
        } else {
            haystack = (*haystack).right;
        }
        if haystack.is_null() {
            break;
        }
    }
    return haystack;
}
unsafe extern "C" fn safe_c2rust_node_find_closest(
    mut haystack: *mut GSequenceNode,
    mut needle: *mut GSequenceNode,
    mut end: *mut GSequenceNode,
    mut iter_cmp: GSequenceIterCompareFunc,
    mut cmp_data: gpointer,
) -> *mut GSequenceNode {
    let mut best: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    let mut c: gint = 0;
    haystack = safe_c2rust_find_root(haystack);
    loop {
        best = haystack;
        if haystack == end {
            c = 1 as ::core::ffi::c_int as gint;
        } else {
            c = iter_cmp.expect("non-null function pointer")(
                haystack as *mut GSequenceIter,
                needle as *mut GSequenceIter,
                cmp_data,
            );
        }
        if c > 0 as ::core::ffi::c_int {
            haystack = (*haystack).left;
        } else {
            haystack = (*haystack).right;
        }
        if haystack.is_null() {
            break;
        }
    }
    if best != end && c <= 0 as ::core::ffi::c_int {
        best = safe_c2rust_node_get_next(best);
    }
    return best;
}
unsafe extern "C" fn safe_c2rust_node_get_length(mut node: *mut GSequenceNode) -> gint {
    node = safe_c2rust_find_root(node);
    return (*node).n_nodes;
}
unsafe extern "C" fn safe_c2rust_real_node_free(
    mut node: *mut GSequenceNode,
    mut seq: *mut GSequence,
) {
    if !node.is_null() {
        safe_c2rust_real_node_free((*node).left, seq);
        safe_c2rust_real_node_free((*node).right, seq);
        if !seq.is_null() && (*seq).data_destroy_notify.is_some() && node != (*seq).end_node {
            (*seq)
                .data_destroy_notify
                .expect("non-null function pointer")((*node).data);
        }
        g_slice_free1(
            ::core::mem::size_of::<GSequenceNode>() as gsize,
            node as gpointer,
        );
    }
}
unsafe extern "C" fn safe_c2rust_node_free(mut node: *mut GSequenceNode, mut seq: *mut GSequence) {
    node = safe_c2rust_find_root(node);
    safe_c2rust_real_node_free(node, seq);
}
unsafe extern "C" fn safe_c2rust_node_update_fields(mut node: *mut GSequenceNode) {
    let mut n_nodes: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    n_nodes += if !(*node).left.is_null() {
        (*(*node).left).n_nodes as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    n_nodes += if !(*node).right.is_null() {
        (*(*node).right).n_nodes as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    (*node).n_nodes = n_nodes as gint;
}
unsafe extern "C" fn safe_c2rust_node_rotate(mut node: *mut GSequenceNode) {
    let mut tmp: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    let mut old: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    if ({
        let mut _g_boolean_var_63: ::core::ffi::c_int = 0;
        if !(*node).parent.is_null() {
            _g_boolean_var_63 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_63 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_63
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gsequence.c\0" as *const u8 as *const ::core::ffi::c_char,
            1867 as ::core::ffi::c_int,
            G_STRFUNC,
            b"node->parent\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_64: ::core::ffi::c_int = 0;
        if (*node).parent != node {
            _g_boolean_var_64 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_64 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_64
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gsequence.c\0" as *const u8 as *const ::core::ffi::c_char,
            1868 as ::core::ffi::c_int,
            G_STRFUNC,
            b"node->parent != node\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !(*node).parent.is_null() && (*(*node).parent).left == node {
        tmp = (*node).right;
        (*node).right = (*node).parent;
        (*node).parent = (*(*node).parent).parent;
        if !(*node).parent.is_null() {
            if (*(*node).parent).left == (*node).right {
                (*(*node).parent).left = node;
            } else {
                (*(*node).parent).right = node;
            }
        }
        if ({
            let mut _g_boolean_var_65: ::core::ffi::c_int = 0;
            if !(*node).right.is_null() {
                _g_boolean_var_65 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_65 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_65
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gsequence.c\0" as *const u8 as *const ::core::ffi::c_char,
                1885 as ::core::ffi::c_int,
                G_STRFUNC,
                b"node->right\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*(*node).right).parent = node;
        (*(*node).right).left = tmp;
        if !(*(*node).right).left.is_null() {
            (*(*(*node).right).left).parent = (*node).right;
        }
        old = (*node).right;
    } else {
        tmp = (*node).left;
        (*node).left = (*node).parent;
        (*node).parent = (*(*node).parent).parent;
        if !(*node).parent.is_null() {
            if (*(*node).parent).right == (*node).left {
                (*(*node).parent).right = node;
            } else {
                (*(*node).parent).left = node;
            }
        }
        if ({
            let mut _g_boolean_var_66: ::core::ffi::c_int = 0;
            if !(*node).left.is_null() {
                _g_boolean_var_66 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_66 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_66
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gsequence.c\0" as *const u8 as *const ::core::ffi::c_char,
                1910 as ::core::ffi::c_int,
                G_STRFUNC,
                b"node->left\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        (*(*node).left).parent = node;
        (*(*node).left).right = tmp;
        if !(*(*node).left).right.is_null() {
            (*(*(*node).left).right).parent = (*node).left;
        }
        old = (*node).left;
    }
    safe_c2rust_node_update_fields(old);
    safe_c2rust_node_update_fields(node);
}
unsafe extern "C" fn safe_c2rust_node_update_fields_deep(mut node: *mut GSequenceNode) {
    if !node.is_null() {
        safe_c2rust_node_update_fields(node);
        safe_c2rust_node_update_fields_deep((*node).parent);
    }
}
unsafe extern "C" fn safe_c2rust_rotate_down(mut node: *mut GSequenceNode, mut priority: guint) {
    let mut left: guint = 0;
    let mut right: guint = 0;
    left = if !(*node).left.is_null() {
        safe_c2rust_get_priority((*node).left)
    } else {
        0 as guint
    };
    right = if !(*node).right.is_null() {
        safe_c2rust_get_priority((*node).right)
    } else {
        0 as guint
    };
    while priority < left || priority < right {
        if left > right {
            safe_c2rust_node_rotate((*node).left);
        } else {
            safe_c2rust_node_rotate((*node).right);
        }
        left = if !(*node).left.is_null() {
            safe_c2rust_get_priority((*node).left)
        } else {
            0 as guint
        };
        right = if !(*node).right.is_null() {
            safe_c2rust_get_priority((*node).right)
        } else {
            0 as guint
        };
    }
}
unsafe extern "C" fn safe_c2rust_node_cut(mut node: *mut GSequenceNode) {
    while !(*node).parent.is_null() {
        safe_c2rust_node_rotate(node);
    }
    if !(*node).left.is_null() {
        (*(*node).left).parent = ::core::ptr::null_mut::<GSequenceNode>();
    }
    (*node).left = ::core::ptr::null_mut::<GSequenceNode>();
    safe_c2rust_node_update_fields(node);
    safe_c2rust_rotate_down(node, safe_c2rust_get_priority(node));
}
unsafe extern "C" fn safe_c2rust_node_join(
    mut left: *mut GSequenceNode,
    mut right: *mut GSequenceNode,
) {
    let mut fake: *mut GSequenceNode = safe_c2rust_node_new(NULL);
    (*fake).left = safe_c2rust_find_root(left);
    (*fake).right = safe_c2rust_find_root(right);
    (*(*fake).left).parent = fake;
    (*(*fake).right).parent = fake;
    safe_c2rust_node_update_fields(fake);
    safe_c2rust_node_unlink(fake);
    safe_c2rust_node_free(fake, ::core::ptr::null_mut::<GSequence>());
}
unsafe extern "C" fn safe_c2rust_node_insert_before(
    mut node: *mut GSequenceNode,
    mut new: *mut GSequenceNode,
) {
    (*new).left = (*node).left;
    if !(*new).left.is_null() {
        (*(*new).left).parent = new;
    }
    (*new).parent = node;
    (*node).left = new;
    safe_c2rust_node_update_fields_deep(new);
    while !(*new).parent.is_null()
        && safe_c2rust_get_priority(new) > safe_c2rust_get_priority((*new).parent)
    {
        safe_c2rust_node_rotate(new);
    }
    safe_c2rust_rotate_down(new, safe_c2rust_get_priority(new));
}
unsafe extern "C" fn safe_c2rust_node_unlink(mut node: *mut GSequenceNode) {
    safe_c2rust_rotate_down(node, 0 as guint);
    if !(*node).parent.is_null() && (*(*node).parent).right == node {
        (*(*node).parent).right = ::core::ptr::null_mut::<GSequenceNode>();
    } else if !(*node).parent.is_null() && (*(*node).parent).left == node {
        (*(*node).parent).left = ::core::ptr::null_mut::<GSequenceNode>();
    }
    if !(*node).parent.is_null() {
        safe_c2rust_node_update_fields_deep((*node).parent);
    }
    (*node).parent = ::core::ptr::null_mut::<GSequenceNode>();
}
unsafe extern "C" fn safe_c2rust_node_insert_sorted(
    mut node: *mut GSequenceNode,
    mut new: *mut GSequenceNode,
    mut end: *mut GSequenceNode,
    mut iter_cmp: GSequenceIterCompareFunc,
    mut cmp_data: gpointer,
) {
    let mut closest: *mut GSequenceNode = ::core::ptr::null_mut::<GSequenceNode>();
    closest = safe_c2rust_node_find_closest(node, new, end, iter_cmp, cmp_data);
    safe_c2rust_node_unlink(new);
    safe_c2rust_node_insert_before(closest, new);
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_sequence_free\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
