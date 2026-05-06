extern "C" {
    fn dcgettext(
        __domainname: *const ::core::ffi::c_char,
        __msgid: *const ::core::ffi::c_char,
        __category: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn dcngettext(
        __domainname: *const ::core::ffi::c_char,
        __msgid1: *const ::core::ffi::c_char,
        __msgid2: *const ::core::ffi::c_char,
        __n: ::core::ffi::c_ulong,
        __category: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn textdomain(__domainname: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn bindtextdomain(
        __domainname: *const ::core::ffi::c_char,
        __dirname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn bind_textdomain_codeset(
        __domainname: *const ::core::ffi::c_char,
        __codeset: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_once_init_enter(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave(location: *mut ::core::ffi::c_void, result: gsize);
}
pub type size_t = usize;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gulong = ::core::ffi::c_ulong;
pub type gpointer = *mut ::core::ffi::c_void;
pub const SHOULD_TRANSLATE: C2RustUnnamed = 1;
pub const SHOULD_NOT_TRANSLATE: C2RustUnnamed = 2;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn safe_c2rust_ensure_gettext_initialized() {
    static mut safe_c2rust_initialised: gsize = 0;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialised;
        } else {
        };
        (({
            let mut gapg_temp_newval: gsize = 0;
            let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_initialised;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter(&raw mut safe_c2rust_initialised as *mut ::core::ffi::c_void) != 0)
            as ::core::ffi::c_int
    }) != 0
    {
        bindtextdomain(GETTEXT_PACKAGE.as_ptr(), GLIB_LOCALE_DIR.as_ptr());
        bind_textdomain_codeset(
            GETTEXT_PACKAGE.as_ptr(),
            b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_initialised = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_initialised as *mut ::core::ffi::c_void,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gsize,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_glib_gettext(mut str: *const gchar) -> *const gchar {
    safe_c2rust_ensure_gettext_initialized();
    return safe_c2rust_g_dgettext(GETTEXT_PACKAGE.as_ptr() as *const gchar, str);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_glib_pgettext(
    mut msgctxtid: *const gchar,
    mut msgidoffset: gsize,
) -> *const gchar {
    safe_c2rust_ensure_gettext_initialized();
    return safe_c2rust_g_dpgettext(
        GETTEXT_PACKAGE.as_ptr() as *const gchar,
        msgctxtid,
        msgidoffset,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_strip_context(
    mut msgid: *const gchar,
    mut msgval: *const gchar,
) -> *const gchar {
    if msgval == msgid {
        let mut c: *const ::core::ffi::c_char =
            strchr(msgid as *const ::core::ffi::c_char, '|' as i32);
        if !c.is_null() {
            return c.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    return msgval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dpgettext(
    mut domain: *const gchar,
    mut msgctxtid: *const gchar,
    mut msgidoffset: gsize,
) -> *const gchar {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut translation: *const gchar = ::core::ptr::null::<gchar>();
    let mut sep: *mut gchar = ::core::ptr::null_mut::<gchar>();
    translation = safe_c2rust_g_dgettext(domain, msgctxtid);
    if translation == msgctxtid {
        if msgidoffset > 0 as gsize {
            return msgctxtid.offset(msgidoffset as isize);
        }
        sep = strchr(msgctxtid as *const ::core::ffi::c_char, '|' as i32) as *mut gchar;
        if !sep.is_null() {
            alloca_allocations.push(::std::vec::from_elem(
                0,
                strlen(msgctxtid as *const ::core::ffi::c_char).wrapping_add(1 as size_t) as usize,
            ));
            let mut tmp: *mut gchar =
                alloca_allocations.last_mut().unwrap().as_mut_ptr().cast() as *mut gchar;
            strcpy(
                tmp as *mut ::core::ffi::c_char,
                msgctxtid as *const ::core::ffi::c_char,
            );
            *tmp.offset(sep.offset_from(msgctxtid) as ::core::ffi::c_long as isize) =
                '\u{4}' as i32 as gchar;
            translation = safe_c2rust_g_dgettext(domain, tmp);
            if translation == tmp as *const gchar {
                return sep.offset(1 as ::core::ffi::c_int as isize);
            }
        }
    }
    return translation;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dpgettext2(
    mut domain: *const gchar,
    mut msgctxt: *const gchar,
    mut msgid: *const gchar,
) -> *const gchar {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut msgctxt_len: size_t =
        strlen(msgctxt as *const ::core::ffi::c_char).wrapping_add(1 as size_t);
    let mut msgid_len: size_t =
        strlen(msgid as *const ::core::ffi::c_char).wrapping_add(1 as size_t);
    let mut translation: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut msg_ctxt_id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    alloca_allocations.push(::std::vec::from_elem(
        0,
        msgctxt_len.wrapping_add(msgid_len) as usize,
    ));
    msg_ctxt_id = alloca_allocations.last_mut().unwrap().as_mut_ptr().cast() as *mut ::core::ffi::c_char;
    memcpy(
        msg_ctxt_id as *mut ::core::ffi::c_void,
        msgctxt as *const ::core::ffi::c_void,
        msgctxt_len.wrapping_sub(1 as size_t),
    );
    *msg_ctxt_id.offset(msgctxt_len.wrapping_sub(1 as size_t) as isize) =
        '\u{4}' as i32 as ::core::ffi::c_char;
    memcpy(
        msg_ctxt_id.offset(msgctxt_len as isize) as *mut ::core::ffi::c_void,
        msgid as *const ::core::ffi::c_void,
        msgid_len,
    );
    translation = safe_c2rust_g_dgettext(domain, msg_ctxt_id) as *const ::core::ffi::c_char;
    if translation == msg_ctxt_id as *const ::core::ffi::c_char {
        *msg_ctxt_id.offset(msgctxt_len.wrapping_sub(1 as size_t) as isize) =
            '|' as i32 as ::core::ffi::c_char;
        translation = safe_c2rust_g_dgettext(domain, msg_ctxt_id) as *const ::core::ffi::c_char;
        if translation == msg_ctxt_id as *const ::core::ffi::c_char {
            return msgid;
        }
    }
    return translation as *const gchar;
}
unsafe extern "C" fn safe_c2rust__g_dgettext_should_translate() -> gboolean {
    static mut safe_c2rust_translate: gsize = 0 as gsize;
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if ({
            if 0 as ::core::ffi::c_int != 0 {
                safe_c2rust_translate;
            } else {
            };
            (({
                let mut gapg_temp_newval: gsize = 0;
                let mut gapg_temp_atomic: *mut gsize = &raw mut safe_c2rust_translate;
                *&raw mut gapg_temp_newval =
                    crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
                gapg_temp_newval
            }) == 0
                && g_once_init_enter(&raw mut safe_c2rust_translate as *mut ::core::ffi::c_void)
                    != 0) as ::core::ffi::c_int
        }) != 0
        {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
        let mut should_translate: gboolean = TRUE;
        let mut default_domain: *const ::core::ffi::c_char =
            textdomain(::core::ptr::null::<::core::ffi::c_char>());
        let mut translator_comment: *const ::core::ffi::c_char = dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\0" as *const u8 as *const ::core::ffi::c_char,
            LC_MESSAGES,
        );
        let mut translate_locale: *const ::core::ffi::c_char =
            setlocale(LC_MESSAGES, ::core::ptr::null::<::core::ffi::c_char>());
        if default_domain.is_null()
            || translator_comment.is_null()
            || translate_locale.is_null()
            || 0 as ::core::ffi::c_int
                != strcmp(
                    default_domain,
                    b"messages\0" as *const u8 as *const ::core::ffi::c_char,
                )
                && '\0' as i32 == *translator_comment as ::core::ffi::c_int
                && 0 as ::core::ffi::c_int
                    != strncmp(
                        translate_locale,
                        b"en_\0" as *const u8 as *const ::core::ffi::c_char,
                        3 as size_t,
                    )
                && 0 as ::core::ffi::c_int
                    != strcmp(
                        translate_locale,
                        b"C\0" as *const u8 as *const ::core::ffi::c_char,
                    )
        {
            should_translate = FALSE as gboolean;
        }
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_translate = (if should_translate != 0 {
                SHOULD_TRANSLATE as ::core::ffi::c_int
            } else {
                SHOULD_NOT_TRANSLATE as ::core::ffi::c_int
            }) as gsize;
        } else {
        };
        g_once_init_leave(
            &raw mut safe_c2rust_translate as *mut ::core::ffi::c_void,
            (if should_translate != 0 {
                SHOULD_TRANSLATE as ::core::ffi::c_int
            } else {
                SHOULD_NOT_TRANSLATE as ::core::ffi::c_int
            }) as gsize,
        );
    }
    return (safe_c2rust_translate == SHOULD_TRANSLATE as ::core::ffi::c_int as gsize)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dgettext(
    mut domain: *const gchar,
    mut msgid: *const gchar,
) -> *const gchar {
    if !domain.is_null()
        && ({
            let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
            if safe_c2rust__g_dgettext_should_translate() == 0 {
                _g_boolean_var_9 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_9 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_9
        }) as ::core::ffi::c_long
            != 0
    {
        return msgid;
    }
    return dcgettext(
        domain as *const ::core::ffi::c_char,
        msgid as *const ::core::ffi::c_char,
        LC_MESSAGES,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dcgettext(
    mut domain: *const gchar,
    mut msgid: *const gchar,
    mut category: gint,
) -> *const gchar {
    if !domain.is_null()
        && ({
            let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
            if safe_c2rust__g_dgettext_should_translate() == 0 {
                _g_boolean_var_10 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_10 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_10
        }) as ::core::ffi::c_long
            != 0
    {
        return msgid;
    }
    return dcgettext(
        domain as *const ::core::ffi::c_char,
        msgid as *const ::core::ffi::c_char,
        category as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_dngettext(
    mut domain: *const gchar,
    mut msgid: *const gchar,
    mut msgid_plural: *const gchar,
    mut n: gulong,
) -> *const gchar {
    if !domain.is_null()
        && ({
            let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
            if safe_c2rust__g_dgettext_should_translate() == 0 {
                _g_boolean_var_11 = 1 as ::core::ffi::c_int;
            } else {
                _g_boolean_var_11 = 0 as ::core::ffi::c_int;
            }
            _g_boolean_var_11
        }) as ::core::ffi::c_long
            != 0
    {
        return if n == 1 as gulong {
            msgid
        } else {
            msgid_plural
        };
    }
    return dcngettext(
        domain as *const ::core::ffi::c_char,
        msgid as *const ::core::ffi::c_char,
        msgid_plural as *const ::core::ffi::c_char,
        n as ::core::ffi::c_ulong,
        LC_MESSAGES,
    );
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const GETTEXT_PACKAGE: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"glib20\0") };
pub const GLIB_LOCALE_DIR: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"/usr/local/share/locale\0")
};
