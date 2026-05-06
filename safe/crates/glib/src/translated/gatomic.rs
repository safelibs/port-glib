pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gintptr = ::core::ffi::c_long;
pub type guintptr = ::core::ffi::c_ulong;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_int_get(mut atomic: *const gint) -> gint {
    return ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            *atomic;
            *atomic;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(atomic as *mut gint);
        gaig_temp
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_int_set(mut atomic: *mut gint, mut newval: gint) {
    let mut gais_temp: gint = newval;
    if 0 as ::core::ffi::c_int != 0 {
        *atomic;
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(atomic as *mut gint, *&raw mut gais_temp);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_int_inc(mut atomic: *mut gint) {
    if 0 as ::core::ffi::c_int != 0 {
        *atomic;
        *atomic;
    } else {
    };
    crate::translated::compat::atomic_xadd_seqcst(atomic, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_int_dec_and_test(mut atomic: *mut gint) -> gboolean {
    return ({
        if 0 as ::core::ffi::c_int != 0 {
            *atomic;
            *atomic;
        } else {
        };
        (crate::translated::compat::atomic_xsub_seqcst(atomic, 1 as ::core::ffi::c_int)
            == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_int_compare_and_exchange(
    mut atomic: *mut gint,
    mut oldval: gint,
    mut newval: gint,
) -> gboolean {
    return ({
        let mut gaicae_oldval: gint = oldval;
        if 0 as ::core::ffi::c_int != 0 {
            *atomic;
        } else {
        };
        let fresh0 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
            atomic,
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
            newval,
        );
        *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh0.0;
        if fresh0.1 as ::core::ffi::c_int != 0 {
            TRUE
        } else {
            FALSE
        }
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_int_compare_and_exchange_full(
    mut atomic: *mut gint,
    mut oldval: gint,
    mut newval: gint,
    mut preval: *mut gint,
) -> gboolean {
    return ({
        if 0 as ::core::ffi::c_int != 0 {
            *atomic;
            *preval;
        } else {
        };
        *preval = oldval;
        let fresh1 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(atomic, *preval, newval);
        *preval = fresh1.0;
        if fresh1.1 as ::core::ffi::c_int != 0 {
            TRUE
        } else {
            FALSE
        }
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_int_exchange(
    mut atomic: *mut gint,
    mut newval: gint,
) -> gint {
    return ({
        if 0 as ::core::ffi::c_int != 0 {
            *atomic;
        } else {
        };
        crate::translated::compat::atomic_xchg_seqcst(atomic, newval)
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_int_add(
    mut atomic: *mut gint,
    mut val: gint,
) -> gint {
    return ({
        if 0 as ::core::ffi::c_int != 0 {
            *atomic;
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(atomic, val)
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_int_and(
    mut atomic: *mut guint,
    mut val: guint,
) -> guint {
    return ({
        if 0 as ::core::ffi::c_int != 0 {
            *atomic;
        } else {
        };
        crate::translated::compat::atomic_and_seqcst(atomic, val)
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_int_or(
    mut atomic: *mut guint,
    mut val: guint,
) -> guint {
    return ({
        if 0 as ::core::ffi::c_int != 0 {
            *atomic;
        } else {
        };
        crate::translated::compat::atomic_or_seqcst(atomic, val)
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_int_xor(
    mut atomic: *mut guint,
    mut val: guint,
) -> guint {
    return ({
        if 0 as ::core::ffi::c_int != 0 {
            *atomic;
        } else {
        };
        crate::translated::compat::atomic_xor_seqcst(atomic, val)
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_pointer_get(
    mut atomic: *const ::core::ffi::c_void,
) -> gpointer {
    return ({
        let mut gapg_temp_newval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut gapg_temp_atomic: *mut gpointer = atomic as *mut gpointer;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_pointer_set(
    mut atomic: *mut ::core::ffi::c_void,
    mut newval: gpointer,
) {
    let mut gaps_temp_atomic: *mut gpointer = atomic as *mut gpointer;
    let mut gaps_temp_newval: gpointer = newval as gpointer;
    if 0 as ::core::ffi::c_int != 0 {
        *(atomic as *mut gpointer);
    } else {
    };
    crate::translated::compat::atomic_store_seqcst(gaps_temp_atomic, *&raw mut gaps_temp_newval);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_pointer_compare_and_exchange(
    mut atomic: *mut ::core::ffi::c_void,
    mut oldval: gpointer,
    mut newval: gpointer,
) -> gboolean {
    return ({
        let mut gapcae_oldval: gpointer = oldval;
        if 0 as ::core::ffi::c_int != 0 {
            *(atomic as *mut gpointer);
        } else {
        };
        let fresh2 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
            atomic as *mut gpointer,
            *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut gpointer),
            newval,
        );
        *(&raw mut gapcae_oldval as *mut ::core::ffi::c_void as *mut gpointer) = fresh2.0;
        if fresh2.1 as ::core::ffi::c_int != 0 {
            TRUE
        } else {
            FALSE
        }
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_pointer_compare_and_exchange_full(
    mut atomic: *mut ::core::ffi::c_void,
    mut oldval: gpointer,
    mut newval: gpointer,
    mut preval: *mut ::core::ffi::c_void,
) -> gboolean {
    return ({
        if 0 as ::core::ffi::c_int != 0 {
            *(atomic as *mut gpointer);
        } else {
        };
        if 0 as ::core::ffi::c_int != 0 {
            *(preval as *mut gpointer);
        } else {
        };
        let ref mut fresh3 = *(preval as *mut gpointer);
        *fresh3 = oldval;
        let fresh4 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
            atomic as *mut gpointer,
            *(preval as *mut gpointer),
            newval,
        );
        *(preval as *mut gpointer) = fresh4.0;
        if fresh4.1 as ::core::ffi::c_int != 0 {
            TRUE
        } else {
            FALSE
        }
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_pointer_exchange(
    mut atomic: *mut ::core::ffi::c_void,
    mut newval: gpointer,
) -> gpointer {
    return ({
        if 0 as ::core::ffi::c_int != 0 {
            *(atomic as *mut gpointer);
        } else {
        };
        crate::translated::compat::atomic_xchg_seqcst(atomic as *mut gpointer, newval)
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_pointer_add(
    mut atomic: *mut ::core::ffi::c_void,
    mut val: gssize,
) -> gintptr {
    return ({
        if 0 as ::core::ffi::c_int != 0 {
            *(atomic as *mut gpointer);
        } else {
        };
        if 0 as ::core::ffi::c_int != 0 {
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(atomic as *mut gintptr, val) as gintptr
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_pointer_and(
    mut atomic: *mut ::core::ffi::c_void,
    mut val: gsize,
) -> guintptr {
    return ({
        let mut gapa_atomic: *mut guintptr = atomic as *mut gpointer as *mut guintptr;
        if 0 as ::core::ffi::c_int != 0 {
            *(atomic as *mut gpointer);
        } else {
        };
        if 0 as ::core::ffi::c_int != 0 {
        } else {
        };
        crate::translated::compat::atomic_and_seqcst(gapa_atomic, val)
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_pointer_or(
    mut atomic: *mut ::core::ffi::c_void,
    mut val: gsize,
) -> guintptr {
    return ({
        let mut gapo_atomic: *mut guintptr = atomic as *mut gpointer as *mut guintptr;
        if 0 as ::core::ffi::c_int != 0 {
            *(atomic as *mut gpointer);
        } else {
        };
        if 0 as ::core::ffi::c_int != 0 {
        } else {
        };
        crate::translated::compat::atomic_or_seqcst(gapo_atomic, val)
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_pointer_xor(
    mut atomic: *mut ::core::ffi::c_void,
    mut val: gsize,
) -> guintptr {
    return ({
        let mut gapx_atomic: *mut guintptr = atomic as *mut gpointer as *mut guintptr;
        if 0 as ::core::ffi::c_int != 0 {
            *(atomic as *mut gpointer);
        } else {
        };
        if 0 as ::core::ffi::c_int != 0 {
        } else {
        };
        crate::translated::compat::atomic_xor_seqcst(gapx_atomic, val)
    });
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_int_exchange_and_add(
    mut atomic: *mut gint,
    mut val: gint,
) -> gint {
    return safe_c2rust_g_atomic_int_add(atomic as *mut gint as *mut gint, val);
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
