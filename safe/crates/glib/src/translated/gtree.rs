extern "C" {
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
pub type gint8 = ::core::ffi::c_schar;
pub type guint8 = ::core::ffi::c_uchar;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
pub type GCompareDataFunc =
    Option<unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint>;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GTraverseType = ::core::ffi::c_uint;
pub const G_LEVEL_ORDER: GTraverseType = 3;
pub const G_POST_ORDER: GTraverseType = 2;
pub const G_PRE_ORDER: GTraverseType = 1;
pub const G_IN_ORDER: GTraverseType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTree {
    pub root: *mut GTreeNode,
    pub key_compare: GCompareDataFunc,
    pub key_destroy_func: GDestroyNotify,
    pub value_destroy_func: GDestroyNotify,
    pub key_compare_data: gpointer,
    pub nnodes: guint,
    pub ref_count: gint,
}
pub type GTreeNode = _GTreeNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTreeNode {
    pub key: gpointer,
    pub value: gpointer,
    pub left: *mut GTreeNode,
    pub right: *mut GTreeNode,
    pub balance: gint8,
    pub left_child: guint8,
    pub right_child: guint8,
}
pub type GTree = _GTree;
pub type GTraverseFunc = Option<unsafe extern "C" fn(gpointer, gpointer, gpointer) -> gboolean>;
pub type GTraverseNodeFunc = Option<unsafe extern "C" fn(*mut GTreeNode, gpointer) -> gboolean>;
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
unsafe extern "C" fn safe_c2rust_g_tree_node_new(
    mut key: gpointer,
    mut value: gpointer,
) -> *mut GTreeNode {
    let mut node: *mut GTreeNode =
        g_slice_alloc(::core::mem::size_of::<GTreeNode>() as gsize) as *mut GTreeNode;
    (*node).balance = 0 as gint8;
    (*node).left = ::core::ptr::null_mut::<GTreeNode>();
    (*node).right = ::core::ptr::null_mut::<GTreeNode>();
    (*node).left_child = FALSE as guint8;
    (*node).right_child = FALSE as guint8;
    (*node).key = key;
    (*node).value = value;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_new(mut key_compare_func: GCompareFunc) -> *mut GTree {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if key_compare_func.is_some() {
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
            b"key_compare_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTree>();
    }
    return safe_c2rust_g_tree_new_full(
        ::core::mem::transmute::<GCompareFunc, GCompareDataFunc>(key_compare_func),
        NULL,
        None,
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_new_with_data(
    mut key_compare_func: GCompareDataFunc,
    mut key_compare_data: gpointer,
) -> *mut GTree {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if key_compare_func.is_some() {
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
            b"key_compare_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTree>();
    }
    return safe_c2rust_g_tree_new_full(key_compare_func, key_compare_data, None, None);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_new_full(
    mut key_compare_func: GCompareDataFunc,
    mut key_compare_data: gpointer,
    mut key_destroy_func: GDestroyNotify,
    mut value_destroy_func: GDestroyNotify,
) -> *mut GTree {
    let mut tree: *mut GTree = ::core::ptr::null_mut::<GTree>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if key_compare_func.is_some() {
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
            b"key_compare_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTree>();
    }
    tree = g_slice_alloc(::core::mem::size_of::<GTree>() as gsize) as *mut GTree;
    (*tree).root = ::core::ptr::null_mut::<GTreeNode>();
    (*tree).key_compare = key_compare_func;
    (*tree).key_destroy_func = key_destroy_func;
    (*tree).value_destroy_func = value_destroy_func;
    (*tree).key_compare_data = key_compare_data;
    (*tree).nnodes = 0 as guint;
    (*tree).ref_count = 1 as ::core::ffi::c_int as gint;
    return tree;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_node_first(mut tree: *mut GTree) -> *mut GTreeNode {
    let mut tmp: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    if (*tree).root.is_null() {
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    tmp = (*tree).root;
    while (*tmp).left_child != 0 {
        tmp = (*tmp).left;
    }
    return tmp;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_node_last(mut tree: *mut GTree) -> *mut GTreeNode {
    let mut tmp: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    if (*tree).root.is_null() {
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    tmp = (*tree).root;
    while (*tmp).right_child != 0 {
        tmp = (*tmp).right;
    }
    return tmp;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_node_previous(
    mut node: *mut GTreeNode,
) -> *mut GTreeNode {
    let mut tmp: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    tmp = (*node).left;
    if (*node).left_child != 0 {
        while (*tmp).right_child != 0 {
            tmp = (*tmp).right;
        }
    }
    return tmp;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_node_next(mut node: *mut GTreeNode) -> *mut GTreeNode {
    let mut tmp: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
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
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    tmp = (*node).right;
    if (*node).right_child != 0 {
        while (*tmp).left_child != 0 {
            tmp = (*tmp).left;
        }
    }
    return tmp;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_remove_all(mut tree: *mut GTree) {
    let mut node: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    let mut next: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    node = safe_c2rust_g_tree_node_first(tree);
    while !node.is_null() {
        next = safe_c2rust_g_tree_node_next(node);
        if (*tree).key_destroy_func.is_some() {
            (*tree).key_destroy_func.expect("non-null function pointer")((*node).key);
        }
        if (*tree).value_destroy_func.is_some() {
            (*tree)
                .value_destroy_func
                .expect("non-null function pointer")((*node).value);
        }
        g_slice_free1(
            ::core::mem::size_of::<GTreeNode>() as gsize,
            node as gpointer,
        );
        node = next;
    }
    (*tree).root = ::core::ptr::null_mut::<GTreeNode>();
    (*tree).nnodes = 0 as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_ref(mut tree: *mut GTree) -> *mut GTree {
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTree>();
    }
    if 0 as ::core::ffi::c_int != 0 {
        (*tree).ref_count;
        (*tree).ref_count;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(&raw mut (*tree).ref_count, 1 as ::core::ffi::c_int);
    return tree;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_unref(mut tree: *mut GTree) {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            (*tree).ref_count;
            (*tree).ref_count;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(
            &raw mut (*tree).ref_count,
            1 as ::core::ffi::c_int,
        ) == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    }) != 0
    {
        safe_c2rust_g_tree_remove_all(tree);
        g_slice_free1(::core::mem::size_of::<GTree>() as gsize, tree as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_destroy(mut tree: *mut GTree) {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_tree_remove_all(tree);
    safe_c2rust_g_tree_unref(tree);
}
unsafe extern "C" fn safe_c2rust_g_tree_insert_replace_node_internal(
    mut tree: *mut GTree,
    mut key: gpointer,
    mut value: gpointer,
    mut replace: gboolean,
    mut null_ret_ok: gboolean,
) -> *mut GTreeNode {
    let mut node: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    node = safe_c2rust_g_tree_insert_internal(tree, key, value, replace, null_ret_ok);
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_insert_node(
    mut tree: *mut GTree,
    mut key: gpointer,
    mut value: gpointer,
) -> *mut GTreeNode {
    return safe_c2rust_g_tree_insert_replace_node_internal(tree, key, value, FALSE, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_insert(
    mut tree: *mut GTree,
    mut key: gpointer,
    mut value: gpointer,
) {
    safe_c2rust_g_tree_insert_replace_node_internal(tree, key, value, FALSE, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_replace_node(
    mut tree: *mut GTree,
    mut key: gpointer,
    mut value: gpointer,
) -> *mut GTreeNode {
    return safe_c2rust_g_tree_insert_replace_node_internal(tree, key, value, TRUE, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_replace(
    mut tree: *mut GTree,
    mut key: gpointer,
    mut value: gpointer,
) {
    safe_c2rust_g_tree_insert_replace_node_internal(tree, key, value, TRUE, FALSE);
}
unsafe extern "C" fn safe_c2rust_g_tree_nnodes_inc_checked(
    mut tree: *mut GTree,
    mut overflow_fatal: gboolean,
) -> gboolean {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if (*tree).nnodes
            == (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint)
        {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
        if overflow_fatal != 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_ERROR,
                b"Incrementing GTree nnodes counter would overflow\0" as *const u8 as *const gchar,
            );
            loop {}
        }
        return FALSE;
    }
    (*tree).nnodes = (*tree).nnodes.wrapping_add(1);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_g_tree_insert_internal(
    mut tree: *mut GTree,
    mut key: gpointer,
    mut value: gpointer,
    mut replace: gboolean,
    mut null_ret_ok: gboolean,
) -> *mut GTreeNode {
    let mut node: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    let mut retnode: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    let mut path: [*mut GTreeNode; 40] = [::core::ptr::null_mut::<GTreeNode>(); 40];
    let mut idx: ::core::ffi::c_int = 0;
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    if (*tree).root.is_null() {
        (*tree).root = safe_c2rust_g_tree_node_new(key, value);
        (*tree).nnodes = (*tree).nnodes.wrapping_add(1);
        return (*tree).root;
    }
    idx = 0 as ::core::ffi::c_int;
    let fresh0 = idx;
    idx = idx + 1;
    path[fresh0 as usize] = ::core::ptr::null_mut::<GTreeNode>();
    node = (*tree).root;
    loop {
        let mut cmp: ::core::ffi::c_int = (*tree).key_compare.expect("non-null function pointer")(
            key as gconstpointer,
            (*node).key as gconstpointer,
            (*tree).key_compare_data,
        ) as ::core::ffi::c_int;
        if cmp == 0 as ::core::ffi::c_int {
            if (*tree).value_destroy_func.is_some() {
                (*tree)
                    .value_destroy_func
                    .expect("non-null function pointer")((*node).value);
            }
            (*node).value = value;
            if replace != 0 {
                if (*tree).key_destroy_func.is_some() {
                    (*tree).key_destroy_func.expect("non-null function pointer")((*node).key);
                }
                (*node).key = key;
            } else if (*tree).key_destroy_func.is_some() {
                (*tree).key_destroy_func.expect("non-null function pointer")(key);
            }
            return node;
        } else if cmp < 0 as ::core::ffi::c_int {
            if (*node).left_child != 0 {
                let fresh1 = idx;
                idx = idx + 1;
                path[fresh1 as usize] = node;
                node = (*node).left;
            } else {
                let mut child: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
                if safe_c2rust_g_tree_nnodes_inc_checked(
                    tree,
                    (null_ret_ok == 0) as ::core::ffi::c_int,
                ) == 0
                {
                    return ::core::ptr::null_mut::<GTreeNode>();
                }
                child = safe_c2rust_g_tree_node_new(key, value);
                (*child).left = (*node).left;
                (*child).right = node;
                (*node).left = child;
                (*node).left_child = TRUE as guint8;
                (*node).balance =
                    ((*node).balance as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as gint8;
                retnode = child;
                break;
            }
        } else if (*node).right_child != 0 {
            let fresh2 = idx;
            idx = idx + 1;
            path[fresh2 as usize] = node;
            node = (*node).right;
        } else {
            let mut child_0: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
            if safe_c2rust_g_tree_nnodes_inc_checked(tree, (null_ret_ok == 0) as ::core::ffi::c_int)
                == 0
            {
                return ::core::ptr::null_mut::<GTreeNode>();
            }
            child_0 = safe_c2rust_g_tree_node_new(key, value);
            (*child_0).right = (*node).right;
            (*child_0).left = node;
            (*node).right = child_0;
            (*node).right_child = TRUE as guint8;
            (*node).balance =
                ((*node).balance as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint8;
            retnode = child_0;
            break;
        }
    }
    loop {
        idx -= 1;
        let mut bparent: *mut GTreeNode = path[idx as usize];
        let mut left_node: gboolean =
            (!bparent.is_null() && node == (*bparent).left) as ::core::ffi::c_int;
        if ({
            let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
            if bparent.is_null() || (*bparent).left == node || (*bparent).right == node {
                _g_boolean_var_22 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_22 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_22
        }) as ::core::ffi::c_long
            != 0
        {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gtree.c\0" as *const u8 as *const ::core::ffi::c_char,
                686 as ::core::ffi::c_int,
                G_STRFUNC,
                b"!bparent || bparent->left == node || bparent->right == node\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if ((*node).balance as ::core::ffi::c_int) < -(1 as ::core::ffi::c_int)
            || (*node).balance as ::core::ffi::c_int > 1 as ::core::ffi::c_int
        {
            node = safe_c2rust_g_tree_node_balance(node);
            if bparent.is_null() {
                (*tree).root = node;
            } else if left_node != 0 {
                (*bparent).left = node;
            } else {
                (*bparent).right = node;
            }
        }
        if (*node).balance as ::core::ffi::c_int == 0 as ::core::ffi::c_int || bparent.is_null() {
            break;
        }
        if left_node != 0 {
            (*bparent).balance =
                ((*bparent).balance as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as gint8;
        } else {
            (*bparent).balance =
                ((*bparent).balance as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint8;
        }
        node = bparent;
    }
    return retnode;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_remove(
    mut tree: *mut GTree,
    mut key: gconstpointer,
) -> gboolean {
    let mut removed: gboolean = 0;
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    removed = safe_c2rust_g_tree_remove_internal(tree, key, FALSE);
    return removed;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_steal(
    mut tree: *mut GTree,
    mut key: gconstpointer,
) -> gboolean {
    let mut removed: gboolean = 0;
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    removed = safe_c2rust_g_tree_remove_internal(tree, key, TRUE);
    return removed;
}
unsafe extern "C" fn safe_c2rust_g_tree_remove_internal(
    mut tree: *mut GTree,
    mut key: gconstpointer,
    mut steal: gboolean,
) -> gboolean {
    let mut node: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    let mut parent: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    let mut balance: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    let mut path: [*mut GTreeNode; 40] = [::core::ptr::null_mut::<GTreeNode>(); 40];
    let mut idx: ::core::ffi::c_int = 0;
    let mut left_node: gboolean = 0;
    if ({
        let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if (*tree).root.is_null() {
        return FALSE;
    }
    idx = 0 as ::core::ffi::c_int;
    let fresh3 = idx;
    idx = idx + 1;
    path[fresh3 as usize] = ::core::ptr::null_mut::<GTreeNode>();
    node = (*tree).root;
    loop {
        let mut cmp: ::core::ffi::c_int = (*tree).key_compare.expect("non-null function pointer")(
            key,
            (*node).key as gconstpointer,
            (*tree).key_compare_data,
        ) as ::core::ffi::c_int;
        if cmp == 0 as ::core::ffi::c_int {
            break;
        }
        if cmp < 0 as ::core::ffi::c_int {
            if (*node).left_child == 0 {
                return FALSE;
            }
            let fresh4 = idx;
            idx = idx + 1;
            path[fresh4 as usize] = node;
            node = (*node).left;
        } else {
            if (*node).right_child == 0 {
                return FALSE;
            }
            let fresh5 = idx;
            idx = idx + 1;
            path[fresh5 as usize] = node;
            node = (*node).right;
        }
    }
    idx -= 1;
    parent = path[idx as usize];
    balance = parent;
    if ({
        let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
        if parent.is_null() || (*parent).left == node || (*parent).right == node {
            _g_boolean_var_26 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_26 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_26
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/glib/gtree.c\0" as *const u8 as *const ::core::ffi::c_char,
            827 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!parent || parent->left == node || parent->right == node\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    left_node = (!parent.is_null() && node == (*parent).left) as ::core::ffi::c_int as gboolean;
    if (*node).left_child == 0 {
        if (*node).right_child == 0 {
            if parent.is_null() {
                (*tree).root = ::core::ptr::null_mut::<GTreeNode>();
            } else if left_node != 0 {
                (*parent).left_child = FALSE as guint8;
                (*parent).left = (*node).left;
                (*parent).balance =
                    ((*parent).balance as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint8;
            } else {
                (*parent).right_child = FALSE as guint8;
                (*parent).right = (*node).right;
                (*parent).balance =
                    ((*parent).balance as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as gint8;
            }
        } else {
            let mut tmp: *mut GTreeNode = safe_c2rust_g_tree_node_next(node);
            (*tmp).left = (*node).left;
            if parent.is_null() {
                (*tree).root = (*node).right;
            } else if left_node != 0 {
                (*parent).left = (*node).right;
                (*parent).balance =
                    ((*parent).balance as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint8;
            } else {
                (*parent).right = (*node).right;
                (*parent).balance =
                    ((*parent).balance as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as gint8;
            }
        }
    } else if (*node).right_child == 0 {
        let mut tmp_0: *mut GTreeNode = safe_c2rust_g_tree_node_previous(node);
        (*tmp_0).right = (*node).right;
        if parent.is_null() {
            (*tree).root = (*node).left;
        } else if left_node != 0 {
            (*parent).left = (*node).left;
            (*parent).balance =
                ((*parent).balance as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint8;
        } else {
            (*parent).right = (*node).left;
            (*parent).balance =
                ((*parent).balance as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as gint8;
        }
    } else {
        let mut prev: *mut GTreeNode = (*node).left;
        let mut next: *mut GTreeNode = (*node).right;
        let mut nextp: *mut GTreeNode = node;
        let mut old_idx: ::core::ffi::c_int = idx + 1 as ::core::ffi::c_int;
        idx += 1;
        while (*next).left_child != 0 {
            nextp = next;
            idx += 1;
            path[idx as usize] = nextp;
            next = (*next).left;
        }
        path[old_idx as usize] = next;
        balance = path[idx as usize];
        if nextp != node {
            if (*next).right_child != 0 {
                (*nextp).left = (*next).right;
            } else {
                (*nextp).left_child = FALSE as guint8;
            }
            (*nextp).balance =
                ((*nextp).balance as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint8;
            (*next).right_child = TRUE as guint8;
            (*next).right = (*node).right;
        } else {
            (*node).balance =
                ((*node).balance as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as gint8;
        }
        while (*prev).right_child != 0 {
            prev = (*prev).right;
        }
        (*prev).right = next;
        (*next).left_child = TRUE as guint8;
        (*next).left = (*node).left;
        (*next).balance = (*node).balance;
        if parent.is_null() {
            (*tree).root = next;
        } else if left_node != 0 {
            (*parent).left = next;
        } else {
            (*parent).right = next;
        }
    }
    if !balance.is_null() {
        loop {
            idx -= 1;
            let mut bparent: *mut GTreeNode = path[idx as usize];
            if ({
                let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
                if bparent.is_null() || (*bparent).left == balance || (*bparent).right == balance {
                    _g_boolean_var_27 = 1 as ::core::ffi::c_int;
                } else {
                    _g_boolean_var_27 = 0 as ::core::ffi::c_int;
                }
                _g_boolean_var_27
            }) as ::core::ffi::c_long
                != 0
            {
            } else {
                g_assertion_message_expr(
                    G_LOG_DOMAIN.as_ptr(),
                    b"../original/glib/gtree.c\0" as *const u8 as *const ::core::ffi::c_char,
                    946 as ::core::ffi::c_int,
                    G_STRFUNC,
                    b"!bparent || bparent->left == balance || bparent->right == balance\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
            left_node = (!bparent.is_null() && balance == (*bparent).left) as ::core::ffi::c_int
                as gboolean;
            if ((*balance).balance as ::core::ffi::c_int) < -(1 as ::core::ffi::c_int)
                || (*balance).balance as ::core::ffi::c_int > 1 as ::core::ffi::c_int
            {
                balance = safe_c2rust_g_tree_node_balance(balance);
                if bparent.is_null() {
                    (*tree).root = balance;
                } else if left_node != 0 {
                    (*bparent).left = balance;
                } else {
                    (*bparent).right = balance;
                }
            }
            if (*balance).balance as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                || bparent.is_null()
            {
                break;
            }
            if left_node != 0 {
                (*bparent).balance =
                    ((*bparent).balance as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint8;
            } else {
                (*bparent).balance =
                    ((*bparent).balance as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as gint8;
            }
            balance = bparent;
        }
    }
    if steal == 0 {
        if (*tree).key_destroy_func.is_some() {
            (*tree).key_destroy_func.expect("non-null function pointer")((*node).key);
        }
        if (*tree).value_destroy_func.is_some() {
            (*tree)
                .value_destroy_func
                .expect("non-null function pointer")((*node).value);
        }
    }
    g_slice_free1(
        ::core::mem::size_of::<GTreeNode>() as gsize,
        node as gpointer,
    );
    (*tree).nnodes = (*tree).nnodes.wrapping_sub(1);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_node_key(mut node: *mut GTreeNode) -> gpointer {
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*node).key;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_node_value(mut node: *mut GTreeNode) -> gpointer {
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if !node.is_null() {
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
            b"node != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return (*node).value;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_lookup_node(
    mut tree: *mut GTree,
    mut key: gconstpointer,
) -> *mut GTreeNode {
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    return safe_c2rust_g_tree_find_node(tree, key);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_lookup(
    mut tree: *mut GTree,
    mut key: gconstpointer,
) -> gpointer {
    let mut node: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    node = safe_c2rust_g_tree_lookup_node(tree, key);
    return if !node.is_null() { (*node).value } else { NULL };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_lookup_extended(
    mut tree: *mut GTree,
    mut lookup_key: gconstpointer,
    mut orig_key: *mut gpointer,
    mut value: *mut gpointer,
) -> gboolean {
    let mut node: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    node = safe_c2rust_g_tree_find_node(tree, lookup_key);
    if !node.is_null() {
        if !orig_key.is_null() {
            *orig_key = (*node).key;
        }
        if !value.is_null() {
            *value = (*node).value;
        }
        return TRUE;
    } else {
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_foreach(
    mut tree: *mut GTree,
    mut func: GTraverseFunc,
    mut user_data: gpointer,
) {
    let mut node: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*tree).root.is_null() {
        return;
    }
    node = safe_c2rust_g_tree_node_first(tree);
    while !node.is_null() {
        if Some(func.expect("non-null function pointer")).expect("non-null function pointer")(
            (*node).key,
            (*node).value,
            user_data,
        ) != 0
        {
            break;
        }
        node = safe_c2rust_g_tree_node_next(node);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_foreach_node(
    mut tree: *mut GTree,
    mut func: GTraverseNodeFunc,
    mut user_data: gpointer,
) {
    let mut node: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*tree).root.is_null() {
        return;
    }
    node = safe_c2rust_g_tree_node_first(tree);
    while !node.is_null() {
        if Some(func.expect("non-null function pointer")).expect("non-null function pointer")(
            node, user_data,
        ) != 0
        {
            break;
        }
        node = safe_c2rust_g_tree_node_next(node);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_traverse(
    mut tree: *mut GTree,
    mut traverse_func: GTraverseFunc,
    mut traverse_type: GTraverseType,
    mut user_data: gpointer,
) {
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*tree).root.is_null() {
        return;
    }
    match traverse_type as ::core::ffi::c_uint {
        1 => {
            safe_c2rust_g_tree_node_pre_order((*tree).root, traverse_func, user_data);
        }
        0 => {
            safe_c2rust_g_tree_node_in_order((*tree).root, traverse_func, user_data);
        }
        2 => {
            safe_c2rust_g_tree_node_post_order((*tree).root, traverse_func, user_data);
        }
        3 => {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"g_tree_traverse(): traverse type G_LEVEL_ORDER isn't implemented.\0" as *const u8
                    as *const gchar,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_search_node(
    mut tree: *mut GTree,
    mut search_func: GCompareFunc,
    mut user_data: gconstpointer,
) -> *mut GTreeNode {
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    if (*tree).root.is_null() {
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    return safe_c2rust_g_tree_node_search((*tree).root, search_func, user_data);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_search(
    mut tree: *mut GTree,
    mut search_func: GCompareFunc,
    mut user_data: gconstpointer,
) -> gpointer {
    let mut node: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    node = safe_c2rust_g_tree_search_node(tree, search_func, user_data);
    return if !node.is_null() { (*node).value } else { NULL };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_lower_bound(
    mut tree: *mut GTree,
    mut key: gconstpointer,
) -> *mut GTreeNode {
    let mut node: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    let mut result: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    let mut cmp: gint = 0;
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    node = (*tree).root;
    if node.is_null() {
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    result = ::core::ptr::null_mut::<GTreeNode>();
    loop {
        cmp = (*tree).key_compare.expect("non-null function pointer")(
            key,
            (*node).key as gconstpointer,
            (*tree).key_compare_data,
        );
        if cmp <= 0 as ::core::ffi::c_int {
            result = node;
            if (*node).left_child == 0 {
                return result;
            }
            node = (*node).left;
        } else {
            if (*node).right_child == 0 {
                return result;
            }
            node = (*node).right;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_upper_bound(
    mut tree: *mut GTree,
    mut key: gconstpointer,
) -> *mut GTreeNode {
    let mut node: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    let mut result: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    let mut cmp: gint = 0;
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    node = (*tree).root;
    if node.is_null() {
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    result = ::core::ptr::null_mut::<GTreeNode>();
    loop {
        cmp = (*tree).key_compare.expect("non-null function pointer")(
            key,
            (*node).key as gconstpointer,
            (*tree).key_compare_data,
        );
        if cmp < 0 as ::core::ffi::c_int {
            result = node;
            if (*node).left_child == 0 {
                return result;
            }
            node = (*node).left;
        } else {
            if (*node).right_child == 0 {
                return result;
            }
            node = (*node).right;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_height(mut tree: *mut GTree) -> gint {
    let mut node: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    let mut height: gint = 0;
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    if (*tree).root.is_null() {
        return 0 as gint;
    }
    height = 0 as ::core::ffi::c_int as gint;
    node = (*tree).root;
    loop {
        height += 1 as ::core::ffi::c_int
            + (if (*node).balance as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                (*node).balance as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            });
        if (*node).left_child == 0 {
            return height;
        }
        node = (*node).left;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_tree_nnodes(mut tree: *mut GTree) -> gint {
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if !tree.is_null() {
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
            b"tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gint;
    }
    return (*tree).nnodes as gint;
}
unsafe extern "C" fn safe_c2rust_g_tree_node_balance(mut node: *mut GTreeNode) -> *mut GTreeNode {
    if ((*node).balance as ::core::ffi::c_int) < -(1 as ::core::ffi::c_int) {
        if (*(*node).left).balance as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            (*node).left = safe_c2rust_g_tree_node_rotate_left((*node).left);
        }
        node = safe_c2rust_g_tree_node_rotate_right(node);
    } else if (*node).balance as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
        if ((*(*node).right).balance as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            (*node).right = safe_c2rust_g_tree_node_rotate_right((*node).right);
        }
        node = safe_c2rust_g_tree_node_rotate_left(node);
    }
    return node;
}
unsafe extern "C" fn safe_c2rust_g_tree_find_node(
    mut tree: *mut GTree,
    mut key: gconstpointer,
) -> *mut GTreeNode {
    let mut node: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    let mut cmp: gint = 0;
    node = (*tree).root;
    if node.is_null() {
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    loop {
        cmp = (*tree).key_compare.expect("non-null function pointer")(
            key,
            (*node).key as gconstpointer,
            (*tree).key_compare_data,
        );
        if cmp == 0 as ::core::ffi::c_int {
            return node;
        } else if cmp < 0 as ::core::ffi::c_int {
            if (*node).left_child == 0 {
                return ::core::ptr::null_mut::<GTreeNode>();
            }
            node = (*node).left;
        } else {
            if (*node).right_child == 0 {
                return ::core::ptr::null_mut::<GTreeNode>();
            }
            node = (*node).right;
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_tree_node_pre_order(
    mut node: *mut GTreeNode,
    mut traverse_func: GTraverseFunc,
    mut data: gpointer,
) -> gint {
    if Some(traverse_func.expect("non-null function pointer")).expect("non-null function pointer")(
        (*node).key,
        (*node).value,
        data,
    ) != 0
    {
        return TRUE;
    }
    if (*node).left_child != 0 {
        if safe_c2rust_g_tree_node_pre_order((*node).left, traverse_func, data) != 0 {
            return TRUE;
        }
    }
    if (*node).right_child != 0 {
        if safe_c2rust_g_tree_node_pre_order((*node).right, traverse_func, data) != 0 {
            return TRUE;
        }
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_tree_node_in_order(
    mut node: *mut GTreeNode,
    mut traverse_func: GTraverseFunc,
    mut data: gpointer,
) -> gint {
    if (*node).left_child != 0 {
        if safe_c2rust_g_tree_node_in_order((*node).left, traverse_func, data) != 0 {
            return TRUE;
        }
    }
    if Some(traverse_func.expect("non-null function pointer")).expect("non-null function pointer")(
        (*node).key,
        (*node).value,
        data,
    ) != 0
    {
        return TRUE;
    }
    if (*node).right_child != 0 {
        if safe_c2rust_g_tree_node_in_order((*node).right, traverse_func, data) != 0 {
            return TRUE;
        }
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_tree_node_post_order(
    mut node: *mut GTreeNode,
    mut traverse_func: GTraverseFunc,
    mut data: gpointer,
) -> gint {
    if (*node).left_child != 0 {
        if safe_c2rust_g_tree_node_post_order((*node).left, traverse_func, data) != 0 {
            return TRUE;
        }
    }
    if (*node).right_child != 0 {
        if safe_c2rust_g_tree_node_post_order((*node).right, traverse_func, data) != 0 {
            return TRUE;
        }
    }
    if Some(traverse_func.expect("non-null function pointer")).expect("non-null function pointer")(
        (*node).key,
        (*node).value,
        data,
    ) != 0
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_g_tree_node_search(
    mut node: *mut GTreeNode,
    mut search_func: GCompareFunc,
    mut data: gconstpointer,
) -> *mut GTreeNode {
    let mut dir: gint = 0;
    if node.is_null() {
        return ::core::ptr::null_mut::<GTreeNode>();
    }
    loop {
        dir = Some(search_func.expect("non-null function pointer"))
            .expect("non-null function pointer")((*node).key as gconstpointer, data);
        if dir == 0 as ::core::ffi::c_int {
            return node;
        } else if dir < 0 as ::core::ffi::c_int {
            if (*node).left_child == 0 {
                return ::core::ptr::null_mut::<GTreeNode>();
            }
            node = (*node).left;
        } else {
            if (*node).right_child == 0 {
                return ::core::ptr::null_mut::<GTreeNode>();
            }
            node = (*node).right;
        }
    }
}
unsafe extern "C" fn safe_c2rust_g_tree_node_rotate_left(
    mut node: *mut GTreeNode,
) -> *mut GTreeNode {
    let mut right: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    let mut a_bal: gint = 0;
    let mut b_bal: gint = 0;
    right = (*node).right;
    if (*right).left_child != 0 {
        (*node).right = (*right).left;
    } else {
        (*node).right_child = FALSE as guint8;
        (*right).left_child = TRUE as guint8;
    }
    (*right).left = node;
    a_bal = (*node).balance as gint;
    b_bal = (*right).balance as gint;
    if b_bal <= 0 as ::core::ffi::c_int {
        if a_bal >= 1 as ::core::ffi::c_int {
            (*right).balance = (b_bal as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as gint8;
        } else {
            (*right).balance = (a_bal as ::core::ffi::c_int + b_bal as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int) as gint8;
        }
        (*node).balance = (a_bal as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as gint8;
    } else {
        if a_bal <= b_bal {
            (*right).balance = (a_bal as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as gint8;
        } else {
            (*right).balance = (b_bal as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as gint8;
        }
        (*node).balance = (a_bal as ::core::ffi::c_int
            - b_bal as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as gint8;
    }
    return right;
}
unsafe extern "C" fn safe_c2rust_g_tree_node_rotate_right(
    mut node: *mut GTreeNode,
) -> *mut GTreeNode {
    let mut left: *mut GTreeNode = ::core::ptr::null_mut::<GTreeNode>();
    let mut a_bal: gint = 0;
    let mut b_bal: gint = 0;
    left = (*node).left;
    if (*left).right_child != 0 {
        (*node).left = (*left).right;
    } else {
        (*node).left_child = FALSE as guint8;
        (*left).right_child = TRUE as guint8;
    }
    (*left).right = node;
    a_bal = (*node).balance as gint;
    b_bal = (*left).balance as gint;
    if b_bal <= 0 as ::core::ffi::c_int {
        if b_bal > a_bal {
            (*left).balance = (b_bal as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint8;
        } else {
            (*left).balance = (a_bal as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as gint8;
        }
        (*node).balance = (a_bal as ::core::ffi::c_int - b_bal as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as gint8;
    } else {
        if a_bal <= -(1 as ::core::ffi::c_int) {
            (*left).balance = (b_bal as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint8;
        } else {
            (*left).balance = (a_bal as ::core::ffi::c_int
                + b_bal as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as gint8;
        }
        (*node).balance = (a_bal as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as gint8;
    }
    return left;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_tree_new_full\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
