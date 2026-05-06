use ::c2rust_bitfields;
extern "C" {
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_slice_alloc0(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
}
pub type gsize = ::core::ffi::c_ulong;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GHook {
    pub data: gpointer,
    pub next: *mut GHook,
    pub prev: *mut GHook,
    pub ref_count: guint,
    pub hook_id: gulong,
    pub flags: guint,
    pub func: gpointer,
    pub destroy: GDestroyNotify,
}
pub type GHook = _GHook;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GHookList {
    pub seq_id: gulong,
    #[bitfield(name = "hook_size", ty = "guint", bits = "0..=15")]
    #[bitfield(name = "is_setup", ty = "guint", bits = "16..=16")]
    pub hook_size_is_setup: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
    pub hooks: *mut GHook,
    pub dummy3: gpointer,
    pub finalize_hook: GHookFinalizeFunc,
    pub dummy: [gpointer; 2],
}
pub type GHookFinalizeFunc = Option<unsafe extern "C" fn(*mut GHookList, *mut GHook) -> ()>;
pub type GHookList = _GHookList;
pub type GHookCompareFunc = Option<unsafe extern "C" fn(*mut GHook, *mut GHook) -> gint>;
pub type GHookFindFunc = Option<unsafe extern "C" fn(*mut GHook, gpointer) -> gboolean>;
pub type GHookMarshaller = Option<unsafe extern "C" fn(*mut GHook, gpointer) -> ()>;
pub type GHookCheckMarshaller = Option<unsafe extern "C" fn(*mut GHook, gpointer) -> gboolean>;
pub type GHookFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHookCheckFunc = Option<unsafe extern "C" fn(gpointer) -> gboolean>;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_HOOK_FLAG_MASK: C2RustUnnamed = 15;
pub const G_HOOK_FLAG_IN_CALL: C2RustUnnamed = 2;
pub const G_HOOK_FLAG_ACTIVE: C2RustUnnamed = 1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn safe_c2rust_default_finalize_hook(
    mut hook_list: *mut GHookList,
    mut hook: *mut GHook,
) {
    let mut destroy: GDestroyNotify = (*hook).destroy;
    if destroy.is_some() {
        (*hook).destroy = None;
        destroy.expect("non-null function pointer")((*hook).data);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_list_init(
    mut hook_list: *mut GHookList,
    mut hook_size: guint,
) {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if hook_size as usize >= ::core::mem::size_of::<GHook>() as usize {
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
            b"hook_size >= sizeof (GHook)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*hook_list).seq_id = 1 as gulong;
    (*hook_list).set_hook_size(hook_size as guint);
    (*hook_list).set_is_setup(TRUE as guint as guint);
    (*hook_list).hooks = ::core::ptr::null_mut::<GHook>();
    (*hook_list).dummy3 = NULL as gpointer;
    (*hook_list).finalize_hook = Some(
        safe_c2rust_default_finalize_hook as unsafe extern "C" fn(*mut GHookList, *mut GHook) -> (),
    ) as GHookFinalizeFunc;
    (*hook_list).dummy[0 as ::core::ffi::c_int as usize] = NULL as gpointer;
    (*hook_list).dummy[1 as ::core::ffi::c_int as usize] = NULL as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_list_clear(mut hook_list: *mut GHookList) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*hook_list).is_setup() != 0 {
        let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
        (*hook_list).set_is_setup(FALSE as guint as guint);
        hook = (*hook_list).hooks;
        if !hook.is_null() {
            loop {
                let mut tmp: *mut GHook = ::core::ptr::null_mut::<GHook>();
                safe_c2rust_g_hook_ref(hook_list, hook);
                safe_c2rust_g_hook_destroy_link(hook_list, hook);
                tmp = (*hook).next;
                safe_c2rust_g_hook_unref(hook_list, hook);
                hook = tmp;
                if hook.is_null() {
                    break;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_alloc(mut hook_list: *mut GHookList) -> *mut GHook {
    let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if (*hook_list).is_setup() != 0 {
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
            b"hook_list->is_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    hook = g_slice_alloc0((*hook_list).hook_size() as gsize) as *mut GHook;
    (*hook).data = NULL as gpointer;
    (*hook).next = ::core::ptr::null_mut::<GHook>();
    (*hook).prev = ::core::ptr::null_mut::<GHook>();
    (*hook).flags = G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint;
    (*hook).ref_count = 0 as guint;
    (*hook).hook_id = 0 as gulong;
    (*hook).func = NULL as gpointer;
    (*hook).destroy = None;
    return hook;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_free(
    mut hook_list: *mut GHookList,
    mut hook: *mut GHook,
) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if (*hook_list).is_setup() != 0 {
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
            b"hook_list->is_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !hook.is_null() {
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
            b"hook != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if (*hook).next.is_null()
            && (*hook).prev.is_null()
            && (*hook).hook_id == 0 as gulong
            && (*hook).ref_count == 0 as guint
        {
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
            b"G_HOOK_IS_UNLINKED (hook)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !((*hook).flags & G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint != 0 as guint) {
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
            b"!G_HOOK_IN_CALL (hook)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*hook_list).finalize_hook.is_some() {
        (*hook_list)
            .finalize_hook
            .expect("non-null function pointer")(hook_list, hook);
    }
    g_slice_free1((*hook_list).hook_size() as gsize, hook as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_destroy_link(
    mut hook_list: *mut GHookList,
    mut hook: *mut GHook,
) {
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if !hook.is_null() {
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
            b"hook != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*hook).flags &= !(G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int) as guint;
    if (*hook).hook_id != 0 {
        (*hook).hook_id = 0 as gulong;
        safe_c2rust_g_hook_unref(hook_list, hook);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_destroy(
    mut hook_list: *mut GHookList,
    mut hook_id: gulong,
) -> gboolean {
    let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if hook_id > 0 as gulong {
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
            b"hook_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    hook = safe_c2rust_g_hook_get(hook_list, hook_id);
    if !hook.is_null() {
        safe_c2rust_g_hook_destroy_link(hook_list, hook);
        return TRUE;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_unref(
    mut hook_list: *mut GHookList,
    mut hook: *mut GHook,
) {
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_23: ::core::ffi::c_int = 0;
        if !hook.is_null() {
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
            b"hook != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_24: ::core::ffi::c_int = 0;
        if (*hook).ref_count > 0 as guint {
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
            b"hook->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*hook).ref_count = (*hook).ref_count.wrapping_sub(1);
    if (*hook).ref_count == 0 {
        if ({
            let mut _g_boolean_var_25: ::core::ffi::c_int = 0;
            if (*hook).hook_id == 0 as gulong {
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
                b"hook->hook_id == 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        if ({
            let mut _g_boolean_var_26: ::core::ffi::c_int = 0;
            if !((*hook).flags & G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint != 0 as guint) {
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
                b"!G_HOOK_IN_CALL (hook)\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return;
        }
        if !(*hook).prev.is_null() {
            (*(*hook).prev).next = (*hook).next;
        } else {
            (*hook_list).hooks = (*hook).next;
        }
        if !(*hook).next.is_null() {
            (*(*hook).next).prev = (*hook).prev;
            (*hook).next = ::core::ptr::null_mut::<GHook>();
        }
        (*hook).prev = ::core::ptr::null_mut::<GHook>();
        if (*hook_list).is_setup() == 0 {
            (*hook_list).set_is_setup(TRUE as guint as guint);
            safe_c2rust_g_hook_free(hook_list, hook);
            (*hook_list).set_is_setup(FALSE as guint as guint);
            (*hook_list).hooks.is_null();
        } else {
            safe_c2rust_g_hook_free(hook_list, hook);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_ref(
    mut hook_list: *mut GHookList,
    mut hook: *mut GHook,
) -> *mut GHook {
    if ({
        let mut _g_boolean_var_27: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    if ({
        let mut _g_boolean_var_28: ::core::ffi::c_int = 0;
        if !hook.is_null() {
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
            b"hook != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    if ({
        let mut _g_boolean_var_29: ::core::ffi::c_int = 0;
        if (*hook).ref_count > 0 as guint {
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
            b"hook->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    (*hook).ref_count = (*hook).ref_count.wrapping_add(1);
    return hook;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_prepend(
    mut hook_list: *mut GHookList,
    mut hook: *mut GHook,
) {
    if ({
        let mut _g_boolean_var_30: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    safe_c2rust_g_hook_insert_before(hook_list, (*hook_list).hooks, hook);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_insert_before(
    mut hook_list: *mut GHookList,
    mut sibling: *mut GHook,
    mut hook: *mut GHook,
) {
    if ({
        let mut _g_boolean_var_31: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_32: ::core::ffi::c_int = 0;
        if (*hook_list).is_setup() != 0 {
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
            b"hook_list->is_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_33: ::core::ffi::c_int = 0;
        if !hook.is_null() {
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
            b"hook != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_34: ::core::ffi::c_int = 0;
        if (*hook).next.is_null()
            && (*hook).prev.is_null()
            && (*hook).hook_id == 0 as gulong
            && (*hook).ref_count == 0 as guint
        {
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
            b"G_HOOK_IS_UNLINKED (hook)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_35: ::core::ffi::c_int = 0;
        if (*hook).ref_count == 0 as guint {
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
            b"hook->ref_count == 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let fresh0 = (*hook_list).seq_id;
    (*hook_list).seq_id = (*hook_list).seq_id.wrapping_add(1);
    (*hook).hook_id = fresh0;
    (*hook).ref_count = 1 as guint;
    if !sibling.is_null() {
        if !(*sibling).prev.is_null() {
            (*hook).prev = (*sibling).prev;
            (*(*hook).prev).next = hook;
            (*hook).next = sibling;
            (*sibling).prev = hook;
        } else {
            (*hook_list).hooks = hook;
            (*hook).next = sibling;
            (*sibling).prev = hook;
        }
    } else if !(*hook_list).hooks.is_null() {
        sibling = (*hook_list).hooks;
        while !(*sibling).next.is_null() {
            sibling = (*sibling).next;
        }
        (*hook).prev = sibling;
        (*sibling).next = hook;
    } else {
        (*hook_list).hooks = hook;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_list_invoke(
    mut hook_list: *mut GHookList,
    mut may_recurse: gboolean,
) {
    let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
    if ({
        let mut _g_boolean_var_36: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_37: ::core::ffi::c_int = 0;
        if (*hook_list).is_setup() != 0 {
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
            b"hook_list->is_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    hook = safe_c2rust_g_hook_first_valid(hook_list, may_recurse);
    while !hook.is_null() {
        let mut func: GHookFunc = None;
        let mut was_in_call: gboolean = 0;
        func = ::core::mem::transmute::<gpointer, GHookFunc>((*hook).func);
        was_in_call = ((*hook).flags & G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint
            != 0 as guint) as ::core::ffi::c_int as gboolean;
        (*hook).flags |= G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint;
        func.expect("non-null function pointer")((*hook).data);
        if was_in_call == 0 {
            (*hook).flags &= !(G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int) as guint;
        }
        hook = safe_c2rust_g_hook_next_valid(hook_list, hook, may_recurse);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_list_invoke_check(
    mut hook_list: *mut GHookList,
    mut may_recurse: gboolean,
) {
    let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
    if ({
        let mut _g_boolean_var_38: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_39: ::core::ffi::c_int = 0;
        if (*hook_list).is_setup() != 0 {
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
            b"hook_list->is_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    hook = safe_c2rust_g_hook_first_valid(hook_list, may_recurse);
    while !hook.is_null() {
        let mut func: GHookCheckFunc = None;
        let mut was_in_call: gboolean = 0;
        let mut need_destroy: gboolean = 0;
        func = ::core::mem::transmute::<gpointer, GHookCheckFunc>((*hook).func);
        was_in_call = ((*hook).flags & G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint
            != 0 as guint) as ::core::ffi::c_int as gboolean;
        (*hook).flags |= G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint;
        need_destroy = (func.expect("non-null function pointer")((*hook).data) == 0)
            as ::core::ffi::c_int as gboolean;
        if was_in_call == 0 {
            (*hook).flags &= !(G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int) as guint;
        }
        if need_destroy != 0 {
            safe_c2rust_g_hook_destroy_link(hook_list, hook);
        }
        hook = safe_c2rust_g_hook_next_valid(hook_list, hook, may_recurse);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_list_marshal_check(
    mut hook_list: *mut GHookList,
    mut may_recurse: gboolean,
    mut marshaller: GHookCheckMarshaller,
    mut data: gpointer,
) {
    let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
    if ({
        let mut _g_boolean_var_40: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_41: ::core::ffi::c_int = 0;
        if (*hook_list).is_setup() != 0 {
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
            b"hook_list->is_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_42: ::core::ffi::c_int = 0;
        if marshaller.is_some() {
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
            b"marshaller != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    hook = safe_c2rust_g_hook_first_valid(hook_list, may_recurse);
    while !hook.is_null() {
        let mut was_in_call: gboolean = 0;
        let mut need_destroy: gboolean = 0;
        was_in_call = ((*hook).flags & G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint
            != 0 as guint) as ::core::ffi::c_int as gboolean;
        (*hook).flags |= G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint;
        need_destroy = (marshaller.expect("non-null function pointer")(hook, data) == 0)
            as ::core::ffi::c_int as gboolean;
        if was_in_call == 0 {
            (*hook).flags &= !(G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int) as guint;
        }
        if need_destroy != 0 {
            safe_c2rust_g_hook_destroy_link(hook_list, hook);
        }
        hook = safe_c2rust_g_hook_next_valid(hook_list, hook, may_recurse);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_list_marshal(
    mut hook_list: *mut GHookList,
    mut may_recurse: gboolean,
    mut marshaller: GHookMarshaller,
    mut data: gpointer,
) {
    let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
    if ({
        let mut _g_boolean_var_43: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_44: ::core::ffi::c_int = 0;
        if (*hook_list).is_setup() != 0 {
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
            b"hook_list->is_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_45: ::core::ffi::c_int = 0;
        if marshaller.is_some() {
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
            b"marshaller != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    hook = safe_c2rust_g_hook_first_valid(hook_list, may_recurse);
    while !hook.is_null() {
        let mut was_in_call: gboolean = 0;
        was_in_call = ((*hook).flags & G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint
            != 0 as guint) as ::core::ffi::c_int as gboolean;
        (*hook).flags |= G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint;
        marshaller.expect("non-null function pointer")(hook, data);
        if was_in_call == 0 {
            (*hook).flags &= !(G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int) as guint;
        }
        hook = safe_c2rust_g_hook_next_valid(hook_list, hook, may_recurse);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_first_valid(
    mut hook_list: *mut GHookList,
    mut may_be_in_call: gboolean,
) -> *mut GHook {
    if ({
        let mut _g_boolean_var_46: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    if (*hook_list).is_setup() != 0 {
        let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
        hook = (*hook_list).hooks;
        if !hook.is_null() {
            safe_c2rust_g_hook_ref(hook_list, hook);
            if (*hook).hook_id != 0 as gulong
                && (*hook).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint != 0
                && (may_be_in_call != 0
                    || !((*hook).flags & G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint
                        != 0 as guint))
            {
                return hook;
            } else {
                return safe_c2rust_g_hook_next_valid(hook_list, hook, may_be_in_call);
            }
        }
    }
    return ::core::ptr::null_mut::<GHook>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_next_valid(
    mut hook_list: *mut GHookList,
    mut hook: *mut GHook,
    mut may_be_in_call: gboolean,
) -> *mut GHook {
    let mut ohook: *mut GHook = hook;
    if ({
        let mut _g_boolean_var_47: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    if hook.is_null() {
        return ::core::ptr::null_mut::<GHook>();
    }
    hook = (*hook).next;
    while !hook.is_null() {
        if (*hook).hook_id != 0 as gulong
            && (*hook).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint != 0
            && (may_be_in_call != 0
                || !((*hook).flags & G_HOOK_FLAG_IN_CALL as ::core::ffi::c_int as guint
                    != 0 as guint))
        {
            safe_c2rust_g_hook_ref(hook_list, hook);
            safe_c2rust_g_hook_unref(hook_list, ohook);
            return hook;
        }
        hook = (*hook).next;
    }
    safe_c2rust_g_hook_unref(hook_list, ohook);
    return ::core::ptr::null_mut::<GHook>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_get(
    mut hook_list: *mut GHookList,
    mut hook_id: gulong,
) -> *mut GHook {
    let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
    if ({
        let mut _g_boolean_var_48: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    if ({
        let mut _g_boolean_var_49: ::core::ffi::c_int = 0;
        if hook_id > 0 as gulong {
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
            b"hook_id > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    hook = (*hook_list).hooks;
    while !hook.is_null() {
        if (*hook).hook_id == hook_id {
            return hook;
        }
        hook = (*hook).next;
    }
    return ::core::ptr::null_mut::<GHook>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_find(
    mut hook_list: *mut GHookList,
    mut need_valids: gboolean,
    mut func: GHookFindFunc,
    mut data: gpointer,
) -> *mut GHook {
    let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
    if ({
        let mut _g_boolean_var_50: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    if ({
        let mut _g_boolean_var_51: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    hook = (*hook_list).hooks;
    while !hook.is_null() {
        let mut tmp: *mut GHook = ::core::ptr::null_mut::<GHook>();
        if (*hook).hook_id == 0 {
            hook = (*hook).next;
        } else {
            safe_c2rust_g_hook_ref(hook_list, hook);
            if func.expect("non-null function pointer")(hook, data) != 0
                && (*hook).hook_id != 0
                && (need_valids == 0
                    || (*hook).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint
                        != 0 as guint)
            {
                safe_c2rust_g_hook_unref(hook_list, hook);
                return hook;
            }
            tmp = (*hook).next;
            safe_c2rust_g_hook_unref(hook_list, hook);
            hook = tmp;
        }
    }
    return ::core::ptr::null_mut::<GHook>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_find_data(
    mut hook_list: *mut GHookList,
    mut need_valids: gboolean,
    mut data: gpointer,
) -> *mut GHook {
    let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
    if ({
        let mut _g_boolean_var_52: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    hook = (*hook_list).hooks;
    while !hook.is_null() {
        if (*hook).data == data
            && (*hook).hook_id != 0
            && (need_valids == 0
                || (*hook).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint != 0 as guint)
        {
            return hook;
        }
        hook = (*hook).next;
    }
    return ::core::ptr::null_mut::<GHook>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_find_func(
    mut hook_list: *mut GHookList,
    mut need_valids: gboolean,
    mut func: gpointer,
) -> *mut GHook {
    let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
    if ({
        let mut _g_boolean_var_53: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    if ({
        let mut _g_boolean_var_54: ::core::ffi::c_int = 0;
        if !func.is_null() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    hook = (*hook_list).hooks;
    while !hook.is_null() {
        if (*hook).func == func
            && (*hook).hook_id != 0
            && (need_valids == 0
                || (*hook).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint != 0 as guint)
        {
            return hook;
        }
        hook = (*hook).next;
    }
    return ::core::ptr::null_mut::<GHook>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_find_func_data(
    mut hook_list: *mut GHookList,
    mut need_valids: gboolean,
    mut func: gpointer,
    mut data: gpointer,
) -> *mut GHook {
    let mut hook: *mut GHook = ::core::ptr::null_mut::<GHook>();
    if ({
        let mut _g_boolean_var_55: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    if ({
        let mut _g_boolean_var_56: ::core::ffi::c_int = 0;
        if !func.is_null() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GHook>();
    }
    hook = (*hook_list).hooks;
    while !hook.is_null() {
        if (*hook).data == data
            && (*hook).func == func
            && (*hook).hook_id != 0
            && (need_valids == 0
                || (*hook).flags & G_HOOK_FLAG_ACTIVE as ::core::ffi::c_int as guint != 0 as guint)
        {
            return hook;
        }
        hook = (*hook).next;
    }
    return ::core::ptr::null_mut::<GHook>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_insert_sorted(
    mut hook_list: *mut GHookList,
    mut hook: *mut GHook,
    mut func: GHookCompareFunc,
) {
    let mut sibling: *mut GHook = ::core::ptr::null_mut::<GHook>();
    if ({
        let mut _g_boolean_var_57: ::core::ffi::c_int = 0;
        if !hook_list.is_null() {
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
            b"hook_list != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_58: ::core::ffi::c_int = 0;
        if (*hook_list).is_setup() != 0 {
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
            b"hook_list->is_setup\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_59: ::core::ffi::c_int = 0;
        if !hook.is_null() {
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
            b"hook != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_60: ::core::ffi::c_int = 0;
        if (*hook).next.is_null()
            && (*hook).prev.is_null()
            && (*hook).hook_id == 0 as gulong
            && (*hook).ref_count == 0 as guint
        {
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
            b"G_HOOK_IS_UNLINKED (hook)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_61: ::core::ffi::c_int = 0;
        if !(*hook).func.is_null() {
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
            b"hook->func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_62: ::core::ffi::c_int = 0;
        if func.is_some() {
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
            b"func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    sibling = (*hook_list).hooks;
    while !sibling.is_null() && (*sibling).hook_id == 0 {
        sibling = (*sibling).next;
    }
    while !sibling.is_null() {
        let mut tmp: *mut GHook = ::core::ptr::null_mut::<GHook>();
        safe_c2rust_g_hook_ref(hook_list, sibling);
        if func.expect("non-null function pointer")(hook, sibling) <= 0 as ::core::ffi::c_int
            && (*sibling).hook_id != 0
        {
            safe_c2rust_g_hook_unref(hook_list, sibling);
            break;
        } else {
            tmp = (*sibling).next;
            while !tmp.is_null() && (*tmp).hook_id == 0 {
                tmp = (*tmp).next;
            }
            safe_c2rust_g_hook_unref(hook_list, sibling);
            sibling = tmp;
        }
    }
    safe_c2rust_g_hook_insert_before(hook_list, sibling, hook);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_hook_compare_ids(
    mut new_hook: *mut GHook,
    mut sibling: *mut GHook,
) -> gint {
    if (*new_hook).hook_id < (*sibling).hook_id {
        return -(1 as gint);
    } else if (*new_hook).hook_id > (*sibling).hook_id {
        return 1 as gint;
    }
    return 0 as gint;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_hook_list_init\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
