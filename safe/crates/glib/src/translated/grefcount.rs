extern "C" {
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
}
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type grefcount = gint;
pub type gatomicrefcount = gint;
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
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
pub const G_MININT: ::core::ffi::c_int = INT_MIN;
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ref_count_init(mut rc: *mut grefcount) {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !rc.is_null() {
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
            b"rc != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    *rc = -(1 as ::core::ffi::c_int) as grefcount;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ref_count_inc(mut rc: *mut grefcount) {
    let mut rrc: grefcount = 0;
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !rc.is_null() {
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
            b"rc != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    rrc = *rc;
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if rrc < 0 as ::core::ffi::c_int {
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
            b"rrc < 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if rrc == G_MININT {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Reference count %p has reached saturation\0" as *const u8 as *const gchar,
            rc,
        );
        return;
    }
    rrc -= 1 as ::core::ffi::c_int;
    *rc = rrc;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ref_count_dec(mut rc: *mut grefcount) -> gboolean {
    let mut rrc: grefcount = 0;
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !rc.is_null() {
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
            b"rc != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    rrc = *rc;
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if rrc < 0 as ::core::ffi::c_int {
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
            b"rrc < 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    rrc += 1 as ::core::ffi::c_int;
    if rrc == 0 as ::core::ffi::c_int {
        return TRUE;
    }
    *rc = rrc;
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_ref_count_compare(
    mut rc: *mut grefcount,
    mut val: gint,
) -> gboolean {
    let mut rrc: grefcount = 0;
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !rc.is_null() {
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
            b"rc != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if val >= 0 as ::core::ffi::c_int {
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
            b"val >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    rrc = *rc;
    if val == G_MAXINT {
        return (rrc == G_MININT) as ::core::ffi::c_int;
    }
    return (rrc == -val) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_ref_count_init(mut arc: *mut gatomicrefcount) {
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if !arc.is_null() {
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
            b"arc != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    *arc = 1 as ::core::ffi::c_int as gatomicrefcount;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_ref_count_inc(mut arc: *mut gatomicrefcount) {
    let mut old_value: gint = 0;
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !arc.is_null() {
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
            b"arc != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    old_value = ({
        if 0 as ::core::ffi::c_int != 0 {
            *arc;
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(arc, 1 as ::core::ffi::c_int)
    });
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if old_value > 0 as ::core::ffi::c_int {
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
            b"old_value > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if old_value == G_MAXINT {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"Reference count has reached saturation\0" as *const u8 as *const gchar,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_ref_count_dec(
    mut arc: *mut gatomicrefcount,
) -> gboolean {
    let mut old_value: gint = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !arc.is_null() {
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
            b"arc != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    old_value = ({
        if 0 as ::core::ffi::c_int != 0 {
            *arc;
            -(1 as ::core::ffi::c_int);
        } else {
        };
        crate::translated::compat::atomic_xadd_seqcst(arc, -(1 as ::core::ffi::c_int))
    });
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if old_value > 0 as ::core::ffi::c_int {
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
            b"old_value > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (old_value == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_atomic_ref_count_compare(
    mut arc: *mut gatomicrefcount,
    mut val: gint,
) -> gboolean {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !arc.is_null() {
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
            b"arc != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if val >= 0 as ::core::ffi::c_int {
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
            b"val >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    return (({
        let mut gaig_temp: gint = 0;
        if 0 as ::core::ffi::c_int != 0 {
            *arc;
            *arc;
        } else {
        };
        *&raw mut gaig_temp = crate::translated::compat::atomic_load_seqcst(arc as *mut gint);
        gaig_temp
    }) == val) as ::core::ffi::c_int;
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_ref_count_init\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
