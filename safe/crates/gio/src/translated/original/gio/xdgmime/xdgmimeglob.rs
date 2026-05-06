extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    static safe_c2rust___gio_xdg_utf8_skip: *const ::core::ffi::c_char;
    fn __gio_xdg_convert_to_ucs4(
        source: *const ::core::ffi::c_char,
        len: *mut ::core::ffi::c_int,
    ) -> *mut xdg_unichar_t;
    fn __gio_xdg_reverse_ucs4(source: *mut xdg_unichar_t, len: ::core::ffi::c_int);
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn fnmatch(
        __pattern: *const ::core::ffi::c_char,
        __name: *const ::core::ffi::c_char,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XdgGlobHash {
    pub literal_list: *mut XdgGlobList,
    pub simple_node: *mut XdgGlobHashNode,
    pub full_list: *mut XdgGlobList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XdgGlobList {
    pub data: *const ::core::ffi::c_char,
    pub mime_type: *const ::core::ffi::c_char,
    pub weight: ::core::ffi::c_int,
    pub case_sensitive: ::core::ffi::c_int,
    pub next: *mut XdgGlobList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XdgGlobHashNode {
    pub character: xdg_unichar_t,
    pub mime_type: *const ::core::ffi::c_char,
    pub weight: ::core::ffi::c_int,
    pub case_sensitive: ::core::ffi::c_int,
    pub next: *mut XdgGlobHashNode,
    pub child: *mut XdgGlobHashNode,
}
pub type xdg_unichar_t = ::core::ffi::c_uint;
pub type XdgGlobType = ::core::ffi::c_uint;
pub const XDG_GLOB_FULL: XdgGlobType = 2;
pub const XDG_GLOB_SIMPLE: XdgGlobType = 1;
pub const XDG_GLOB_LITERAL: XdgGlobType = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MimeWeight {
    pub mime: *const ::core::ffi::c_char,
    pub weight: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_atoi(
    mut __nptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 85] = unsafe {
    ::core::mem::transmute::<[u8; 85], [::core::ffi::c_char; 85]>(
        *b"void __gio_xdg_hash_append_glob(XdgGlobHash *, const char *, const char *, int, int)\0",
    )
};
unsafe extern "C" fn safe_c2rust__xdg_glob_list_new() -> *mut XdgGlobList {
    let mut new_element: *mut XdgGlobList = ::core::ptr::null_mut::<XdgGlobList>();
    new_element =
        calloc(1 as size_t, ::core::mem::size_of::<XdgGlobList>() as size_t) as *mut XdgGlobList;
    return new_element;
}
unsafe extern "C" fn safe_c2rust__xdg_glob_list_free(mut glob_list: *mut XdgGlobList) {
    let mut ptr: *mut XdgGlobList = ::core::ptr::null_mut::<XdgGlobList>();
    let mut next: *mut XdgGlobList = ::core::ptr::null_mut::<XdgGlobList>();
    ptr = glob_list;
    while !ptr.is_null() {
        next = (*ptr).next;
        if !(*ptr).data.is_null() {
            free((*ptr).data as *mut ::core::ffi::c_void);
        }
        if !(*ptr).mime_type.is_null() {
            free((*ptr).mime_type as *mut ::core::ffi::c_void);
        }
        free(ptr as *mut ::core::ffi::c_void);
        ptr = next;
    }
}
unsafe extern "C" fn safe_c2rust__xdg_glob_list_append(
    mut glob_list: *mut XdgGlobList,
    mut data: *mut ::core::ffi::c_void,
    mut mime_type: *const ::core::ffi::c_char,
    mut weight: ::core::ffi::c_int,
    mut case_sensitive: ::core::ffi::c_int,
) -> *mut XdgGlobList {
    let mut new_element: *mut XdgGlobList = ::core::ptr::null_mut::<XdgGlobList>();
    let mut tmp_element: *mut XdgGlobList = ::core::ptr::null_mut::<XdgGlobList>();
    tmp_element = glob_list;
    while !tmp_element.is_null() {
        if strcmp((*tmp_element).data, data as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            && strcmp((*tmp_element).mime_type, mime_type) == 0 as ::core::ffi::c_int
        {
            return glob_list;
        }
        tmp_element = (*tmp_element).next;
    }
    new_element = safe_c2rust__xdg_glob_list_new();
    (*new_element).data = data as *const ::core::ffi::c_char;
    (*new_element).mime_type = mime_type;
    (*new_element).weight = weight;
    (*new_element).case_sensitive = case_sensitive;
    if glob_list.is_null() {
        return new_element;
    }
    tmp_element = glob_list;
    while !(*tmp_element).next.is_null() {
        tmp_element = (*tmp_element).next;
    }
    (*tmp_element).next = new_element;
    return glob_list;
}
unsafe extern "C" fn safe_c2rust__xdg_glob_hash_node_new() -> *mut XdgGlobHashNode {
    let mut glob_hash_node: *mut XdgGlobHashNode = ::core::ptr::null_mut::<XdgGlobHashNode>();
    glob_hash_node = calloc(
        1 as size_t,
        ::core::mem::size_of::<XdgGlobHashNode>() as size_t,
    ) as *mut XdgGlobHashNode;
    return glob_hash_node;
}
unsafe extern "C" fn safe_c2rust__xdg_glob_hash_node_dump(
    mut glob_hash_node: *mut XdgGlobHashNode,
    mut depth: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < depth {
        printf(b" \0" as *const u8 as *const ::core::ffi::c_char);
        i += 1;
    }
    printf(
        b"%c\0" as *const u8 as *const ::core::ffi::c_char,
        (*glob_hash_node).character as ::core::ffi::c_char as ::core::ffi::c_int,
    );
    if !(*glob_hash_node).mime_type.is_null() {
        printf(
            b" - %s %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*glob_hash_node).mime_type,
            (*glob_hash_node).weight,
        );
    } else {
        printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if !(*glob_hash_node).child.is_null() {
        safe_c2rust__xdg_glob_hash_node_dump(
            (*glob_hash_node).child,
            depth + 1 as ::core::ffi::c_int,
        );
    }
    if !(*glob_hash_node).next.is_null() {
        safe_c2rust__xdg_glob_hash_node_dump((*glob_hash_node).next, depth);
    }
}
unsafe extern "C" fn safe_c2rust__xdg_glob_hash_insert_ucs4(
    mut glob_hash_node: *mut XdgGlobHashNode,
    mut text: *mut xdg_unichar_t,
    mut mime_type: *const ::core::ffi::c_char,
    mut weight: ::core::ffi::c_int,
    mut case_sensitive: ::core::ffi::c_int,
) -> *mut XdgGlobHashNode {
    let mut node: *mut XdgGlobHashNode = ::core::ptr::null_mut::<XdgGlobHashNode>();
    let mut character: xdg_unichar_t = 0;
    character = *text.offset(0 as ::core::ffi::c_int as isize);
    if glob_hash_node.is_null() || character < (*glob_hash_node).character {
        node = safe_c2rust__xdg_glob_hash_node_new();
        (*node).character = character;
        (*node).next = glob_hash_node;
        glob_hash_node = node;
    } else if character == (*glob_hash_node).character {
        node = glob_hash_node;
    } else {
        let mut prev_node: *mut XdgGlobHashNode = ::core::ptr::null_mut::<XdgGlobHashNode>();
        let mut found_node: ::core::ffi::c_int = FALSE;
        prev_node = glob_hash_node;
        node = (*prev_node).next;
        while !node.is_null() {
            if character < (*node).character {
                node = safe_c2rust__xdg_glob_hash_node_new();
                (*node).character = character;
                (*node).next = (*prev_node).next;
                (*prev_node).next = node;
                found_node = TRUE;
                break;
            } else if character == (*node).character {
                found_node = TRUE;
                break;
            } else {
                prev_node = node;
                node = (*node).next;
            }
        }
        if found_node == 0 {
            node = safe_c2rust__xdg_glob_hash_node_new();
            (*node).character = character;
            (*node).next = (*prev_node).next;
            (*prev_node).next = node;
        }
    }
    text = text.offset(1);
    if *text == 0 as xdg_unichar_t {
        if !(*node).mime_type.is_null() {
            if strcmp((*node).mime_type, mime_type) != 0 as ::core::ffi::c_int {
                let mut child: *mut XdgGlobHashNode = ::core::ptr::null_mut::<XdgGlobHashNode>();
                let mut found_node_0: ::core::ffi::c_int = FALSE;
                child = (*node).child;
                while !child.is_null() && (*child).character == 0 as xdg_unichar_t {
                    if strcmp((*child).mime_type, mime_type) == 0 as ::core::ffi::c_int {
                        found_node_0 = TRUE;
                        break;
                    } else {
                        child = (*child).next;
                    }
                }
                if found_node_0 == 0 {
                    child = safe_c2rust__xdg_glob_hash_node_new();
                    (*child).character = 0 as xdg_unichar_t;
                    (*child).mime_type = strdup(mime_type);
                    (*child).weight = weight;
                    (*child).case_sensitive = case_sensitive;
                    (*child).child = ::core::ptr::null_mut::<XdgGlobHashNode>();
                    (*child).next = (*node).child;
                    (*node).child = child;
                }
            }
        } else {
            (*node).mime_type = strdup(mime_type);
            (*node).weight = weight;
            (*node).case_sensitive = case_sensitive;
        }
    } else {
        (*node).child = safe_c2rust__xdg_glob_hash_insert_ucs4(
            (*node).child,
            text,
            mime_type,
            weight,
            case_sensitive,
        );
    }
    return glob_hash_node;
}
unsafe extern "C" fn safe_c2rust__xdg_glob_hash_insert_text(
    mut glob_hash_node: *mut XdgGlobHashNode,
    mut text: *const ::core::ffi::c_char,
    mut mime_type: *const ::core::ffi::c_char,
    mut weight: ::core::ffi::c_int,
    mut case_sensitive: ::core::ffi::c_int,
) -> *mut XdgGlobHashNode {
    let mut node: *mut XdgGlobHashNode = ::core::ptr::null_mut::<XdgGlobHashNode>();
    let mut unitext: *mut xdg_unichar_t = ::core::ptr::null_mut::<xdg_unichar_t>();
    let mut len: ::core::ffi::c_int = 0;
    unitext = __gio_xdg_convert_to_ucs4(text, &raw mut len);
    __gio_xdg_reverse_ucs4(unitext, len);
    node = safe_c2rust__xdg_glob_hash_insert_ucs4(
        glob_hash_node,
        unitext,
        mime_type,
        weight,
        case_sensitive,
    );
    free(unitext as *mut ::core::ffi::c_void);
    return node;
}
unsafe extern "C" fn safe_c2rust__xdg_glob_hash_node_lookup_file_name(
    mut glob_hash_node: *mut XdgGlobHashNode,
    mut file_name: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut case_sensitive_check: ::core::ffi::c_int,
    mut mime_types: *mut MimeWeight,
    mut n_mime_types: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    let mut node: *mut XdgGlobHashNode = ::core::ptr::null_mut::<XdgGlobHashNode>();
    let mut character: xdg_unichar_t = 0;
    if glob_hash_node.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    character = *file_name.offset((len - 1 as ::core::ffi::c_int) as isize) as xdg_unichar_t;
    node = glob_hash_node;
    while !node.is_null() && character >= (*node).character {
        if character == (*node).character {
            len -= 1;
            n = 0 as ::core::ffi::c_int;
            if len > 0 as ::core::ffi::c_int {
                n = safe_c2rust__xdg_glob_hash_node_lookup_file_name(
                    (*node).child,
                    file_name,
                    len,
                    case_sensitive_check,
                    mime_types,
                    n_mime_types,
                );
            }
            if n == 0 as ::core::ffi::c_int {
                if !(*node).mime_type.is_null()
                    && (case_sensitive_check != 0 || (*node).case_sensitive == 0)
                {
                    let ref mut fresh4 = (*mime_types.offset(n as isize)).mime;
                    *fresh4 = (*node).mime_type;
                    (*mime_types.offset(n as isize)).weight = (*node).weight;
                    n += 1;
                }
                node = (*node).child;
                while n < n_mime_types && !node.is_null() && (*node).character == 0 as xdg_unichar_t
                {
                    if !(*node).mime_type.is_null()
                        && (case_sensitive_check != 0 || (*node).case_sensitive == 0)
                    {
                        let ref mut fresh5 = (*mime_types.offset(n as isize)).mime;
                        *fresh5 = (*node).mime_type;
                        (*mime_types.offset(n as isize)).weight = (*node).weight;
                        n += 1;
                    }
                    node = (*node).next;
                }
            }
            return n;
        }
        node = (*node).next;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_compare_mime_weight(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut aa: *const MimeWeight = a as *const MimeWeight;
    let mut bb: *const MimeWeight = b as *const MimeWeight;
    return (*bb).weight - (*aa).weight;
}
unsafe extern "C" fn safe_c2rust_ascii_tolower(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut lower: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    lower = strdup(str);
    p = lower;
    while *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        let mut c: ::core::ffi::c_char = *p;
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = (if c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'Z' as i32
        {
            c as ::core::ffi::c_int - 'A' as i32 + 'a' as i32
        } else {
            c as ::core::ffi::c_int
        }) as ::core::ffi::c_char;
    }
    return lower;
}
unsafe extern "C" fn safe_c2rust_filter_out_dupes(
    mut mimes: *mut MimeWeight,
    mut n_mimes: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut last: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    last = n_mimes;
    i = 0 as ::core::ffi::c_int;
    while i < last {
        j = i + 1 as ::core::ffi::c_int;
        while j < last {
            if strcmp(
                (*mimes.offset(i as isize)).mime,
                (*mimes.offset(j as isize)).mime,
            ) == 0 as ::core::ffi::c_int
            {
                (*mimes.offset(i as isize)).weight =
                    if (*mimes.offset(i as isize)).weight > (*mimes.offset(j as isize)).weight {
                        (*mimes.offset(i as isize)).weight
                    } else {
                        (*mimes.offset(j as isize)).weight
                    };
                last -= 1;
                let ref mut fresh3 = (*mimes.offset(j as isize)).mime;
                *fresh3 = (*mimes.offset(last as isize)).mime;
                (*mimes.offset(j as isize)).weight = (*mimes.offset(last as isize)).weight;
            } else {
                j += 1;
            }
        }
        i += 1;
    }
    return last;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_hash_lookup_file_name(
    mut glob_hash: *mut XdgGlobHash,
    mut file_name: *const ::core::ffi::c_char,
    mut mime_types: *mut *const ::core::ffi::c_char,
    mut n_mime_types: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut list: *mut XdgGlobList = ::core::ptr::null_mut::<XdgGlobList>();
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut mimes: [MimeWeight; 10] = [MimeWeight {
        mime: ::core::ptr::null::<::core::ffi::c_char>(),
        weight: 0,
    }; 10];
    let mut n_mimes: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_int = 0;
    let mut lower_case: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    '_c2rust_label: {
        if !file_name.is_null() && n_mime_types > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"file_name != NULL && n_mime_types > 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../original/gio/xdgmime/xdgmimeglob.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                434 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    n = 0 as ::core::ffi::c_int;
    lower_case = safe_c2rust_ascii_tolower(file_name);
    list = (*glob_hash).literal_list;
    while !list.is_null() {
        if strcmp((*list).data, file_name) == 0 as ::core::ffi::c_int {
            let ref mut fresh0 = *mime_types.offset(0 as ::core::ffi::c_int as isize);
            *fresh0 = (*list).mime_type;
            free(lower_case as *mut ::core::ffi::c_void);
            return 1 as ::core::ffi::c_int;
        }
        list = (*list).next;
    }
    list = (*glob_hash).literal_list;
    while !list.is_null() {
        if (*list).case_sensitive == 0
            && strcmp((*list).data, lower_case) == 0 as ::core::ffi::c_int
        {
            let ref mut fresh1 = *mime_types.offset(0 as ::core::ffi::c_int as isize);
            *fresh1 = (*list).mime_type;
            free(lower_case as *mut ::core::ffi::c_void);
            return 1 as ::core::ffi::c_int;
        }
        list = (*list).next;
    }
    len = strlen(file_name) as ::core::ffi::c_int;
    n = safe_c2rust__xdg_glob_hash_node_lookup_file_name(
        (*glob_hash).simple_node,
        lower_case,
        len,
        FALSE,
        &raw mut mimes as *mut MimeWeight,
        n_mimes,
    );
    if n < 2 as ::core::ffi::c_int {
        n += safe_c2rust__xdg_glob_hash_node_lookup_file_name(
            (*glob_hash).simple_node,
            file_name,
            len,
            TRUE,
            (&raw mut mimes as *mut MimeWeight).offset(n as isize),
            n_mimes - n,
        );
    }
    if n < 2 as ::core::ffi::c_int {
        list = (*glob_hash).full_list;
        while !list.is_null() && n < n_mime_types {
            if fnmatch((*list).data, file_name, 0 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int
            {
                mimes[n as usize].mime = (*list).mime_type;
                mimes[n as usize].weight = (*list).weight;
                n += 1;
            }
            list = (*list).next;
        }
    }
    free(lower_case as *mut ::core::ffi::c_void);
    n = safe_c2rust_filter_out_dupes(&raw mut mimes as *mut MimeWeight, n);
    qsort(
        &raw mut mimes as *mut MimeWeight as *mut ::core::ffi::c_void,
        n as size_t,
        ::core::mem::size_of::<MimeWeight>() as size_t,
        Some(
            safe_c2rust_compare_mime_weight
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    if n_mime_types < n {
        n = n_mime_types;
    }
    i = 0 as ::core::ffi::c_int;
    while i < n {
        let ref mut fresh2 = *mime_types.offset(i as isize);
        *fresh2 = mimes[i as usize].mime;
        i += 1;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_hash_new() -> *mut XdgGlobHash {
    let mut glob_hash: *mut XdgGlobHash = ::core::ptr::null_mut::<XdgGlobHash>();
    glob_hash =
        calloc(1 as size_t, ::core::mem::size_of::<XdgGlobHash>() as size_t) as *mut XdgGlobHash;
    return glob_hash;
}
unsafe extern "C" fn safe_c2rust__xdg_glob_hash_free_nodes(mut node: *mut XdgGlobHashNode) {
    if !node.is_null() {
        if !(*node).child.is_null() {
            safe_c2rust__xdg_glob_hash_free_nodes((*node).child);
        }
        if !(*node).next.is_null() {
            safe_c2rust__xdg_glob_hash_free_nodes((*node).next);
        }
        if !(*node).mime_type.is_null() {
            free((*node).mime_type as *mut ::core::ffi::c_void);
        }
        free(node as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_hash_free(mut glob_hash: *mut XdgGlobHash) {
    safe_c2rust__xdg_glob_list_free((*glob_hash).literal_list);
    safe_c2rust__xdg_glob_list_free((*glob_hash).full_list);
    safe_c2rust__xdg_glob_hash_free_nodes((*glob_hash).simple_node);
    free(glob_hash as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_determine_type(
    mut glob: *const ::core::ffi::c_char,
) -> XdgGlobType {
    let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut maybe_in_simple_glob: ::core::ffi::c_int = FALSE;
    let mut first_char: ::core::ffi::c_int = TRUE;
    ptr = glob;
    while *ptr as ::core::ffi::c_int != '\0' as i32 {
        if *ptr as ::core::ffi::c_int == '*' as i32 && first_char != 0 {
            maybe_in_simple_glob = TRUE;
        } else if *ptr as ::core::ffi::c_int == '\\' as i32
            || *ptr as ::core::ffi::c_int == '[' as i32
            || *ptr as ::core::ffi::c_int == '?' as i32
            || *ptr as ::core::ffi::c_int == '*' as i32
        {
            return XDG_GLOB_FULL;
        }
        first_char = FALSE;
        ptr = ptr.offset(
            *safe_c2rust___gio_xdg_utf8_skip.offset(*(ptr as *mut ::core::ffi::c_uchar) as isize)
                as ::core::ffi::c_int as isize,
        ) as *mut ::core::ffi::c_char;
    }
    if maybe_in_simple_glob != 0 {
        return XDG_GLOB_SIMPLE;
    } else {
        return XDG_GLOB_LITERAL;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_hash_append_glob(
    mut glob_hash: *mut XdgGlobHash,
    mut glob: *const ::core::ffi::c_char,
    mut mime_type: *const ::core::ffi::c_char,
    mut weight: ::core::ffi::c_int,
    mut case_sensitive: ::core::ffi::c_int,
) {
    let mut type_0: XdgGlobType = XDG_GLOB_LITERAL;
    '_c2rust_label: {
        if !glob_hash.is_null() {
        } else {
            __assert_fail(
                b"glob_hash != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"../original/gio/xdgmime/xdgmimeglob.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                571 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_0: {
        if !glob.is_null() {
        } else {
            __assert_fail(
                b"glob != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"../original/gio/xdgmime/xdgmimeglob.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                572 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    type_0 = safe_c2rust___gio_xdg_determine_type(glob);
    match type_0 as ::core::ffi::c_uint {
        0 => {
            (*glob_hash).literal_list = safe_c2rust__xdg_glob_list_append(
                (*glob_hash).literal_list,
                strdup(glob) as *mut ::core::ffi::c_void,
                strdup(mime_type),
                weight,
                case_sensitive,
            );
        }
        1 => {
            (*glob_hash).simple_node = safe_c2rust__xdg_glob_hash_insert_text(
                (*glob_hash).simple_node,
                glob.offset(1 as ::core::ffi::c_int as isize),
                mime_type,
                weight,
                case_sensitive,
            );
        }
        2 => {
            (*glob_hash).full_list = safe_c2rust__xdg_glob_list_append(
                (*glob_hash).full_list,
                strdup(glob) as *mut ::core::ffi::c_void,
                strdup(mime_type),
                weight,
                case_sensitive,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_hash_dump(mut glob_hash: *mut XdgGlobHash) {
    let mut list: *mut XdgGlobList = ::core::ptr::null_mut::<XdgGlobList>();
    printf(b"LITERAL STRINGS\n\0" as *const u8 as *const ::core::ffi::c_char);
    if glob_hash.is_null() || (*glob_hash).literal_list.is_null() {
        printf(b"    None\n\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        list = (*glob_hash).literal_list;
        while !list.is_null() {
            printf(
                b"    %s - %s %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*list).data as *mut ::core::ffi::c_char,
                (*list).mime_type,
                (*list).weight,
            );
            list = (*list).next;
        }
    }
    printf(b"\nSIMPLE GLOBS\n\0" as *const u8 as *const ::core::ffi::c_char);
    if glob_hash.is_null() || (*glob_hash).simple_node.is_null() {
        printf(b"    None\n\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        safe_c2rust__xdg_glob_hash_node_dump((*glob_hash).simple_node, 4 as ::core::ffi::c_int);
    }
    printf(b"\nFULL GLOBS\n\0" as *const u8 as *const ::core::ffi::c_char);
    if glob_hash.is_null() || (*glob_hash).full_list.is_null() {
        printf(b"    None\n\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        list = (*glob_hash).full_list;
        while !list.is_null() {
            printf(
                b"    %s - %s %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*list).data as *mut ::core::ffi::c_char,
                (*list).mime_type,
                (*list).weight,
            );
            list = (*list).next;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_glob_read_from_file(
    mut glob_hash: *mut XdgGlobHash,
    mut file_name: *const ::core::ffi::c_char,
    mut version_two: ::core::ffi::c_int,
) {
    let mut glob_file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut line: [::core::ffi::c_char; 255] = [0; 255];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    glob_file = fopen(file_name, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if glob_file.is_null() {
        return;
    }
    while !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        255 as ::core::ffi::c_int,
        glob_file,
    )
    .is_null()
    {
        let mut colon: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut mimetype: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut glob: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut weight: ::core::ffi::c_int = 0;
        let mut case_sensitive: ::core::ffi::c_int = 0;
        if line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '#' as i32
            || line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            continue;
        }
        end = (&raw mut line as *mut ::core::ffi::c_char)
            .offset(strlen(&raw mut line as *mut ::core::ffi::c_char) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        if *end as ::core::ffi::c_int == '\n' as i32 {
            *end = 0 as ::core::ffi::c_char;
        }
        p = &raw mut line as *mut ::core::ffi::c_char;
        if version_two != 0 {
            colon = strchr(p, ':' as i32);
            if colon.is_null() {
                continue;
            }
            *colon = 0 as ::core::ffi::c_char;
            weight = safe_c2rust_atoi(p);
            p = colon.offset(1 as ::core::ffi::c_int as isize);
        } else {
            weight = 50 as ::core::ffi::c_int;
        }
        colon = strchr(p, ':' as i32);
        if colon.is_null() {
            continue;
        }
        *colon = 0 as ::core::ffi::c_char;
        mimetype = p;
        p = colon.offset(1 as ::core::ffi::c_int as isize);
        glob = p;
        case_sensitive = FALSE;
        colon = strchr(p, ':' as i32);
        if version_two != 0 && !colon.is_null() {
            let mut flag: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            *colon = 0 as ::core::ffi::c_char;
            p = colon.offset(1 as ::core::ffi::c_int as isize);
            colon = strchr(p, ':' as i32);
            if !colon.is_null() {
                *colon = 0 as ::core::ffi::c_char;
            }
            flag = strstr(p, b"cs\0" as *const u8 as *const ::core::ffi::c_char);
            if !flag.is_null()
                && (flag == p
                    || *flag.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                        == ',' as i32)
                && (*flag.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    || *flag.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ',' as i32)
            {
                case_sensitive = TRUE;
            }
        }
        safe_c2rust___gio_xdg_hash_append_glob(glob_hash, glob, mimetype, weight, case_sensitive);
    }
    fclose(glob_file);
}
