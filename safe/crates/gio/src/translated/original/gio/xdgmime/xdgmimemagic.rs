extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn __gio_xdg_mime_type_equal(
        mime_a: *const ::core::ffi::c_char,
        mime_b: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn ungetc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn __uflow(_: *mut FILE) -> ::core::ffi::c_int;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XdgMimeMagic {
    pub match_list: *mut XdgMimeMagicMatch,
    pub max_extent: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XdgMimeMagicMatch {
    pub mime_type: *const ::core::ffi::c_char,
    pub priority: ::core::ffi::c_int,
    pub matchlet: *mut XdgMimeMagicMatchlet,
    pub next: *mut XdgMimeMagicMatch,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XdgMimeMagicMatchlet {
    pub indent: ::core::ffi::c_int,
    pub offset: ::core::ffi::c_int,
    pub value_length: ::core::ffi::c_uint,
    pub value: *mut ::core::ffi::c_uchar,
    pub mask: *mut ::core::ffi::c_uchar,
    pub range_length: ::core::ffi::c_uint,
    pub word_size: ::core::ffi::c_uint,
    pub next: *mut XdgMimeMagicMatchlet,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub const XDG_MIME_MAGIC_EOF: XdgMimeMagicState = 3;
pub const XDG_MIME_MAGIC_SECTION: XdgMimeMagicState = 0;
pub type XdgMimeMagicState = ::core::ffi::c_uint;
pub const XDG_MIME_MAGIC_ERROR: XdgMimeMagicState = 2;
pub const XDG_MIME_MAGIC_MAGIC: XdgMimeMagicState = 1;
pub type xdg_uint32_t = ::core::ffi::c_uint;
pub type xdg_uint16_t = ::core::ffi::c_ushort;
pub const _ISdigit: C2RustUnnamed = 2048;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[inline]
unsafe extern "C" fn safe_c2rust_getc_unlocked(mut __fp: *mut FILE) -> ::core::ffi::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
        *(fresh0 as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int
    };
}
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_match_new() -> *mut XdgMimeMagicMatch {
    return calloc(
        1 as size_t,
        ::core::mem::size_of::<XdgMimeMagicMatch>() as size_t,
    ) as *mut XdgMimeMagicMatch;
}
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_matchlet_new() -> *mut XdgMimeMagicMatchlet {
    let mut matchlet: *mut XdgMimeMagicMatchlet = ::core::ptr::null_mut::<XdgMimeMagicMatchlet>();
    matchlet = malloc(::core::mem::size_of::<XdgMimeMagicMatchlet>() as size_t)
        as *mut XdgMimeMagicMatchlet;
    if matchlet.is_null() {
        return ::core::ptr::null_mut::<XdgMimeMagicMatchlet>();
    }
    (*matchlet).indent = 0 as ::core::ffi::c_int;
    (*matchlet).offset = 0 as ::core::ffi::c_int;
    (*matchlet).value_length = 0 as ::core::ffi::c_uint;
    (*matchlet).value = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*matchlet).mask = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*matchlet).range_length = 1 as ::core::ffi::c_uint;
    (*matchlet).word_size = 1 as ::core::ffi::c_uint;
    (*matchlet).next = ::core::ptr::null_mut::<XdgMimeMagicMatchlet>();
    return matchlet;
}
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_matchlet_free(
    mut mime_magic_matchlet: *mut XdgMimeMagicMatchlet,
) {
    if !mime_magic_matchlet.is_null() {
        if !(*mime_magic_matchlet).next.is_null() {
            safe_c2rust__xdg_mime_magic_matchlet_free((*mime_magic_matchlet).next);
        }
        if !(*mime_magic_matchlet).value.is_null() {
            free((*mime_magic_matchlet).value as *mut ::core::ffi::c_void);
        }
        if !(*mime_magic_matchlet).mask.is_null() {
            free((*mime_magic_matchlet).mask as *mut ::core::ffi::c_void);
        }
        free(mime_magic_matchlet as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_match_free(
    mut mime_magic_match: *mut XdgMimeMagicMatch,
) {
    let mut ptr: *mut XdgMimeMagicMatch = ::core::ptr::null_mut::<XdgMimeMagicMatch>();
    let mut next: *mut XdgMimeMagicMatch = ::core::ptr::null_mut::<XdgMimeMagicMatch>();
    ptr = mime_magic_match;
    while !ptr.is_null() {
        next = (*ptr).next;
        if !(*ptr).mime_type.is_null() {
            free((*ptr).mime_type as *mut ::core::ffi::c_void);
        }
        if !(*ptr).matchlet.is_null() {
            safe_c2rust__xdg_mime_magic_matchlet_free((*ptr).matchlet);
        }
        free(ptr as *mut ::core::ffi::c_void);
        ptr = next;
    }
}
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_read_to_newline(
    mut magic_file: *mut FILE,
    mut end_of_file: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    let mut retval: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut c: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut pos: ::core::ffi::c_int = 0;
    len = 128 as ::core::ffi::c_int;
    pos = 0 as ::core::ffi::c_int;
    retval = malloc(len as size_t) as *mut ::core::ffi::c_uchar;
    *end_of_file = FALSE;
    while FALSE == 0 {
        c = safe_c2rust_getc_unlocked(magic_file);
        if c == EOF {
            *end_of_file = TRUE;
            break;
        } else {
            if c == '\n' as i32 || c == '\0' as i32 {
                break;
            }
            let fresh1 = pos;
            pos = pos + 1;
            *retval.offset(fresh1 as isize) = c as ::core::ffi::c_uchar;
            if pos % 128 as ::core::ffi::c_int == 127 as ::core::ffi::c_int {
                len = len + 128 as ::core::ffi::c_int;
                retval = realloc(retval as *mut ::core::ffi::c_void, len as size_t)
                    as *mut ::core::ffi::c_uchar;
            }
        }
    }
    *retval.offset(pos as isize) = '\0' as i32 as ::core::ffi::c_uchar;
    return retval;
}
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_read_a_number(
    mut magic_file: *mut FILE,
    mut end_of_file: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut number_string: [::core::ffi::c_char; 31] = [0; 31];
    let mut pos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 0;
    let mut retval: ::core::ffi::c_long = -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    while FALSE == 0 {
        c = safe_c2rust_getc_unlocked(magic_file);
        if c == EOF {
            *end_of_file = TRUE;
            break;
        } else if *(*__ctype_b_loc()).offset(c as isize) as ::core::ffi::c_int
            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            == 0
        {
            ungetc(c, magic_file);
            break;
        } else {
            number_string[pos as usize] = c as ::core::ffi::c_char;
            pos += 1;
            if pos == MAX_NUMBER_SIZE {
                break;
            }
        }
    }
    if pos > 0 as ::core::ffi::c_int {
        number_string[pos as usize] = '\0' as i32 as ::core::ffi::c_char;
        *__errno_location() = 0 as ::core::ffi::c_int;
        retval = strtol(
            &raw mut number_string as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        );
        if retval < INT_MIN as ::core::ffi::c_long
            || retval > INT_MAX as ::core::ffi::c_long
            || *__errno_location() != 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
    }
    return retval as ::core::ffi::c_int;
}
pub const MAX_NUMBER_SIZE: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_parse_header(
    mut magic_file: *mut FILE,
    mut match_0: *mut XdgMimeMagicMatch,
) -> XdgMimeMagicState {
    let mut c: ::core::ffi::c_int = 0;
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut end_ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut end_of_file: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    '_c2rust_label: {
        if !magic_file.is_null() {
        } else {
            __assert_fail(
                b"magic_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"../original/gio/xdgmime/xdgmimemagic.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                238 as ::core::ffi::c_uint,
                b"XdgMimeMagicState _xdg_mime_magic_parse_header(FILE *, XdgMimeMagicMatch *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !match_0.is_null() {
        } else {
            __assert_fail(
                b"match != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"../original/gio/xdgmime/xdgmimemagic.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                239 as ::core::ffi::c_uint,
                b"XdgMimeMagicState _xdg_mime_magic_parse_header(FILE *, XdgMimeMagicMatch *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    c = safe_c2rust_getc_unlocked(magic_file);
    if c == EOF {
        return XDG_MIME_MAGIC_EOF;
    }
    if c != '[' as i32 {
        return XDG_MIME_MAGIC_ERROR;
    }
    (*match_0).priority =
        safe_c2rust__xdg_mime_magic_read_a_number(magic_file, &raw mut end_of_file);
    if end_of_file != 0 {
        return XDG_MIME_MAGIC_EOF;
    }
    if (*match_0).priority == -(1 as ::core::ffi::c_int) {
        return XDG_MIME_MAGIC_ERROR;
    }
    c = safe_c2rust_getc_unlocked(magic_file);
    if c == EOF {
        return XDG_MIME_MAGIC_EOF;
    }
    if c != ':' as i32 {
        return XDG_MIME_MAGIC_ERROR;
    }
    buffer = safe_c2rust__xdg_mime_magic_read_to_newline(magic_file, &raw mut end_of_file)
        as *mut ::core::ffi::c_char;
    if end_of_file != 0 {
        free(buffer as *mut ::core::ffi::c_void);
        return XDG_MIME_MAGIC_EOF;
    }
    end_ptr = buffer;
    while *end_ptr as ::core::ffi::c_int != ']' as i32
        && *end_ptr as ::core::ffi::c_int != '\0' as i32
        && *end_ptr as ::core::ffi::c_int != '\n' as i32
    {
        end_ptr = end_ptr.offset(1);
    }
    if *end_ptr as ::core::ffi::c_int != ']' as i32 {
        free(buffer as *mut ::core::ffi::c_void);
        return XDG_MIME_MAGIC_ERROR;
    }
    *end_ptr = '\0' as i32 as ::core::ffi::c_char;
    (*match_0).mime_type = strdup(buffer);
    free(buffer as *mut ::core::ffi::c_void);
    return XDG_MIME_MAGIC_MAGIC;
}
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_parse_error(
    mut magic_file: *mut FILE,
) -> XdgMimeMagicState {
    let mut c: ::core::ffi::c_int = 0;
    loop {
        c = safe_c2rust_getc_unlocked(magic_file);
        if c == EOF {
            return XDG_MIME_MAGIC_EOF;
        }
        if c == '\n' as i32 {
            return XDG_MIME_MAGIC_SECTION;
        }
    }
}
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_parse_magic_line(
    mut magic_file: *mut FILE,
    mut match_0: *mut XdgMimeMagicMatch,
) -> XdgMimeMagicState {
    let mut matchlet: *mut XdgMimeMagicMatchlet = ::core::ptr::null_mut::<XdgMimeMagicMatchlet>();
    let mut c: ::core::ffi::c_int = 0;
    let mut end_of_file: ::core::ffi::c_int = 0;
    let mut indent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bytes_read: size_t = 0;
    '_c2rust_label: {
        if !magic_file.is_null() {
        } else {
            __assert_fail(
                b"magic_file != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"../original/gio/xdgmime/xdgmimemagic.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                311 as ::core::ffi::c_uint,
                b"XdgMimeMagicState _xdg_mime_magic_parse_magic_line(FILE *, XdgMimeMagicMatch *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    c = safe_c2rust_getc_unlocked(magic_file);
    if c == EOF {
        return XDG_MIME_MAGIC_EOF;
    } else if c == '[' as i32 {
        ungetc(c, magic_file);
        return XDG_MIME_MAGIC_SECTION;
    } else if c == '\n' as i32 {
        return XDG_MIME_MAGIC_MAGIC;
    }
    end_of_file = FALSE;
    if *(*__ctype_b_loc()).offset(c as isize) as ::core::ffi::c_int
        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        ungetc(c, magic_file);
        indent = safe_c2rust__xdg_mime_magic_read_a_number(magic_file, &raw mut end_of_file);
        if end_of_file != 0 {
            return XDG_MIME_MAGIC_EOF;
        }
        if indent == -(1 as ::core::ffi::c_int) {
            return XDG_MIME_MAGIC_ERROR;
        }
        c = safe_c2rust_getc_unlocked(magic_file);
        if c == EOF {
            return XDG_MIME_MAGIC_EOF;
        }
    }
    if c != '>' as i32 {
        return XDG_MIME_MAGIC_ERROR;
    }
    matchlet = safe_c2rust__xdg_mime_magic_matchlet_new();
    if matchlet.is_null() {
        return XDG_MIME_MAGIC_ERROR;
    }
    (*matchlet).indent = indent;
    (*matchlet).offset =
        safe_c2rust__xdg_mime_magic_read_a_number(magic_file, &raw mut end_of_file);
    if end_of_file != 0 {
        safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
        return XDG_MIME_MAGIC_EOF;
    }
    if (*matchlet).offset == -(1 as ::core::ffi::c_int) {
        safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
        return XDG_MIME_MAGIC_ERROR;
    }
    c = safe_c2rust_getc_unlocked(magic_file);
    if c == EOF {
        safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
        return XDG_MIME_MAGIC_EOF;
    } else if c != '=' as i32 {
        safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
        return XDG_MIME_MAGIC_ERROR;
    }
    (*matchlet).value_length = 0 as ::core::ffi::c_uint;
    c = safe_c2rust_getc_unlocked(magic_file);
    if c == EOF {
        safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
        return XDG_MIME_MAGIC_EOF;
    }
    (*matchlet).value_length = (c & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uint;
    (*matchlet).value_length = (*matchlet).value_length << 8 as ::core::ffi::c_int;
    c = safe_c2rust_getc_unlocked(magic_file);
    if c == EOF {
        safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
        return XDG_MIME_MAGIC_EOF;
    }
    (*matchlet).value_length = (*matchlet)
        .value_length
        .wrapping_add((c & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uint);
    (*matchlet).value = malloc((*matchlet).value_length as size_t) as *mut ::core::ffi::c_uchar;
    if (*matchlet).value.is_null() {
        safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
        return XDG_MIME_MAGIC_ERROR;
    }
    bytes_read = fread(
        (*matchlet).value as *mut ::core::ffi::c_void,
        1 as size_t,
        (*matchlet).value_length as size_t,
        magic_file,
    ) as size_t;
    if bytes_read != (*matchlet).value_length as size_t {
        safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
        if feof(magic_file) != 0 {
            return XDG_MIME_MAGIC_EOF;
        } else {
            return XDG_MIME_MAGIC_ERROR;
        }
    }
    c = safe_c2rust_getc_unlocked(magic_file);
    if c == '&' as i32 {
        (*matchlet).mask = malloc((*matchlet).value_length as size_t) as *mut ::core::ffi::c_uchar;
        if (*matchlet).mask.is_null() {
            safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
            return XDG_MIME_MAGIC_ERROR;
        }
        bytes_read = fread(
            (*matchlet).mask as *mut ::core::ffi::c_void,
            1 as size_t,
            (*matchlet).value_length as size_t,
            magic_file,
        ) as size_t;
        if bytes_read != (*matchlet).value_length as size_t {
            safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
            if feof(magic_file) != 0 {
                return XDG_MIME_MAGIC_EOF;
            } else {
                return XDG_MIME_MAGIC_ERROR;
            }
        }
        c = safe_c2rust_getc_unlocked(magic_file);
    }
    if c == '~' as i32 {
        (*matchlet).word_size =
            safe_c2rust__xdg_mime_magic_read_a_number(magic_file, &raw mut end_of_file)
                as ::core::ffi::c_uint;
        if end_of_file != 0 {
            safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
            return XDG_MIME_MAGIC_EOF;
        }
        if (*matchlet).word_size != 0 as ::core::ffi::c_uint
            && (*matchlet).word_size != 1 as ::core::ffi::c_uint
            && (*matchlet).word_size != 2 as ::core::ffi::c_uint
            && (*matchlet).word_size != 4 as ::core::ffi::c_uint
        {
            safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
            return XDG_MIME_MAGIC_ERROR;
        }
        c = safe_c2rust_getc_unlocked(magic_file);
    }
    if c == '+' as i32 {
        (*matchlet).range_length =
            safe_c2rust__xdg_mime_magic_read_a_number(magic_file, &raw mut end_of_file)
                as ::core::ffi::c_uint;
        if end_of_file != 0 {
            safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
            return XDG_MIME_MAGIC_EOF;
        }
        if (*matchlet).range_length == -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint {
            safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
            return XDG_MIME_MAGIC_ERROR;
        }
        c = safe_c2rust_getc_unlocked(magic_file);
    }
    if c == '\n' as i32 {
        if (*matchlet).word_size > 1 as ::core::ffi::c_uint {
            let mut i: ::core::ffi::c_uint = 0;
            if (*matchlet).value_length.wrapping_rem((*matchlet).word_size)
                != 0 as ::core::ffi::c_uint
            {
                safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
                return XDG_MIME_MAGIC_ERROR;
            }
            i = 0 as ::core::ffi::c_uint;
            while i < (*matchlet).value_length {
                if (*matchlet).word_size == 2 as ::core::ffi::c_uint {
                    *((*matchlet).value as *mut xdg_uint16_t).offset(i as isize) =
                        ((*((*matchlet).value.offset(i as isize) as *mut xdg_uint16_t)
                            as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | *((*matchlet).value.offset(i as isize) as *mut xdg_uint16_t)
                                as ::core::ffi::c_int
                                >> 8 as ::core::ffi::c_int) as xdg_uint16_t;
                } else if (*matchlet).word_size == 4 as ::core::ffi::c_uint {
                    *((*matchlet).value as *mut xdg_uint32_t).offset(i as isize) =
                        (*((*matchlet).value.offset(i as isize) as *mut xdg_uint32_t)
                            & 0xff000000 as ::core::ffi::c_uint)
                            >> 24 as ::core::ffi::c_int
                            | (*((*matchlet).value.offset(i as isize) as *mut xdg_uint32_t)
                                & 0xff0000 as ::core::ffi::c_uint)
                                >> 8 as ::core::ffi::c_int
                            | (*((*matchlet).value.offset(i as isize) as *mut xdg_uint32_t)
                                & 0xff00 as ::core::ffi::c_uint)
                                << 8 as ::core::ffi::c_int
                            | (*((*matchlet).value.offset(i as isize) as *mut xdg_uint32_t)
                                & 0xff as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int;
                }
                if !(*matchlet).mask.is_null() {
                    if (*matchlet).word_size == 2 as ::core::ffi::c_uint {
                        *((*matchlet).mask as *mut xdg_uint16_t).offset(i as isize) =
                            ((*((*matchlet).mask.offset(i as isize) as *mut xdg_uint16_t)
                                as ::core::ffi::c_int)
                                << 8 as ::core::ffi::c_int
                                | *((*matchlet).mask.offset(i as isize) as *mut xdg_uint16_t)
                                    as ::core::ffi::c_int
                                    >> 8 as ::core::ffi::c_int)
                                as xdg_uint16_t;
                    } else if (*matchlet).word_size == 4 as ::core::ffi::c_uint {
                        *((*matchlet).mask as *mut xdg_uint32_t).offset(i as isize) =
                            (*((*matchlet).mask.offset(i as isize) as *mut xdg_uint32_t)
                                & 0xff000000 as ::core::ffi::c_uint)
                                >> 24 as ::core::ffi::c_int
                                | (*((*matchlet).mask.offset(i as isize) as *mut xdg_uint32_t)
                                    & 0xff0000 as ::core::ffi::c_uint)
                                    >> 8 as ::core::ffi::c_int
                                | (*((*matchlet).mask.offset(i as isize) as *mut xdg_uint32_t)
                                    & 0xff00 as ::core::ffi::c_uint)
                                    << 8 as ::core::ffi::c_int
                                | (*((*matchlet).mask.offset(i as isize) as *mut xdg_uint32_t)
                                    & 0xff as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int;
                    }
                }
                i = i.wrapping_add((*matchlet).word_size);
            }
        }
        (*matchlet).next = (*match_0).matchlet;
        (*match_0).matchlet = matchlet;
        return XDG_MIME_MAGIC_MAGIC;
    }
    safe_c2rust__xdg_mime_magic_matchlet_free(matchlet);
    if c == EOF {
        return XDG_MIME_MAGIC_EOF;
    }
    return XDG_MIME_MAGIC_ERROR;
}
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_matchlet_compare_to_data(
    mut matchlet: *mut XdgMimeMagicMatchlet,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_uint = 0;
    i = (*matchlet).offset as ::core::ffi::c_uint;
    while i < ((*matchlet).offset as ::core::ffi::c_uint).wrapping_add((*matchlet).range_length) {
        let mut valid_matchlet: ::core::ffi::c_int = TRUE;
        if i.wrapping_add((*matchlet).value_length) as size_t > len {
            return FALSE;
        }
        if !(*matchlet).mask.is_null() {
            j = 0 as ::core::ffi::c_uint;
            while j < (*matchlet).value_length {
                if *(*matchlet).value.offset(j as isize) as ::core::ffi::c_int
                    & *(*matchlet).mask.offset(j as isize) as ::core::ffi::c_int
                    != *(data as *mut ::core::ffi::c_uchar).offset(j.wrapping_add(i) as isize)
                        as ::core::ffi::c_int
                        & *(*matchlet).mask.offset(j as isize) as ::core::ffi::c_int
                {
                    valid_matchlet = FALSE;
                    break;
                } else {
                    j = j.wrapping_add(1);
                }
            }
        } else {
            j = 0 as ::core::ffi::c_uint;
            while j < (*matchlet).value_length {
                if *(*matchlet).value.offset(j as isize) as ::core::ffi::c_int
                    != *(data as *mut ::core::ffi::c_uchar).offset(j.wrapping_add(i) as isize)
                        as ::core::ffi::c_int
                {
                    valid_matchlet = FALSE;
                    break;
                } else {
                    j = j.wrapping_add(1);
                }
            }
        }
        if valid_matchlet != 0 {
            return TRUE;
        }
        i = i.wrapping_add(1);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_matchlet_compare_level(
    mut matchlet: *mut XdgMimeMagicMatchlet,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
    mut indent: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    while !matchlet.is_null() && (*matchlet).indent == indent {
        if safe_c2rust__xdg_mime_magic_matchlet_compare_to_data(matchlet, data, len) != 0 {
            if (*matchlet).next.is_null() || (*(*matchlet).next).indent <= indent {
                return TRUE;
            }
            if safe_c2rust__xdg_mime_magic_matchlet_compare_level(
                (*matchlet).next,
                data,
                len,
                indent + 1 as ::core::ffi::c_int,
            ) != 0
            {
                return TRUE;
            }
        }
        loop {
            matchlet = (*matchlet).next;
            if !(!matchlet.is_null() && (*matchlet).indent > indent) {
                break;
            }
        }
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_match_compare_to_data(
    mut match_0: *mut XdgMimeMagicMatch,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) -> ::core::ffi::c_int {
    return safe_c2rust__xdg_mime_magic_matchlet_compare_level(
        (*match_0).matchlet,
        data,
        len,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_insert_match(
    mut mime_magic: *mut XdgMimeMagic,
    mut match_0: *mut XdgMimeMagicMatch,
) {
    let mut list: *mut XdgMimeMagicMatch = ::core::ptr::null_mut::<XdgMimeMagicMatch>();
    if (*mime_magic).match_list.is_null() {
        (*mime_magic).match_list = match_0;
        return;
    }
    if (*match_0).priority > (*(*mime_magic).match_list).priority {
        (*match_0).next = (*mime_magic).match_list;
        (*mime_magic).match_list = match_0;
        return;
    }
    list = (*mime_magic).match_list;
    while !(*list).next.is_null() {
        if (*(*list).next).priority < (*match_0).priority {
            (*match_0).next = (*list).next;
            (*list).next = match_0;
            return;
        }
        list = (*list).next;
    }
    (*list).next = match_0;
    (*match_0).next = ::core::ptr::null_mut::<XdgMimeMagicMatch>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_magic_new() -> *mut XdgMimeMagic {
    return calloc(
        1 as size_t,
        ::core::mem::size_of::<XdgMimeMagic>() as size_t,
    ) as *mut XdgMimeMagic;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_magic_free(mut mime_magic: *mut XdgMimeMagic) {
    if !mime_magic.is_null() {
        safe_c2rust__xdg_mime_magic_match_free((*mime_magic).match_list);
        free(mime_magic as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_magic_get_buffer_extents(
    mut mime_magic: *mut XdgMimeMagic,
) -> ::core::ffi::c_int {
    return (*mime_magic).max_extent;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_magic_lookup_data(
    mut mime_magic: *mut XdgMimeMagic,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
    mut result_prio: *mut ::core::ffi::c_int,
    mut mime_types: *mut *const ::core::ffi::c_char,
    mut n_mime_types: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut match_0: *mut XdgMimeMagicMatch = ::core::ptr::null_mut::<XdgMimeMagicMatch>();
    let mut mime_type: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0;
    let mut prio: ::core::ffi::c_int = 0;
    prio = 0 as ::core::ffi::c_int;
    mime_type = ::core::ptr::null::<::core::ffi::c_char>();
    match_0 = (*mime_magic).match_list;
    while !match_0.is_null() {
        if safe_c2rust__xdg_mime_magic_match_compare_to_data(match_0, data, len) != 0 {
            prio = (*match_0).priority;
            mime_type = (*match_0).mime_type;
            break;
        } else {
            n = 0 as ::core::ffi::c_int;
            while n < n_mime_types {
                if !(*mime_types.offset(n as isize)).is_null()
                    && __gio_xdg_mime_type_equal(
                        *mime_types.offset(n as isize),
                        (*match_0).mime_type,
                    ) != 0
                {
                    let ref mut fresh2 = *mime_types.offset(n as isize);
                    *fresh2 = ::core::ptr::null::<::core::ffi::c_char>();
                }
                n += 1;
            }
            match_0 = (*match_0).next;
        }
    }
    if mime_type.is_null() {
        n = 0 as ::core::ffi::c_int;
        while n < n_mime_types {
            if !(*mime_types.offset(n as isize)).is_null() {
                mime_type = *mime_types.offset(n as isize);
            }
            n += 1;
        }
    }
    if !result_prio.is_null() {
        *result_prio = prio;
    }
    return mime_type;
}
unsafe extern "C" fn safe_c2rust__xdg_mime_update_mime_magic_extents(
    mut mime_magic: *mut XdgMimeMagic,
) {
    let mut match_0: *mut XdgMimeMagicMatch = ::core::ptr::null_mut::<XdgMimeMagicMatch>();
    let mut max_extent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    match_0 = (*mime_magic).match_list;
    while !match_0.is_null() {
        let mut matchlet: *mut XdgMimeMagicMatchlet =
            ::core::ptr::null_mut::<XdgMimeMagicMatchlet>();
        matchlet = (*match_0).matchlet;
        while !matchlet.is_null() {
            let mut extent: ::core::ffi::c_int = 0;
            extent = (*matchlet)
                .value_length
                .wrapping_add((*matchlet).offset as ::core::ffi::c_uint)
                .wrapping_add((*matchlet).range_length) as ::core::ffi::c_int;
            if max_extent < extent {
                max_extent = extent;
            }
            matchlet = (*matchlet).next;
        }
        match_0 = (*match_0).next;
    }
    (*mime_magic).max_extent = max_extent;
}
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_matchlet_mirror(
    mut matchlets: *mut XdgMimeMagicMatchlet,
) -> *mut XdgMimeMagicMatchlet {
    let mut new_list: *mut XdgMimeMagicMatchlet = ::core::ptr::null_mut::<XdgMimeMagicMatchlet>();
    let mut tmp: *mut XdgMimeMagicMatchlet = ::core::ptr::null_mut::<XdgMimeMagicMatchlet>();
    if matchlets.is_null() || (*matchlets).next.is_null() {
        return matchlets;
    }
    new_list = ::core::ptr::null_mut::<XdgMimeMagicMatchlet>();
    tmp = matchlets;
    while !tmp.is_null() {
        let mut matchlet: *mut XdgMimeMagicMatchlet =
            ::core::ptr::null_mut::<XdgMimeMagicMatchlet>();
        matchlet = tmp;
        tmp = (*tmp).next;
        (*matchlet).next = new_list;
        new_list = matchlet;
    }
    return new_list;
}
unsafe extern "C" fn safe_c2rust__xdg_mime_magic_read_magic_file(
    mut mime_magic: *mut XdgMimeMagic,
    mut magic_file: *mut FILE,
) {
    let mut state: XdgMimeMagicState = XDG_MIME_MAGIC_SECTION;
    let mut match_0: *mut XdgMimeMagicMatch = ::core::ptr::null_mut::<XdgMimeMagicMatch>();
    state = XDG_MIME_MAGIC_SECTION;
    while state as ::core::ffi::c_uint
        != XDG_MIME_MAGIC_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        match state as ::core::ffi::c_uint {
            0 => {
                match_0 = safe_c2rust__xdg_mime_magic_match_new();
                if match_0.is_null() {
                    return;
                }
                state = safe_c2rust__xdg_mime_magic_parse_header(magic_file, match_0);
                if state as ::core::ffi::c_uint
                    == XDG_MIME_MAGIC_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
                    || state as ::core::ffi::c_uint
                        == XDG_MIME_MAGIC_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    safe_c2rust__xdg_mime_magic_match_free(match_0);
                }
            }
            1 => {
                state = safe_c2rust__xdg_mime_magic_parse_magic_line(magic_file, match_0);
                if state as ::core::ffi::c_uint
                    == XDG_MIME_MAGIC_SECTION as ::core::ffi::c_int as ::core::ffi::c_uint
                    || state as ::core::ffi::c_uint
                        == XDG_MIME_MAGIC_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
                        && !(*match_0).mime_type.is_null()
                {
                    (*match_0).matchlet =
                        safe_c2rust__xdg_mime_magic_matchlet_mirror((*match_0).matchlet);
                    safe_c2rust__xdg_mime_magic_insert_match(mime_magic, match_0);
                } else if state as ::core::ffi::c_uint
                    == XDG_MIME_MAGIC_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
                    || state as ::core::ffi::c_uint
                        == XDG_MIME_MAGIC_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    safe_c2rust__xdg_mime_magic_match_free(match_0);
                }
            }
            2 => {
                state = safe_c2rust__xdg_mime_magic_parse_error(magic_file);
                '_c2rust_label: {
                    if state as ::core::ffi::c_uint
                        == XDG_MIME_MAGIC_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
                        || state as ::core::ffi::c_uint
                            == XDG_MIME_MAGIC_SECTION as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                    } else {
                        __assert_fail(
                            b"state == XDG_MIME_MAGIC_EOF || state == XDG_MIME_MAGIC_SECTION\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            b"../original/gio/xdgmime/xdgmimemagic.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            788 as ::core::ffi::c_uint,
                            b"void _xdg_mime_magic_read_magic_file(XdgMimeMagic *, FILE *)\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
            }
            3 | _ => {
                '_c2rust_label_0: {
                    __assert_fail(
                        b"0\0" as *const u8 as *const ::core::ffi::c_char,
                        b"../original/gio/xdgmime/xdgmimemagic.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        794 as ::core::ffi::c_uint,
                        b"void _xdg_mime_magic_read_magic_file(XdgMimeMagic *, FILE *)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                };
            }
        }
    }
    safe_c2rust__xdg_mime_update_mime_magic_extents(mime_magic);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_magic_read_from_file(
    mut mime_magic: *mut XdgMimeMagic,
    mut file_name: *const ::core::ffi::c_char,
) {
    let mut magic_file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut header: [::core::ffi::c_char; 12] = [0; 12];
    magic_file = fopen(file_name, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if magic_file.is_null() {
        return;
    }
    if fread(
        &raw mut header as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        1 as size_t,
        12 as size_t,
        magic_file,
    ) == 12 as ::core::ffi::c_ulong
    {
        if memcmp(
            b"MIME-Magic\0\n\0" as *const u8 as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            &raw mut header as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            12 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            safe_c2rust__xdg_mime_magic_read_magic_file(mime_magic, magic_file);
        }
    }
    fclose(magic_file);
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
