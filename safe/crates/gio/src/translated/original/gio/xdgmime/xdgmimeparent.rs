extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
pub struct XdgParentList {
    pub parents: *mut XdgMimeParents,
    pub n_mimes: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XdgMimeParents {
    pub mime: *mut ::core::ffi::c_char,
    pub parents: *mut *mut ::core::ffi::c_char,
    pub n_parents: ::core::ffi::c_int,
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_bsearch(
    mut __key: *const ::core::ffi::c_void,
    mut __base: *const ::core::ffi::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut ::core::ffi::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>();
    let mut __comparison: ::core::ffi::c_int = 0;
    __l = 0 as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2 as size_t);
        __p = (__base as *const ::core::ffi::c_char).offset(__idx.wrapping_mul(__size) as isize)
            as *const ::core::ffi::c_void;
        __comparison = Some(__compar.expect("non-null function pointer"))
            .expect("non-null function pointer")(__key, __p);
        if __comparison < 0 as ::core::ffi::c_int {
            __u = __idx;
        } else if __comparison > 0 as ::core::ffi::c_int {
            __l = __idx.wrapping_add(1 as size_t);
        } else {
            return __p as *mut ::core::ffi::c_void;
        }
    }
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_parent_list_new() -> *mut XdgParentList {
    let mut list: *mut XdgParentList = ::core::ptr::null_mut::<XdgParentList>();
    list = malloc(::core::mem::size_of::<XdgParentList>() as size_t) as *mut XdgParentList;
    (*list).parents = ::core::ptr::null_mut::<XdgMimeParents>();
    (*list).n_mimes = 0 as ::core::ffi::c_int;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_parent_list_free(mut list: *mut XdgParentList) {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    if !(*list).parents.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*list).n_mimes {
            p = (*(*list).parents.offset(i as isize)).parents;
            while !(*p).is_null() {
                free(*p as *mut ::core::ffi::c_void);
                p = p.offset(1);
            }
            free((*(*list).parents.offset(i as isize)).parents as *mut ::core::ffi::c_void);
            free((*(*list).parents.offset(i as isize)).mime as *mut ::core::ffi::c_void);
            i += 1;
        }
        free((*list).parents as *mut ::core::ffi::c_void);
    }
    free(list as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn safe_c2rust_parent_entry_cmp(
    mut v1: *const ::core::ffi::c_void,
    mut v2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        (*(v1 as *mut XdgMimeParents)).mime,
        (*(v2 as *mut XdgMimeParents)).mime,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_parent_list_lookup(
    mut list: *mut XdgParentList,
    mut mime: *const ::core::ffi::c_char,
) -> *mut *const ::core::ffi::c_char {
    let mut entry: *mut XdgMimeParents = ::core::ptr::null_mut::<XdgMimeParents>();
    let mut key: XdgMimeParents = XdgMimeParents {
        mime: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        parents: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        n_parents: 0,
    };
    if (*list).n_mimes > 0 as ::core::ffi::c_int {
        key.mime = mime as *mut ::core::ffi::c_char;
        key.parents = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        key.n_parents = 0 as ::core::ffi::c_int;
        entry = safe_c2rust_bsearch(
            &raw mut key as *const ::core::ffi::c_void,
            (*list).parents as *const ::core::ffi::c_void,
            (*list).n_mimes as size_t,
            ::core::mem::size_of::<XdgMimeParents>() as size_t,
            Some(
                safe_c2rust_parent_entry_cmp
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        ) as *mut XdgMimeParents;
        if !entry.is_null() {
            return (*entry).parents as *mut *const ::core::ffi::c_char;
        }
    }
    return ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_parent_read_from_file(
    mut list: *mut XdgParentList,
    mut file_name: *const ::core::ffi::c_char,
) {
    let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut line: [::core::ffi::c_char; 255] = [0; 255];
    let mut i: ::core::ffi::c_int = 0;
    let mut alloc: ::core::ffi::c_int = 0;
    let mut entry: *mut XdgMimeParents = ::core::ptr::null_mut::<XdgMimeParents>();
    file = fopen(file_name, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if file.is_null() {
        return;
    }
    alloc = (*list).n_mimes + 16 as ::core::ffi::c_int;
    (*list).parents = realloc(
        (*list).parents as *mut ::core::ffi::c_void,
        (alloc as size_t).wrapping_mul(::core::mem::size_of::<XdgMimeParents>() as size_t),
    ) as *mut XdgMimeParents;
    while !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        255 as ::core::ffi::c_int,
        file,
    )
    .is_null()
    {
        let mut sep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '#' as i32 {
            continue;
        }
        sep = strchr(&raw mut line as *mut ::core::ffi::c_char, ' ' as i32);
        if sep.is_null() {
            continue;
        }
        let fresh0 = sep;
        sep = sep.offset(1);
        *fresh0 = '\0' as i32 as ::core::ffi::c_char;
        *sep.offset(strlen(sep).wrapping_sub(1 as size_t) as isize) =
            '\0' as i32 as ::core::ffi::c_char;
        entry = ::core::ptr::null_mut::<XdgMimeParents>();
        i = 0 as ::core::ffi::c_int;
        while i < (*list).n_mimes {
            if strcmp(
                (*(*list).parents.offset(i as isize)).mime,
                &raw mut line as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                entry = (*list).parents.offset(i as isize) as *mut XdgMimeParents
                    as *mut XdgMimeParents;
                break;
            } else {
                i += 1;
            }
        }
        if entry.is_null() {
            if (*list).n_mimes == alloc {
                alloc <<= 1 as ::core::ffi::c_int;
                (*list).parents = realloc(
                    (*list).parents as *mut ::core::ffi::c_void,
                    (alloc as size_t)
                        .wrapping_mul(::core::mem::size_of::<XdgMimeParents>() as size_t),
                ) as *mut XdgMimeParents;
            }
            let ref mut fresh1 = (*(*list).parents.offset((*list).n_mimes as isize)).mime;
            *fresh1 = strdup(&raw mut line as *mut ::core::ffi::c_char);
            let ref mut fresh2 = (*(*list).parents.offset((*list).n_mimes as isize)).parents;
            *fresh2 = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
            entry = (*list).parents.offset((*list).n_mimes as isize) as *mut XdgMimeParents
                as *mut XdgMimeParents;
            (*list).n_mimes += 1;
        }
        if (*entry).parents.is_null() {
            (*entry).n_parents = 1 as ::core::ffi::c_int;
            (*entry).parents = malloc(
                (((*entry).n_parents + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
            ) as *mut *mut ::core::ffi::c_char;
        } else {
            (*entry).n_parents += 1 as ::core::ffi::c_int;
            (*entry).parents = realloc(
                (*entry).parents as *mut ::core::ffi::c_void,
                (((*entry).n_parents + 2 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
            ) as *mut *mut ::core::ffi::c_char;
        }
        let ref mut fresh3 = *(*entry)
            .parents
            .offset(((*entry).n_parents - 1 as ::core::ffi::c_int) as isize);
        *fresh3 = strdup(sep);
        let ref mut fresh4 = *(*entry).parents.offset((*entry).n_parents as isize);
        *fresh4 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    (*list).parents = realloc(
        (*list).parents as *mut ::core::ffi::c_void,
        ((*list).n_mimes as size_t)
            .wrapping_mul(::core::mem::size_of::<XdgMimeParents>() as size_t),
    ) as *mut XdgMimeParents;
    fclose(file);
    if (*list).n_mimes > 1 as ::core::ffi::c_int {
        qsort(
            (*list).parents as *mut ::core::ffi::c_void,
            (*list).n_mimes as size_t,
            ::core::mem::size_of::<XdgMimeParents>() as size_t,
            Some(
                safe_c2rust_parent_entry_cmp
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_parent_list_dump(mut list: *mut XdgParentList) {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    if !(*list).parents.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*list).n_mimes {
            p = (*(*list).parents.offset(i as isize)).parents;
            while !(*p).is_null() {
                printf(
                    b"%s %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*(*list).parents.offset(i as isize)).mime,
                    *p,
                );
                p = p.offset(1);
            }
            i += 1;
        }
    }
}
