extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
}
pub type size_t = usize;
pub type gsize = ::core::ffi::c_ulong;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCopyFunc = Option<unsafe extern "C" fn(gconstpointer, gpointer) -> gpointer>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GNode {
    pub data: gpointer,
    pub next: *mut GNode,
    pub prev: *mut GNode,
    pub parent: *mut GNode,
    pub children: *mut GNode,
}
pub type GNode = _GNode;
pub type GTraverseFlags = ::core::ffi::c_uint;
pub const G_TRAVERSE_NON_LEAFS: GTraverseFlags = 2;
pub const G_TRAVERSE_LEAFS: GTraverseFlags = 1;
pub const G_TRAVERSE_MASK: GTraverseFlags = 3;
pub const G_TRAVERSE_ALL: GTraverseFlags = 3;
pub const G_TRAVERSE_NON_LEAVES: GTraverseFlags = 2;
pub const G_TRAVERSE_LEAVES: GTraverseFlags = 1;
pub type GTraverseType = ::core::ffi::c_uint;
pub const G_LEVEL_ORDER: GTraverseType = 3;
pub const G_POST_ORDER: GTraverseType = 2;
pub const G_PRE_ORDER: GTraverseType = 1;
pub const G_IN_ORDER: GTraverseType = 0;
pub type GNodeTraverseFunc = Option<unsafe extern "C" fn(*mut GNode, gpointer) -> gboolean>;
pub type GNodeForeachFunc = Option<unsafe extern "C" fn(*mut GNode, gpointer) -> ()>;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_new(mut data: gpointer) -> *mut GNode {
    let mut node: *mut GNode = ({
        let mut __s: gsize = ::core::mem::size_of::<GNode>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GNode;
    (*node).data = data;
    return node;
}
unsafe extern "C" fn safe_c2rust_g_nodes_free(mut node: *mut GNode) {
    while !node.is_null() {
        let mut next: *mut GNode = (*node).next;
        if !(*node).children.is_null() {
            safe_c2rust_g_nodes_free((*node).children);
        }
        g_slice_free1(::core::mem::size_of::<GNode>() as gsize, node as gpointer);
        node = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_destroy(mut root: *mut GNode) {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !root.is_null() {
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
            b"root != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !((*root).parent.is_null() && (*root).prev.is_null() && (*root).next.is_null()) {
        safe_c2rust_g_node_unlink(root);
    }
    safe_c2rust_g_nodes_free(root);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_unlink(mut node: *mut GNode) {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if !(*node).prev.is_null() {
        (*(*node).prev).next = (*node).next;
    } else if !(*node).parent.is_null() {
        (*(*node).parent).children = (*node).next;
    }
    (*node).parent = ::core::ptr::null_mut::<GNode>();
    if !(*node).next.is_null() {
        (*(*node).next).prev = (*node).prev;
        (*node).next = ::core::ptr::null_mut::<GNode>();
    }
    (*node).prev = ::core::ptr::null_mut::<GNode>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_copy_deep(
    mut node: *mut GNode,
    mut copy_func: GCopyFunc,
    mut data: gpointer,
) -> *mut GNode {
    let mut new_node: *mut GNode = ::core::ptr::null_mut::<GNode>();
    if copy_func.is_none() {
        return safe_c2rust_g_node_copy(node);
    }
    if !node.is_null() {
        let mut child: *mut GNode = ::core::ptr::null_mut::<GNode>();
        let mut new_child: *mut GNode = ::core::ptr::null_mut::<GNode>();
        new_node = safe_c2rust_g_node_new(copy_func.expect("non-null function pointer")(
            (*node).data as gconstpointer,
            data,
        ));
        child = safe_c2rust_g_node_last_child(node);
        while !child.is_null() {
            new_child = safe_c2rust_g_node_copy_deep(child, copy_func, data);
            safe_c2rust_g_node_prepend(new_node, new_child);
            child = (*child).prev;
        }
    }
    return new_node;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_copy(mut node: *mut GNode) -> *mut GNode {
    let mut new_node: *mut GNode = ::core::ptr::null_mut::<GNode>();
    if !node.is_null() {
        let mut child: *mut GNode = ::core::ptr::null_mut::<GNode>();
        new_node = safe_c2rust_g_node_new((*node).data);
        child = safe_c2rust_g_node_last_child(node);
        while !child.is_null() {
            safe_c2rust_g_node_prepend(new_node, safe_c2rust_g_node_copy(child));
            child = (*child).prev;
        }
    }
    return new_node;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_insert(
    mut parent: *mut GNode,
    mut position: gint,
    mut node: *mut GNode,
) -> *mut GNode {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !parent.is_null() {
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
            b"parent != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return node;
    }
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return node;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if (*node).parent.is_null() && (*node).prev.is_null() && (*node).next.is_null() {
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
            b"G_NODE_IS_ROOT (node)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return node;
    }
    if position > 0 as ::core::ffi::c_int {
        return safe_c2rust_g_node_insert_before(
            parent,
            safe_c2rust_g_node_nth_child(parent, position as guint),
            node,
        );
    } else if position == 0 as ::core::ffi::c_int {
        return safe_c2rust_g_node_prepend(parent, node);
    } else {
        return safe_c2rust_g_node_insert_before(parent, ::core::ptr::null_mut::<GNode>(), node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_insert_before(
    mut parent: *mut GNode,
    mut sibling: *mut GNode,
    mut node: *mut GNode,
) -> *mut GNode {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !parent.is_null() {
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
            b"parent != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return node;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return node;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if (*node).parent.is_null() && (*node).prev.is_null() && (*node).next.is_null() {
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
            b"G_NODE_IS_ROOT (node)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return node;
    }
    if !sibling.is_null() {
        if ({
            let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
            if (*sibling).parent == parent {
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
                b"sibling->parent == parent\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return node;
        }
    }
    (*node).parent = parent;
    if !sibling.is_null() {
        if !(*sibling).prev.is_null() {
            (*node).prev = (*sibling).prev;
            (*(*node).prev).next = node;
            (*node).next = sibling;
            (*sibling).prev = node;
        } else {
            (*(*node).parent).children = node;
            (*node).next = sibling;
            (*sibling).prev = node;
        }
    } else if !(*parent).children.is_null() {
        sibling = (*parent).children;
        while !(*sibling).next.is_null() {
            sibling = (*sibling).next;
        }
        (*node).prev = sibling;
        (*sibling).next = node;
    } else {
        (*(*node).parent).children = node;
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_insert_after(
    mut parent: *mut GNode,
    mut sibling: *mut GNode,
    mut node: *mut GNode,
) -> *mut GNode {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !parent.is_null() {
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
            b"parent != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return node;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return node;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if (*node).parent.is_null() && (*node).prev.is_null() && (*node).next.is_null() {
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
            b"G_NODE_IS_ROOT (node)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return node;
    }
    if !sibling.is_null() {
        if ({
            let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
            if (*sibling).parent == parent {
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
                b"sibling->parent == parent\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return node;
        }
    }
    (*node).parent = parent;
    if !sibling.is_null() {
        if !(*sibling).next.is_null() {
            (*(*sibling).next).prev = node;
        }
        (*node).next = (*sibling).next;
        (*node).prev = sibling;
        (*sibling).next = node;
    } else {
        if !(*parent).children.is_null() {
            (*node).next = (*parent).children;
            (*(*parent).children).prev = node;
        }
        (*parent).children = node;
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_prepend(
    mut parent: *mut GNode,
    mut node: *mut GNode,
) -> *mut GNode {
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !parent.is_null() {
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
            b"parent != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return node;
    }
    return safe_c2rust_g_node_insert_before(parent, (*parent).children, node);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_get_root(mut node: *mut GNode) -> *mut GNode {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GNode>();
    }
    while !(*node).parent.is_null() {
        node = (*node).parent;
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_is_ancestor(
    mut node: *mut GNode,
    mut descendant: *mut GNode,
) -> gboolean {
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !descendant.is_null() {
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
            b"descendant != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    while !descendant.is_null() {
        if (*descendant).parent == node {
            return TRUE;
        }
        descendant = (*descendant).parent;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_depth(mut node: *mut GNode) -> guint {
    let mut depth: guint = 0 as guint;
    while !node.is_null() {
        depth = depth.wrapping_add(1);
        node = (*node).parent;
    }
    return depth;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_reverse_children(mut node: *mut GNode) {
    let mut child: *mut GNode = ::core::ptr::null_mut::<GNode>();
    let mut last: *mut GNode = ::core::ptr::null_mut::<GNode>();
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    child = (*node).children;
    last = ::core::ptr::null_mut::<GNode>();
    while !child.is_null() {
        last = child;
        child = (*last).next;
        (*last).next = (*last).prev;
        (*last).prev = child;
    }
    (*node).children = last;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_max_height(mut root: *mut GNode) -> guint {
    let mut child: *mut GNode = ::core::ptr::null_mut::<GNode>();
    let mut max_height: guint = 0 as guint;
    if root.is_null() {
        return 0 as guint;
    }
    child = (*root).children;
    while !child.is_null() {
        let mut tmp_height: guint = 0;
        tmp_height = safe_c2rust_g_node_max_height(child);
        if tmp_height > max_height {
            max_height = tmp_height;
        }
        child = (*child).next;
    }
    return max_height.wrapping_add(1 as guint);
}
unsafe extern "C" fn safe_c2rust_g_node_traverse_pre_order(
    mut node: *mut GNode,
    mut flags: GTraverseFlags,
    mut func: GNodeTraverseFunc,
    mut data: gpointer,
) -> gboolean {
    if !(*node).children.is_null() {
        let mut child: *mut GNode = ::core::ptr::null_mut::<GNode>();
        if flags as ::core::ffi::c_uint
            & G_TRAVERSE_NON_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && func.expect("non-null function pointer")(node, data) != 0
        {
            return TRUE;
        }
        child = (*node).children;
        while !child.is_null() {
            let mut current: *mut GNode = ::core::ptr::null_mut::<GNode>();
            current = child;
            child = (*current).next;
            if safe_c2rust_g_node_traverse_pre_order(current, flags, func, data) != 0 {
                return TRUE;
            }
        }
    } else if flags as ::core::ffi::c_uint
        & G_TRAVERSE_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && func.expect("non-null function pointer")(node, data) != 0
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_node_depth_traverse_pre_order(
    mut node: *mut GNode,
    mut flags: GTraverseFlags,
    mut depth: guint,
    mut func: GNodeTraverseFunc,
    mut data: gpointer,
) -> gboolean {
    if !(*node).children.is_null() {
        let mut child: *mut GNode = ::core::ptr::null_mut::<GNode>();
        if flags as ::core::ffi::c_uint
            & G_TRAVERSE_NON_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && func.expect("non-null function pointer")(node, data) != 0
        {
            return TRUE;
        }
        depth = depth.wrapping_sub(1);
        if depth == 0 {
            return FALSE;
        }
        child = (*node).children;
        while !child.is_null() {
            let mut current: *mut GNode = ::core::ptr::null_mut::<GNode>();
            current = child;
            child = (*current).next;
            if safe_c2rust_g_node_depth_traverse_pre_order(current, flags, depth, func, data) != 0 {
                return TRUE;
            }
        }
    } else if flags as ::core::ffi::c_uint
        & G_TRAVERSE_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && func.expect("non-null function pointer")(node, data) != 0
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_node_traverse_post_order(
    mut node: *mut GNode,
    mut flags: GTraverseFlags,
    mut func: GNodeTraverseFunc,
    mut data: gpointer,
) -> gboolean {
    if !(*node).children.is_null() {
        let mut child: *mut GNode = ::core::ptr::null_mut::<GNode>();
        child = (*node).children;
        while !child.is_null() {
            let mut current: *mut GNode = ::core::ptr::null_mut::<GNode>();
            current = child;
            child = (*current).next;
            if safe_c2rust_g_node_traverse_post_order(current, flags, func, data) != 0 {
                return TRUE;
            }
        }
        if flags as ::core::ffi::c_uint
            & G_TRAVERSE_NON_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && func.expect("non-null function pointer")(node, data) != 0
        {
            return TRUE;
        }
    } else if flags as ::core::ffi::c_uint
        & G_TRAVERSE_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && func.expect("non-null function pointer")(node, data) != 0
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_node_depth_traverse_post_order(
    mut node: *mut GNode,
    mut flags: GTraverseFlags,
    mut depth: guint,
    mut func: GNodeTraverseFunc,
    mut data: gpointer,
) -> gboolean {
    if !(*node).children.is_null() {
        depth = depth.wrapping_sub(1);
        if depth != 0 {
            let mut child: *mut GNode = ::core::ptr::null_mut::<GNode>();
            child = (*node).children;
            while !child.is_null() {
                let mut current: *mut GNode = ::core::ptr::null_mut::<GNode>();
                current = child;
                child = (*current).next;
                if safe_c2rust_g_node_depth_traverse_post_order(current, flags, depth, func, data)
                    != 0
                {
                    return TRUE;
                }
            }
        }
        if flags as ::core::ffi::c_uint
            & G_TRAVERSE_NON_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && func.expect("non-null function pointer")(node, data) != 0
        {
            return TRUE;
        }
    } else if flags as ::core::ffi::c_uint
        & G_TRAVERSE_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && func.expect("non-null function pointer")(node, data) != 0
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_node_traverse_in_order(
    mut node: *mut GNode,
    mut flags: GTraverseFlags,
    mut func: GNodeTraverseFunc,
    mut data: gpointer,
) -> gboolean {
    if !(*node).children.is_null() {
        let mut child: *mut GNode = ::core::ptr::null_mut::<GNode>();
        let mut current: *mut GNode = ::core::ptr::null_mut::<GNode>();
        child = (*node).children;
        current = child;
        child = (*current).next;
        if safe_c2rust_g_node_traverse_in_order(current, flags, func, data) != 0 {
            return TRUE;
        }
        if flags as ::core::ffi::c_uint
            & G_TRAVERSE_NON_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && func.expect("non-null function pointer")(node, data) != 0
        {
            return TRUE;
        }
        while !child.is_null() {
            current = child;
            child = (*current).next;
            if safe_c2rust_g_node_traverse_in_order(current, flags, func, data) != 0 {
                return TRUE;
            }
        }
    } else if flags as ::core::ffi::c_uint
        & G_TRAVERSE_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && func.expect("non-null function pointer")(node, data) != 0
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_node_depth_traverse_in_order(
    mut node: *mut GNode,
    mut flags: GTraverseFlags,
    mut depth: guint,
    mut func: GNodeTraverseFunc,
    mut data: gpointer,
) -> gboolean {
    if !(*node).children.is_null() {
        depth = depth.wrapping_sub(1);
        if depth != 0 {
            let mut child: *mut GNode = ::core::ptr::null_mut::<GNode>();
            let mut current: *mut GNode = ::core::ptr::null_mut::<GNode>();
            child = (*node).children;
            current = child;
            child = (*current).next;
            if safe_c2rust_g_node_depth_traverse_in_order(current, flags, depth, func, data) != 0 {
                return TRUE;
            }
            if flags as ::core::ffi::c_uint
                & G_TRAVERSE_NON_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
                && func.expect("non-null function pointer")(node, data) != 0
            {
                return TRUE;
            }
            while !child.is_null() {
                current = child;
                child = (*current).next;
                if safe_c2rust_g_node_depth_traverse_in_order(current, flags, depth, func, data)
                    != 0
                {
                    return TRUE;
                }
            }
        } else if flags as ::core::ffi::c_uint
            & G_TRAVERSE_NON_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && func.expect("non-null function pointer")(node, data) != 0
        {
            return TRUE;
        }
    } else if flags as ::core::ffi::c_uint
        & G_TRAVERSE_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
        && func.expect("non-null function pointer")(node, data) != 0
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_node_traverse_level(
    mut node: *mut GNode,
    mut flags: GTraverseFlags,
    mut level: guint,
    mut func: GNodeTraverseFunc,
    mut data: gpointer,
    mut more_levels: *mut gboolean,
) -> gboolean {
    if level == 0 as guint {
        if !(*node).children.is_null() {
            *more_levels = TRUE as gboolean;
            return (flags as ::core::ffi::c_uint
                & G_TRAVERSE_NON_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
                && func.expect("non-null function pointer")(node, data) != 0)
                as ::core::ffi::c_int;
        } else {
            return (flags as ::core::ffi::c_uint
                & G_TRAVERSE_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
                && func.expect("non-null function pointer")(node, data) != 0)
                as ::core::ffi::c_int;
        }
    } else {
        node = (*node).children;
        while !node.is_null() {
            if safe_c2rust_g_node_traverse_level(
                node,
                flags,
                level.wrapping_sub(1 as guint),
                func,
                data,
                more_levels,
            ) != 0
            {
                return TRUE;
            }
            node = (*node).next;
        }
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_node_depth_traverse_level(
    mut node: *mut GNode,
    mut flags: GTraverseFlags,
    mut depth: gint,
    mut func: GNodeTraverseFunc,
    mut data: gpointer,
) -> gboolean {
    let mut level: guint = 0;
    let mut more_levels: gboolean = 0;
    level = 0 as guint;
    while depth < 0 as ::core::ffi::c_int || level != depth as guint {
        more_levels = FALSE as gboolean;
        if safe_c2rust_g_node_traverse_level(node, flags, level, func, data, &raw mut more_levels)
            != 0
        {
            return TRUE;
        }
        if more_levels == 0 {
            break;
        }
        level = level.wrapping_add(1);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_traverse(
    mut root: *mut GNode,
    mut order: GTraverseType,
    mut flags: GTraverseFlags,
    mut depth: gint,
    mut func: GNodeTraverseFunc,
    mut data: gpointer,
) {
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if !root.is_null() {
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
            b"root != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if order as ::core::ffi::c_uint
            <= G_LEVEL_ORDER as ::core::ffi::c_int as ::core::ffi::c_uint
        {
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
            b"order <= G_LEVEL_ORDER\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            <= G_TRAVERSE_MASK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
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
            b"flags <= G_TRAVERSE_MASK\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if depth == -(1 as ::core::ffi::c_int) || depth > 0 as ::core::ffi::c_int {
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
            b"depth == -1 || depth > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    match order as ::core::ffi::c_uint {
        1 => {
            if depth < 0 as ::core::ffi::c_int {
                safe_c2rust_g_node_traverse_pre_order(root, flags, func, data);
            } else {
                safe_c2rust_g_node_depth_traverse_pre_order(
                    root,
                    flags,
                    depth as guint,
                    func,
                    data,
                );
            }
        }
        2 => {
            if depth < 0 as ::core::ffi::c_int {
                safe_c2rust_g_node_traverse_post_order(root, flags, func, data);
            } else {
                safe_c2rust_g_node_depth_traverse_post_order(
                    root,
                    flags,
                    depth as guint,
                    func,
                    data,
                );
            }
        }
        0 => {
            if depth < 0 as ::core::ffi::c_int {
                safe_c2rust_g_node_traverse_in_order(root, flags, func, data);
            } else {
                safe_c2rust_g_node_depth_traverse_in_order(root, flags, depth as guint, func, data);
            }
        }
        3 => {
            safe_c2rust_g_node_depth_traverse_level(root, flags, depth, func, data);
        }
        _ => {}
    };
}
unsafe extern "C" fn safe_c2rust_g_node_find_func(
    mut node: *mut GNode,
    mut data: gpointer,
) -> gboolean {
    let mut d: *mut gpointer = data as *mut gpointer;
    if *d != (*node).data {
        return FALSE;
    }
    d = d.offset(1);
    *d = node as gpointer;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_find(
    mut root: *mut GNode,
    mut order: GTraverseType,
    mut flags: GTraverseFlags,
    mut data: gpointer,
) -> *mut GNode {
    let mut d: [gpointer; 2] = [::core::ptr::null_mut::<::core::ffi::c_void>(); 2];
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !root.is_null() {
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
            b"root != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GNode>();
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if order as ::core::ffi::c_uint
            <= G_LEVEL_ORDER as ::core::ffi::c_int as ::core::ffi::c_uint
        {
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
            b"order <= G_LEVEL_ORDER\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GNode>();
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            <= G_TRAVERSE_MASK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
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
            b"flags <= G_TRAVERSE_MASK\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GNode>();
    }
    d[0 as ::core::ffi::c_int as usize] = data;
    d[1 as ::core::ffi::c_int as usize] = NULL as gpointer;
    safe_c2rust_g_node_traverse(
        root,
        order,
        flags,
        -(1 as gint),
        Some(
            safe_c2rust_g_node_find_func as unsafe extern "C" fn(*mut GNode, gpointer) -> gboolean,
        ),
        &raw mut d as *mut gpointer as gpointer,
    );
    return d[1 as ::core::ffi::c_int as usize] as *mut GNode;
}
unsafe extern "C" fn safe_c2rust_g_node_count_func(
    mut node: *mut GNode,
    mut flags: GTraverseFlags,
    mut n: *mut guint,
) {
    if !(*node).children.is_null() {
        let mut child: *mut GNode = ::core::ptr::null_mut::<GNode>();
        if flags as ::core::ffi::c_uint
            & G_TRAVERSE_NON_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            *n = (*n).wrapping_add(1);
        }
        child = (*node).children;
        while !child.is_null() {
            safe_c2rust_g_node_count_func(child, flags, n);
            child = (*child).next;
        }
    } else if flags as ::core::ffi::c_uint
        & G_TRAVERSE_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        *n = (*n).wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_n_nodes(
    mut root: *mut GNode,
    mut flags: GTraverseFlags,
) -> guint {
    let mut n: guint = 0 as guint;
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !root.is_null() {
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
            b"root != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            <= G_TRAVERSE_MASK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
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
            b"flags <= G_TRAVERSE_MASK\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    safe_c2rust_g_node_count_func(root, flags, &raw mut n);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_last_child(mut node: *mut GNode) -> *mut GNode {
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GNode>();
    }
    node = (*node).children;
    if !node.is_null() {
        while !(*node).next.is_null() {
            node = (*node).next;
        }
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_nth_child(
    mut node: *mut GNode,
    mut n: guint,
) -> *mut GNode {
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GNode>();
    }
    node = (*node).children;
    if !node.is_null() {
        loop {
            let fresh0 = n;
            n = n.wrapping_sub(1);
            if !(fresh0 > 0 as guint && !node.is_null()) {
                break;
            }
            node = (*node).next;
        }
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_n_children(mut node: *mut GNode) -> guint {
    let mut n: guint = 0 as guint;
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as guint;
    }
    node = (*node).children;
    while !node.is_null() {
        n = n.wrapping_add(1);
        node = (*node).next;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_find_child(
    mut node: *mut GNode,
    mut flags: GTraverseFlags,
    mut data: gpointer,
) -> *mut GNode {
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GNode>();
    }
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            <= G_TRAVERSE_MASK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
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
            b"flags <= G_TRAVERSE_MASK\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GNode>();
    }
    node = (*node).children;
    while !node.is_null() {
        if (*node).data == data {
            if (*node).children.is_null() {
                if flags as ::core::ffi::c_uint
                    & G_TRAVERSE_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    return node;
                }
            } else if flags as ::core::ffi::c_uint
                & G_TRAVERSE_NON_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                return node;
            }
        }
        node = (*node).next;
    }
    return ::core::ptr::null_mut::<GNode>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_child_position(
    mut node: *mut GNode,
    mut child: *mut GNode,
) -> gint {
    let mut n: guint = 0 as guint;
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if !child.is_null() {
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
            b"child != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if (*child).parent == node {
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
            b"child->parent == node\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    node = (*node).children;
    while !node.is_null() {
        if node == child {
            return n as gint;
        }
        n = n.wrapping_add(1);
        node = (*node).next;
    }
    return -(1 as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_child_index(
    mut node: *mut GNode,
    mut data: gpointer,
) -> gint {
    let mut n: guint = 0 as guint;
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as gint);
    }
    node = (*node).children;
    while !node.is_null() {
        if (*node).data == data {
            return n as gint;
        }
        n = n.wrapping_add(1);
        node = (*node).next;
    }
    return -(1 as gint);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_first_sibling(mut node: *mut GNode) -> *mut GNode {
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GNode>();
    }
    if !(*node).parent.is_null() {
        return (*(*node).parent).children;
    }
    while !(*node).prev.is_null() {
        node = (*node).prev;
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_last_sibling(mut node: *mut GNode) -> *mut GNode {
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GNode>();
    }
    while !(*node).next.is_null() {
        node = (*node).next;
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_node_children_foreach(
    mut node: *mut GNode,
    mut flags: GTraverseFlags,
    mut func: GNodeForeachFunc,
    mut data: gpointer,
) {
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if flags as ::core::ffi::c_uint
            <= G_TRAVERSE_MASK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
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
            b"flags <= G_TRAVERSE_MASK\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    node = (*node).children;
    while !node.is_null() {
        let mut current: *mut GNode = ::core::ptr::null_mut::<GNode>();
        current = node;
        node = (*current).next;
        if (*current).children.is_null() {
            if flags as ::core::ffi::c_uint
                & G_TRAVERSE_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                func.expect("non-null function pointer")(current, data);
            }
        } else if flags as ::core::ffi::c_uint
            & G_TRAVERSE_NON_LEAFS as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            func.expect("non-null function pointer")(current, data);
        }
    }
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_node_unlink\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
