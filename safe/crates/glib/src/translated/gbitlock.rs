extern "C" {
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn syscall(__sysno: ::core::ffi::c_long, ...) -> ::core::ffi::c_long;
}
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub const UINT_MAX: ::core::ffi::c_uint = (__INT_MAX__ as ::core::ffi::c_uint)
    .wrapping_mul(2 as ::core::ffi::c_uint)
    .wrapping_add(1 as ::core::ffi::c_uint);
pub const G_MAXUINT: ::core::ffi::c_uint = UINT_MAX;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const __NR_futex: ::core::ffi::c_int = 202 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust_g_futex_wait(mut address: *const gint, mut value: gint) {
    let mut saved_errno: ::core::ffi::c_int = *__errno_location();
    let mut res: ::core::ffi::c_int = syscall(
        __NR_futex as ::core::ffi::c_long,
        address,
        (0 as ::core::ffi::c_int | 128 as ::core::ffi::c_int) as gsize,
        value as gsize,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int && *__errno_location() == EAGAIN {
        *__errno_location() = saved_errno;
    }
}
unsafe extern "C" fn safe_c2rust_g_futex_wake(mut address: *const gint) {
    let mut saved_errno: ::core::ffi::c_int = *__errno_location();
    let mut res: ::core::ffi::c_int = syscall(
        __NR_futex as ::core::ffi::c_long,
        address,
        (1 as ::core::ffi::c_int | 128 as ::core::ffi::c_int) as gsize,
        1 as ::core::ffi::c_int as gsize,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int && *__errno_location() == EAGAIN {
        *__errno_location() = saved_errno;
    }
}
static mut safe_c2rust_g_bit_lock_contended: [gint; 11] = [0; 11];
#[inline(always)]
unsafe extern "C" fn safe_c2rust_bit_lock_contended_class(mut address: gpointer) -> guint {
    return (address as usize).wrapping_rem(
        (::core::mem::size_of::<[gint; 11]>() as usize)
            .wrapping_div(::core::mem::size_of::<gint>() as usize),
    ) as guint;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bit_lock(mut address: *mut gint, mut lock_bit: gint) {
    let mut address_nonvolatile: *mut gint = address as *mut gint;
    let mut mask: guint = (1 as guint) << lock_bit;
    let mut v: guint = 0;
    loop {
        v = ({
            if 0 as ::core::ffi::c_int != 0 {
                *address_nonvolatile;
            } else {
            };
            crate::translated::compat::atomic_or_seqcst(address_nonvolatile, mask as gint) as guint
        });
        if !(v & mask != 0) {
            break;
        }
        let mut class: guint =
            safe_c2rust_bit_lock_contended_class(address_nonvolatile as gpointer);
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_g_bit_lock_contended[class as usize];
            1 as ::core::ffi::c_int;
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(
            (&raw mut safe_c2rust_g_bit_lock_contended as *mut gint).offset(class as isize)
                as *mut gint,
            1 as ::core::ffi::c_int,
        );
        safe_c2rust_g_futex_wait(address_nonvolatile, v as gint);
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_g_bit_lock_contended[class as usize];
            -(1 as ::core::ffi::c_int);
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(
            (&raw mut safe_c2rust_g_bit_lock_contended as *mut gint).offset(class as isize)
                as *mut gint,
            -(1 as ::core::ffi::c_int),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bit_trylock(
    mut address: *mut gint,
    mut lock_bit: gint,
) -> gboolean {
    let mut address_nonvolatile: *mut gint = address as *mut gint;
    let mut mask: guint = (1 as guint) << lock_bit;
    let mut v: guint = 0;
    v = ({
        if 0 as ::core::ffi::c_int != 0 {
            *address_nonvolatile;
        } else {
        };
        crate::translated::compat::atomic_or_seqcst(address_nonvolatile, mask as gint) as guint
    });
    return (!v & mask) as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_bit_unlock(mut address: *mut gint, mut lock_bit: gint) {
    let mut address_nonvolatile: *mut gint = address as *mut gint;
    let mut mask: guint = (1 as guint) << lock_bit;
    if 0 as ::core::ffi::c_int != 0 {
        *address_nonvolatile;
        !mask;
    } else {
    };
    crate::translated::compat::atomic_and_seqcst(address_nonvolatile, !mask as gint);
    let mut class: guint = safe_c2rust_bit_lock_contended_class(address_nonvolatile as gpointer);
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_g_bit_lock_contended[class as usize];
            safe_c2rust_g_bit_lock_contended[class as usize];
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
            (&raw mut safe_c2rust_g_bit_lock_contended as *mut gint).offset(class as isize)
                as *mut gint,
        );
        gaig_temp
    }) != 0
    {
        safe_c2rust_g_futex_wake(address_nonvolatile);
    }
}
unsafe extern "C" fn safe_c2rust_g_futex_int_address(
    mut address: *const ::core::ffi::c_void,
) -> *const gint {
    let mut int_address: *const gint = address as *const gint;
    return int_address;
}
#[inline(always)]
unsafe extern "C" fn safe_c2rust_pointer_bit_lock_mask_ptr(
    mut ptr: gpointer,
    mut lock_bit: guint,
    mut set: gboolean,
    mut preserve_mask: guintptr,
    mut preserve_ptr: gpointer,
) -> gpointer {
    let mut x_ptr: guintptr = 0;
    let mut x_preserve_ptr: guintptr = 0;
    let mut lock_mask: guintptr = 0;
    x_ptr = ptr as guintptr;
    if preserve_mask != 0 as guintptr {
        x_preserve_ptr = preserve_ptr as guintptr;
        x_ptr = x_preserve_ptr & preserve_mask | x_ptr & !preserve_mask;
    }
    if lock_bit == G_MAXUINT {
        return x_ptr as gpointer;
    }
    lock_mask = ((1 as ::core::ffi::c_uint) << lock_bit) as guintptr;
    if set != 0 {
        return (x_ptr | lock_mask) as gpointer;
    } else {
        return (x_ptr & !lock_mask) as gpointer;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pointer_bit_lock_and_get(
    mut address: gpointer,
    mut lock_bit: guint,
    mut out_ptr: *mut guintptr,
) {
    let mut class: guint = safe_c2rust_bit_lock_contended_class(address);
    let mut mask: guintptr = 0;
    let mut v: guintptr = 0;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if lock_bit < 32 as guint {
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
            b"lock_bit < 32\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    mask = ((1 as ::core::ffi::c_uint) << lock_bit) as guintptr;
    loop {
        v = ({
            let mut gapo_atomic: *mut guintptr = address as *mut gpointer as *mut guintptr;
            if 0 as ::core::ffi::c_int != 0 {
                *(address as *mut gpointer);
            } else {
            };
            if 0 as ::core::ffi::c_int != 0 {
            } else {
            };
            crate::translated::compat::atomic_or_seqcst(gapo_atomic, mask)
        });
        if !(v & mask != 0) {
            break;
        }
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_g_bit_lock_contended[class as usize];
            1 as ::core::ffi::c_int;
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(
            (&raw mut safe_c2rust_g_bit_lock_contended as *mut gint).offset(class as isize)
                as *mut gint,
            1 as ::core::ffi::c_int,
        );
        safe_c2rust_g_futex_wait(
            safe_c2rust_g_futex_int_address(address as *const ::core::ffi::c_void),
            v as guint as gint,
        );
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_g_bit_lock_contended[class as usize];
            -(1 as ::core::ffi::c_int);
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(
            (&raw mut safe_c2rust_g_bit_lock_contended as *mut gint).offset(class as isize)
                as *mut gint,
            -(1 as ::core::ffi::c_int),
        );
    }
    if !out_ptr.is_null() {
        *out_ptr = v | mask;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pointer_bit_lock(
    mut address: *mut ::core::ffi::c_void,
    mut lock_bit: gint,
) {
    safe_c2rust_g_pointer_bit_lock_and_get(
        address as *mut gpointer as gpointer,
        lock_bit as guint,
        ::core::ptr::null_mut::<guintptr>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pointer_bit_trylock(
    mut address: *mut ::core::ffi::c_void,
    mut lock_bit: gint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if lock_bit < 32 as ::core::ffi::c_int {
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
            b"lock_bit < 32\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    let mut address_nonvolatile: *mut ::core::ffi::c_void = address as *mut ::core::ffi::c_void;
    let mut pointer_address: *mut gpointer = address_nonvolatile as *mut gpointer;
    let mut mask: gsize = ((1 as ::core::ffi::c_uint) << lock_bit) as gsize;
    let mut v: guintptr = 0;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if lock_bit < 32 as ::core::ffi::c_int {
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
            b"lock_bit < 32\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    v = ({
        let mut gapo_atomic: *mut guintptr = pointer_address as *mut guintptr;
        if 0 as ::core::ffi::c_int != 0 {
            *pointer_address;
        } else {
        };
        if 0 as ::core::ffi::c_int != 0 {
        } else {
        };
        crate::translated::compat::atomic_or_seqcst(gapo_atomic, mask)
    });
    return (!v & mask != 0 as gsize) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pointer_bit_unlock(
    mut address: *mut ::core::ffi::c_void,
    mut lock_bit: gint,
) {
    let mut address_nonvolatile: *mut ::core::ffi::c_void = address as *mut ::core::ffi::c_void;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if lock_bit < 32 as ::core::ffi::c_int {
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
            b"lock_bit < 32\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut pointer_address: *mut gpointer = address_nonvolatile as *mut gpointer;
    let mut mask: gsize = ((1 as ::core::ffi::c_uint) << lock_bit) as gsize;
    let mut gapa_atomic: *mut guintptr = pointer_address as *mut guintptr;
    if 0 as ::core::ffi::c_int != 0 {
        *pointer_address;
    } else {
    };
    if 0 as ::core::ffi::c_int != 0 {
        !mask;
        !mask;
    } else {
    };
    crate::translated::compat::atomic_and_seqcst(gapa_atomic, !mask);
    let mut class: guint = safe_c2rust_bit_lock_contended_class(address_nonvolatile as gpointer);
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_g_bit_lock_contended[class as usize];
            safe_c2rust_g_bit_lock_contended[class as usize];
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
            (&raw mut safe_c2rust_g_bit_lock_contended as *mut gint).offset(class as isize)
                as *mut gint,
        );
        gaig_temp
    }) != 0
    {
        safe_c2rust_g_futex_wake(safe_c2rust_g_futex_int_address(address_nonvolatile));
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pointer_bit_lock_mask_ptr(
    mut ptr: gpointer,
    mut lock_bit: guint,
    mut set: gboolean,
    mut preserve_mask: guintptr,
    mut preserve_ptr: gpointer,
) -> gpointer {
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if lock_bit < 32 as ::core::ffi::c_uint
            || lock_bit
                == (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint)
        {
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
            b"lock_bit < 32u || lock_bit == G_MAXUINT\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ptr;
    }
    return safe_c2rust_pointer_bit_lock_mask_ptr(ptr, lock_bit, set, preserve_mask, preserve_ptr);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_pointer_bit_unlock_and_set(
    mut address: *mut ::core::ffi::c_void,
    mut lock_bit: guint,
    mut ptr: gpointer,
    mut preserve_mask: guintptr,
) {
    let mut pointer_address: *mut gpointer = address as *mut gpointer;
    let mut class: guint = safe_c2rust_bit_lock_contended_class(address as gpointer);
    let mut ptr2: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if lock_bit < 32 as ::core::ffi::c_uint {
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
            b"lock_bit < 32u\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if preserve_mask != 0 as guintptr {
        let mut old_ptr: gpointer = ({
            let mut gapg_temp_newval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
            let mut gapg_temp_atomic: *mut gpointer = address as *mut gpointer;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        });
        loop {
            ptr2 =
                safe_c2rust_pointer_bit_lock_mask_ptr(ptr, lock_bit, FALSE, preserve_mask, old_ptr);
            if !(({
                if 0 as ::core::ffi::c_int != 0 {
                    *pointer_address;
                } else {
                };
                if 0 as ::core::ffi::c_int != 0 {
                    old_ptr;
                } else {
                };
                old_ptr = old_ptr;
                let fresh0 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    pointer_address,
                    *&raw mut old_ptr,
                    ptr2,
                );
                *&raw mut old_ptr = fresh0.0;
                if fresh0.1 as ::core::ffi::c_int != 0 {
                    TRUE
                } else {
                    FALSE
                }
            }) == 0)
            {
                break;
            }
        }
    } else {
        ptr2 = safe_c2rust_pointer_bit_lock_mask_ptr(ptr, lock_bit, FALSE, 0 as guintptr, NULL);
        let mut gaps_temp_atomic: *mut gpointer = pointer_address as *mut gpointer;
        let mut gaps_temp_newval: gpointer = ptr2 as gpointer;
        if 0 as ::core::ffi::c_int != 0 {
            *pointer_address;
        } else {
        };
        crate::translated::compat::atomic_store_seqcst(gaps_temp_atomic, *&raw mut gaps_temp_newval);
    }
    if ({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_g_bit_lock_contended[class as usize];
            safe_c2rust_g_bit_lock_contended[class as usize];
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(
            (&raw mut safe_c2rust_g_bit_lock_contended as *mut gint).offset(class as isize)
                as *mut gint,
        );
        gaig_temp
    }) > 0 as ::core::ffi::c_int
    {
        safe_c2rust_g_futex_wake(safe_c2rust_g_futex_int_address(address));
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if ptr
            == safe_c2rust_pointer_bit_lock_mask_ptr(
                ptr,
                lock_bit,
                0 as gboolean,
                0 as guintptr,
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
            )
        {
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
            b"ptr == pointer_bit_lock_mask_ptr (ptr, lock_bit, FALSE, 0, NULL)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_pointer_bit_lock_and_get\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
