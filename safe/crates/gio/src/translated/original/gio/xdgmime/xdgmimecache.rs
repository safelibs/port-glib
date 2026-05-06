extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fnmatch(
        __pattern: *const ::core::ffi::c_char,
        __name: *const ::core::ffi::c_char,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
    ) -> *mut ::core::ffi::c_void;
    fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    static safe_c2rust__gio_xdg_type_unknown: [::core::ffi::c_char; 0];
    static safe_c2rust__gio_xdg_type_empty: [::core::ffi::c_char; 0];
    fn _gio_xdg_media_type_equal(
        mime_a: *const ::core::ffi::c_char,
        mime_b: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _gio_xdg_unalias_mime_type(mime: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    static mut safe_c2rust__caches: *mut *mut XdgMimeCache;
    fn __gio_xdg_utf8_validate(source: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn __gio_xdg_get_base_name(file_name: *const ::core::ffi::c_char)
        -> *const ::core::ffi::c_char;
    fn _xdg_binary_or_text_fallback(
        data: *const ::core::ffi::c_void,
        len: size_t,
    ) -> *const ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _XdgMimeCache {
    pub ref_count: ::core::ffi::c_int,
    pub minor: ::core::ffi::c_int,
    pub size: size_t,
    pub buffer: *mut ::core::ffi::c_char,
}
pub type XdgMimeCache = _XdgMimeCache;
pub type xdg_uint16_t = ::core::ffi::c_ushort;
pub type xdg_uint32_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MimeWeight {
    pub mime: *const ::core::ffi::c_char,
    pub weight: ::core::ffi::c_int,
}
pub type xdg_unichar_t = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust___bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn safe_c2rust___bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as __uint32_t) >> 24 as ::core::ffi::c_int
        | (__bsx & 0xff0000 as __uint32_t) >> 8 as ::core::ffi::c_int
        | (__bsx & 0xff00 as __uint32_t) << 8 as ::core::ffi::c_int
        | (__bsx & 0xff as __uint32_t) << 24 as ::core::ffi::c_int;
}
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MAP_SHARED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MAP_FAILED: *mut ::core::ffi::c_void =
    -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_void;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const _O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAJOR_VERSION: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINOR_VERSION_MIN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MINOR_VERSION_MAX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_cache_ref(
    mut cache: *mut XdgMimeCache,
) -> *mut XdgMimeCache {
    (*cache).ref_count += 1;
    return cache;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_cache_unref(mut cache: *mut XdgMimeCache) {
    (*cache).ref_count -= 1;
    if (*cache).ref_count == 0 as ::core::ffi::c_int {
        munmap((*cache).buffer as *mut ::core::ffi::c_void, (*cache).size);
        free(cache as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_cache_new_from_file(
    mut file_name: *const ::core::ffi::c_char,
) -> *mut XdgMimeCache {
    let mut cache: *mut XdgMimeCache = ::core::ptr::null_mut::<XdgMimeCache>();
    let mut fd: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut minor: ::core::ffi::c_int = 0;
    loop {
        fd = open(file_name, O_RDONLY | _O_BINARY, 0 as ::core::ffi::c_int);
        if !(fd == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    if fd < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<XdgMimeCache>();
    }
    if !(fstat(fd, &raw mut st) < 0 as ::core::ffi::c_int || st.st_size < 40 as __off_t) {
        buffer = mmap(
            NULL,
            st.st_size as size_t,
            PROT_READ,
            MAP_SHARED,
            fd,
            0 as __off64_t,
        ) as *mut ::core::ffi::c_char;
        if !(buffer == MAP_FAILED as *mut ::core::ffi::c_char) {
            minor = safe_c2rust___bswap_16(
                *(buffer.offset(2 as ::core::ffi::c_int as isize) as *mut xdg_uint16_t),
            ) as ::core::ffi::c_int;
            if safe_c2rust___bswap_16(
                *(buffer.offset(0 as ::core::ffi::c_int as isize) as *mut xdg_uint16_t),
            ) as ::core::ffi::c_int
                != MAJOR_VERSION
                || (minor < MINOR_VERSION_MIN || minor > MINOR_VERSION_MAX)
            {
                munmap(buffer as *mut ::core::ffi::c_void, st.st_size as size_t);
            } else {
                cache =
                    malloc(::core::mem::size_of::<XdgMimeCache>() as size_t) as *mut XdgMimeCache;
                (*cache).minor = minor;
                (*cache).ref_count = 1 as ::core::ffi::c_int;
                (*cache).buffer = buffer;
                (*cache).size = st.st_size as size_t;
            }
        }
    }
    if fd != -(1 as ::core::ffi::c_int) {
        close(fd);
    }
    return cache;
}
unsafe extern "C" fn safe_c2rust_cache_magic_matchlet_compare_to_data(
    mut cache: *mut XdgMimeCache,
    mut offset: xdg_uint32_t,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut range_start: xdg_uint32_t =
        safe_c2rust___bswap_32(*((*cache).buffer.offset(offset as isize) as *mut xdg_uint32_t))
            as xdg_uint32_t;
    let mut range_length: xdg_uint32_t = safe_c2rust___bswap_32(
        *((*cache)
            .buffer
            .offset(offset.wrapping_add(4 as xdg_uint32_t) as isize)
            as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    let mut data_length: xdg_uint32_t = safe_c2rust___bswap_32(
        *((*cache)
            .buffer
            .offset(offset.wrapping_add(12 as xdg_uint32_t) as isize)
            as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    let mut data_offset: xdg_uint32_t = safe_c2rust___bswap_32(
        *((*cache)
            .buffer
            .offset(offset.wrapping_add(16 as xdg_uint32_t) as isize)
            as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    let mut mask_offset: xdg_uint32_t = safe_c2rust___bswap_32(
        *((*cache)
            .buffer
            .offset(offset.wrapping_add(20 as xdg_uint32_t) as isize)
            as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    let mut i: xdg_uint32_t = 0;
    let mut j: xdg_uint32_t = 0;
    i = range_start;
    while i < range_start.wrapping_add(range_length) {
        let mut valid_matchlet: ::core::ffi::c_int = TRUE;
        if i.wrapping_add(data_length) as size_t > len {
            return FALSE;
        }
        if mask_offset != 0 {
            j = 0 as xdg_uint32_t;
            while j < data_length {
                if *((*cache).buffer as *mut ::core::ffi::c_uchar)
                    .offset(data_offset.wrapping_add(j) as isize)
                    as ::core::ffi::c_int
                    & *((*cache).buffer as *mut ::core::ffi::c_uchar)
                        .offset(mask_offset.wrapping_add(j) as isize)
                        as ::core::ffi::c_int
                    != *(data as *mut ::core::ffi::c_uchar).offset(j.wrapping_add(i) as isize)
                        as ::core::ffi::c_int
                        & *((*cache).buffer as *mut ::core::ffi::c_uchar)
                            .offset(mask_offset.wrapping_add(j) as isize)
                            as ::core::ffi::c_int
                {
                    valid_matchlet = FALSE;
                    break;
                } else {
                    j = j.wrapping_add(1);
                }
            }
        } else {
            valid_matchlet = (memcmp(
                (*cache).buffer.offset(data_offset as isize) as *const ::core::ffi::c_void,
                (data as *mut ::core::ffi::c_uchar).offset(i as isize)
                    as *const ::core::ffi::c_void,
                data_length as size_t,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
        if valid_matchlet != 0 {
            return TRUE;
        }
        i = i.wrapping_add(1);
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_cache_magic_matchlet_compare(
    mut cache: *mut XdgMimeCache,
    mut offset: xdg_uint32_t,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut n_children: xdg_uint32_t = safe_c2rust___bswap_32(
        *((*cache)
            .buffer
            .offset(offset.wrapping_add(24 as xdg_uint32_t) as isize)
            as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    let mut child_offset: xdg_uint32_t = safe_c2rust___bswap_32(
        *((*cache)
            .buffer
            .offset(offset.wrapping_add(28 as xdg_uint32_t) as isize)
            as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    if child_offset & 0x3 as xdg_uint32_t != 0
        || child_offset as size_t > (*cache).size
        || n_children as size_t
            > (*cache)
                .size
                .wrapping_sub(child_offset as size_t)
                .wrapping_div(32 as size_t)
    {
        return FALSE;
    }
    let mut i: xdg_uint32_t = 0;
    if safe_c2rust_cache_magic_matchlet_compare_to_data(cache, offset, data, len) != 0 {
        if n_children == 0 as xdg_uint32_t {
            return TRUE;
        }
        i = 0 as xdg_uint32_t;
        while i < n_children {
            if safe_c2rust_cache_magic_matchlet_compare(
                cache,
                child_offset.wrapping_add((32 as xdg_uint32_t).wrapping_mul(i)),
                data,
                len,
            ) != 0
            {
                return TRUE;
            }
            i = i.wrapping_add(1);
        }
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_cache_magic_compare_to_data(
    mut cache: *mut XdgMimeCache,
    mut offset: xdg_uint32_t,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
    mut prio: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut priority: xdg_uint32_t =
        safe_c2rust___bswap_32(*((*cache).buffer.offset(offset as isize) as *mut xdg_uint32_t))
            as xdg_uint32_t;
    let mut mimetype_offset: xdg_uint32_t = safe_c2rust___bswap_32(
        *((*cache)
            .buffer
            .offset(offset.wrapping_add(4 as xdg_uint32_t) as isize)
            as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    let mut n_matchlets: xdg_uint32_t = safe_c2rust___bswap_32(
        *((*cache)
            .buffer
            .offset(offset.wrapping_add(8 as xdg_uint32_t) as isize)
            as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    let mut matchlet_offset: xdg_uint32_t = safe_c2rust___bswap_32(
        *((*cache)
            .buffer
            .offset(offset.wrapping_add(12 as xdg_uint32_t) as isize)
            as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    if matchlet_offset & 0x3 as xdg_uint32_t != 0
        || matchlet_offset as size_t > (*cache).size
        || n_matchlets as size_t
            > (*cache)
                .size
                .wrapping_sub(matchlet_offset as size_t)
                .wrapping_div(32 as size_t)
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    let mut i: xdg_uint32_t = 0;
    i = 0 as xdg_uint32_t;
    while i < n_matchlets {
        if safe_c2rust_cache_magic_matchlet_compare(
            cache,
            matchlet_offset.wrapping_add(i.wrapping_mul(32 as xdg_uint32_t)),
            data,
            len,
        ) != 0
        {
            *prio = priority as ::core::ffi::c_int;
            return (*cache).buffer.offset(mimetype_offset as isize);
        }
        i = i.wrapping_add(1);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn safe_c2rust_cache_magic_lookup_data(
    mut cache: *mut XdgMimeCache,
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
    mut prio: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut list_offset: xdg_uint32_t = 0;
    let mut n_entries: xdg_uint32_t = 0;
    let mut offset: xdg_uint32_t = 0;
    let mut j: xdg_uint32_t = 0;
    *prio = 0 as ::core::ffi::c_int;
    list_offset = safe_c2rust___bswap_32(
        *((*cache).buffer.offset(24 as ::core::ffi::c_int as isize) as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    if list_offset & 0x3 as xdg_uint32_t != 0
        || list_offset as size_t > (*cache).size
        || 1 as size_t
            > (*cache)
                .size
                .wrapping_sub(list_offset as size_t)
                .wrapping_div(12 as size_t)
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    n_entries = safe_c2rust___bswap_32(
        *((*cache).buffer.offset(list_offset as isize) as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    offset = safe_c2rust___bswap_32(
        *((*cache)
            .buffer
            .offset(list_offset.wrapping_add(8 as xdg_uint32_t) as isize)
            as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    if offset & 0x3 as xdg_uint32_t != 0
        || offset as size_t > (*cache).size
        || n_entries as size_t
            > (*cache)
                .size
                .wrapping_sub(offset as size_t)
                .wrapping_div(16 as size_t)
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    j = 0 as xdg_uint32_t;
    while j < n_entries {
        let mut match_0: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        match_0 = safe_c2rust_cache_magic_compare_to_data(
            cache,
            offset.wrapping_add((16 as xdg_uint32_t).wrapping_mul(j)),
            data,
            len,
            prio,
        );
        if !match_0.is_null() {
            return match_0;
        }
        j = j.wrapping_add(1);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn safe_c2rust_cache_alias_lookup(
    mut alias: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut min: ::core::ffi::c_int = 0;
    let mut max: ::core::ffi::c_int = 0;
    let mut mid: ::core::ffi::c_int = 0;
    let mut cmp: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while !(*safe_c2rust__caches.offset(i as isize)).is_null() {
        let mut cache: *mut XdgMimeCache = *safe_c2rust__caches.offset(i as isize);
        let mut list_offset: xdg_uint32_t = 0;
        let mut n_entries: xdg_uint32_t = 0;
        let mut offset: xdg_uint32_t = 0;
        if !(*cache).buffer.is_null() {
            list_offset = safe_c2rust___bswap_32(
                *((*cache).buffer.offset(4 as ::core::ffi::c_int as isize) as *mut xdg_uint32_t),
            ) as xdg_uint32_t;
            if !(list_offset & 0x3 as xdg_uint32_t != 0
                || list_offset as size_t > (*cache).size
                || 1 as size_t
                    > (*cache)
                        .size
                        .wrapping_sub(list_offset as size_t)
                        .wrapping_div(4 as size_t))
            {
                n_entries = safe_c2rust___bswap_32(
                    *((*cache).buffer.offset(list_offset as isize) as *mut xdg_uint32_t),
                ) as xdg_uint32_t;
                if !(list_offset.wrapping_add(4 as xdg_uint32_t) & 0x3 as xdg_uint32_t != 0
                    || list_offset.wrapping_add(4 as xdg_uint32_t) as size_t > (*cache).size
                    || n_entries as size_t
                        > (*cache)
                            .size
                            .wrapping_sub(list_offset.wrapping_add(4 as xdg_uint32_t) as size_t)
                            .wrapping_div(8 as size_t))
                {
                    min = 0 as ::core::ffi::c_int;
                    max = n_entries.wrapping_sub(1 as xdg_uint32_t) as ::core::ffi::c_int;
                    while max >= min {
                        mid = (min + max) / 2 as ::core::ffi::c_int;
                        offset = safe_c2rust___bswap_32(
                            *((*cache).buffer.offset(
                                list_offset
                                    .wrapping_add(4 as xdg_uint32_t)
                                    .wrapping_add((8 as ::core::ffi::c_int * mid) as xdg_uint32_t)
                                    as isize,
                            ) as *mut xdg_uint32_t),
                        ) as xdg_uint32_t;
                        ptr = (*cache).buffer.offset(offset as isize);
                        cmp = strcmp(ptr, alias);
                        if cmp < 0 as ::core::ffi::c_int {
                            min = mid + 1 as ::core::ffi::c_int;
                        } else if cmp > 0 as ::core::ffi::c_int {
                            max = mid - 1 as ::core::ffi::c_int;
                        } else {
                            offset = safe_c2rust___bswap_32(
                                *((*cache).buffer.offset(
                                    list_offset
                                        .wrapping_add(4 as xdg_uint32_t)
                                        .wrapping_add(
                                            (8 as ::core::ffi::c_int * mid) as xdg_uint32_t,
                                        )
                                        .wrapping_add(4 as xdg_uint32_t)
                                        as isize,
                                ) as *mut xdg_uint32_t),
                            ) as xdg_uint32_t;
                            return (*cache).buffer.offset(offset as isize);
                        }
                    }
                }
            }
        }
        i += 1;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn safe_c2rust_cache_glob_lookup_literal(
    mut file_name: *const ::core::ffi::c_char,
    mut mime_types: *mut *const ::core::ffi::c_char,
    mut n_mime_types: ::core::ffi::c_int,
    mut case_sensitive_check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut min: ::core::ffi::c_int = 0;
    let mut max: ::core::ffi::c_int = 0;
    let mut mid: ::core::ffi::c_int = 0;
    let mut cmp: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if n_mime_types > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"n_mime_types > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"../original/gio/xdgmime/xdgmimecache.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                370 as ::core::ffi::c_uint,
                b"int cache_glob_lookup_literal(const char *, const char **, int, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    i = 0 as ::core::ffi::c_int;
    while !(*safe_c2rust__caches.offset(i as isize)).is_null() {
        let mut cache: *mut XdgMimeCache = *safe_c2rust__caches.offset(i as isize);
        let mut list_offset: xdg_uint32_t = 0;
        let mut n_entries: xdg_uint32_t = 0;
        let mut offset: xdg_uint32_t = 0;
        if !(*cache).buffer.is_null() {
            list_offset = safe_c2rust___bswap_32(
                *((*cache).buffer.offset(12 as ::core::ffi::c_int as isize) as *mut xdg_uint32_t),
            ) as xdg_uint32_t;
            if !(list_offset & 0x3 as xdg_uint32_t != 0
                || list_offset as size_t > (*cache).size
                || 1 as size_t
                    > (*cache)
                        .size
                        .wrapping_sub(list_offset as size_t)
                        .wrapping_div(4 as size_t))
            {
                n_entries = safe_c2rust___bswap_32(
                    *((*cache).buffer.offset(list_offset as isize) as *mut xdg_uint32_t),
                ) as xdg_uint32_t;
                if !(list_offset.wrapping_add(4 as xdg_uint32_t) & 0x3 as xdg_uint32_t != 0
                    || list_offset.wrapping_add(4 as xdg_uint32_t) as size_t > (*cache).size
                    || n_entries as size_t
                        > (*cache)
                            .size
                            .wrapping_sub(list_offset.wrapping_add(4 as xdg_uint32_t) as size_t)
                            .wrapping_div(12 as size_t))
                {
                    min = 0 as ::core::ffi::c_int;
                    max = n_entries.wrapping_sub(1 as xdg_uint32_t) as ::core::ffi::c_int;
                    while max >= min {
                        mid = (min + max) / 2 as ::core::ffi::c_int;
                        offset = safe_c2rust___bswap_32(
                            *((*cache).buffer.offset(
                                list_offset
                                    .wrapping_add(4 as xdg_uint32_t)
                                    .wrapping_add((12 as ::core::ffi::c_int * mid) as xdg_uint32_t)
                                    as isize,
                            ) as *mut xdg_uint32_t),
                        ) as xdg_uint32_t;
                        ptr = (*cache).buffer.offset(offset as isize);
                        cmp = strcmp(ptr, file_name);
                        if cmp < 0 as ::core::ffi::c_int {
                            min = mid + 1 as ::core::ffi::c_int;
                        } else if cmp > 0 as ::core::ffi::c_int {
                            max = mid - 1 as ::core::ffi::c_int;
                        } else {
                            let mut weight: ::core::ffi::c_int = safe_c2rust___bswap_32(
                                *((*cache).buffer.offset(
                                    list_offset
                                        .wrapping_add(4 as xdg_uint32_t)
                                        .wrapping_add(
                                            (12 as ::core::ffi::c_int * mid) as xdg_uint32_t,
                                        )
                                        .wrapping_add(8 as xdg_uint32_t)
                                        as isize,
                                ) as *mut xdg_uint32_t),
                            )
                                as ::core::ffi::c_int;
                            let mut case_sensitive: ::core::ffi::c_int =
                                weight & 0x100 as ::core::ffi::c_int;
                            weight = weight & 0xff as ::core::ffi::c_int;
                            if case_sensitive_check != 0 || case_sensitive == 0 {
                                offset = safe_c2rust___bswap_32(
                                    *((*cache).buffer.offset(
                                        list_offset
                                            .wrapping_add(4 as xdg_uint32_t)
                                            .wrapping_add(
                                                (12 as ::core::ffi::c_int * mid) as xdg_uint32_t,
                                            )
                                            .wrapping_add(4 as xdg_uint32_t)
                                            as isize,
                                    ) as *mut xdg_uint32_t),
                                ) as xdg_uint32_t;
                                let ref mut fresh6 =
                                    *mime_types.offset(0 as ::core::ffi::c_int as isize);
                                *fresh6 = (*cache).buffer.offset(offset as isize)
                                    as *const ::core::ffi::c_char;
                                return 1 as ::core::ffi::c_int;
                            }
                            return 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_cache_glob_lookup_fnmatch(
    mut file_name: *const ::core::ffi::c_char,
    mut mime_types: *mut MimeWeight,
    mut n_mime_types: ::core::ffi::c_int,
    mut case_sensitive_check: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut mime_type: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut j: xdg_uint32_t = 0;
    n = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while !(*safe_c2rust__caches.offset(i as isize)).is_null() {
        let mut cache: *mut XdgMimeCache = *safe_c2rust__caches.offset(i as isize);
        let mut list_offset: xdg_uint32_t = 0;
        let mut n_entries: xdg_uint32_t = 0;
        if !(*cache).buffer.is_null() {
            list_offset = safe_c2rust___bswap_32(
                *((*cache).buffer.offset(20 as ::core::ffi::c_int as isize) as *mut xdg_uint32_t),
            ) as xdg_uint32_t;
            if !(list_offset & 0x3 as xdg_uint32_t != 0
                || list_offset as size_t > (*cache).size
                || 1 as size_t
                    > (*cache)
                        .size
                        .wrapping_sub(list_offset as size_t)
                        .wrapping_div(4 as size_t))
            {
                n_entries = safe_c2rust___bswap_32(
                    *((*cache).buffer.offset(list_offset as isize) as *mut xdg_uint32_t),
                ) as xdg_uint32_t;
                if !(list_offset.wrapping_add(4 as xdg_uint32_t) & 0x3 as xdg_uint32_t != 0
                    || list_offset.wrapping_add(4 as xdg_uint32_t) as size_t > (*cache).size
                    || n_entries as size_t
                        > (*cache)
                            .size
                            .wrapping_sub(list_offset.wrapping_add(4 as xdg_uint32_t) as size_t)
                            .wrapping_div(12 as size_t))
                {
                    j = 0 as xdg_uint32_t;
                    while j < n_entries && n < n_mime_types {
                        let mut offset: xdg_uint32_t = safe_c2rust___bswap_32(
                            *((*cache).buffer.offset(
                                list_offset
                                    .wrapping_add(4 as xdg_uint32_t)
                                    .wrapping_add((12 as xdg_uint32_t).wrapping_mul(j))
                                    as isize,
                            ) as *mut xdg_uint32_t),
                        ) as xdg_uint32_t;
                        let mut mimetype_offset: xdg_uint32_t = safe_c2rust___bswap_32(
                            *((*cache).buffer.offset(
                                list_offset
                                    .wrapping_add(4 as xdg_uint32_t)
                                    .wrapping_add((12 as xdg_uint32_t).wrapping_mul(j))
                                    .wrapping_add(4 as xdg_uint32_t)
                                    as isize,
                            ) as *mut xdg_uint32_t),
                        )
                            as xdg_uint32_t;
                        let mut weight: ::core::ffi::c_int = safe_c2rust___bswap_32(
                            *((*cache).buffer.offset(
                                list_offset
                                    .wrapping_add(4 as xdg_uint32_t)
                                    .wrapping_add((12 as xdg_uint32_t).wrapping_mul(j))
                                    .wrapping_add(8 as xdg_uint32_t)
                                    as isize,
                            ) as *mut xdg_uint32_t),
                        )
                            as ::core::ffi::c_int;
                        let mut case_sensitive: ::core::ffi::c_int =
                            weight & 0x100 as ::core::ffi::c_int;
                        weight = weight & 0xff as ::core::ffi::c_int;
                        ptr = (*cache).buffer.offset(offset as isize);
                        mime_type = (*cache).buffer.offset(mimetype_offset as isize);
                        if case_sensitive_check != 0 || case_sensitive == 0 {
                            if fnmatch(ptr, file_name, 0 as ::core::ffi::c_int)
                                == 0 as ::core::ffi::c_int
                            {
                                let ref mut fresh4 = (*mime_types.offset(n as isize)).mime;
                                *fresh4 = mime_type;
                                (*mime_types.offset(n as isize)).weight = weight;
                                n += 1;
                            }
                        }
                        j = j.wrapping_add(1);
                    }
                    if n == n_mime_types {
                        break;
                    }
                }
            }
        }
        i += 1;
    }
    return n;
}
unsafe extern "C" fn safe_c2rust_cache_glob_node_lookup_suffix(
    mut cache: *mut XdgMimeCache,
    mut n_entries: xdg_uint32_t,
    mut offset: xdg_uint32_t,
    mut file_name: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut case_sensitive_check: ::core::ffi::c_int,
    mut mime_types: *mut MimeWeight,
    mut n_mime_types: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut character: xdg_unichar_t = 0;
    let mut match_char: xdg_unichar_t = 0;
    let mut mimetype_offset: xdg_uint32_t = 0;
    let mut n_children: xdg_uint32_t = 0;
    let mut child_offset: xdg_uint32_t = 0;
    let mut weight: ::core::ffi::c_int = 0;
    let mut case_sensitive: ::core::ffi::c_int = 0;
    let mut i: xdg_uint32_t = 0;
    let mut min: ::core::ffi::c_int = 0;
    let mut max: ::core::ffi::c_int = 0;
    let mut mid: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    character = *file_name.offset((len - 1 as ::core::ffi::c_int) as isize) as xdg_unichar_t;
    '_c2rust_label: {
        if character != 0 as xdg_unichar_t {
        } else {
            __assert_fail(
                b"character != 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"../original/gio/xdgmime/xdgmimecache.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                507 as ::core::ffi::c_uint,
                b"int cache_glob_node_lookup_suffix(XdgMimeCache *, xdg_uint32_t, xdg_uint32_t, const char *, int, int, MimeWeight *, int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    min = 0 as ::core::ffi::c_int;
    max = n_entries.wrapping_sub(1 as xdg_uint32_t) as ::core::ffi::c_int;
    while max >= min {
        mid = (min + max) / 2 as ::core::ffi::c_int;
        match_char = safe_c2rust___bswap_32(
            *((*cache).buffer.offset(
                offset.wrapping_add((12 as ::core::ffi::c_int * mid) as xdg_uint32_t) as isize,
            ) as *mut xdg_uint32_t),
        ) as xdg_unichar_t;
        if match_char < character {
            min = mid + 1 as ::core::ffi::c_int;
        } else if match_char > character {
            max = mid - 1 as ::core::ffi::c_int;
        } else {
            len -= 1;
            n = 0 as ::core::ffi::c_int;
            n_children = safe_c2rust___bswap_32(
                *((*cache).buffer.offset(
                    offset
                        .wrapping_add((12 as ::core::ffi::c_int * mid) as xdg_uint32_t)
                        .wrapping_add(4 as xdg_uint32_t) as isize,
                ) as *mut xdg_uint32_t),
            ) as xdg_uint32_t;
            child_offset = safe_c2rust___bswap_32(
                *((*cache).buffer.offset(
                    offset
                        .wrapping_add((12 as ::core::ffi::c_int * mid) as xdg_uint32_t)
                        .wrapping_add(8 as xdg_uint32_t) as isize,
                ) as *mut xdg_uint32_t),
            ) as xdg_uint32_t;
            if child_offset & 0x3 as xdg_uint32_t != 0
                || child_offset as size_t > (*cache).size
                || n_children as size_t
                    > (*cache)
                        .size
                        .wrapping_sub(child_offset as size_t)
                        .wrapping_div(12 as size_t)
            {
                continue;
            }
            if len > 0 as ::core::ffi::c_int {
                n = safe_c2rust_cache_glob_node_lookup_suffix(
                    cache,
                    n_children,
                    child_offset,
                    file_name,
                    len,
                    case_sensitive_check,
                    mime_types,
                    n_mime_types,
                );
            }
            if n == 0 as ::core::ffi::c_int {
                i = 0 as xdg_uint32_t;
                while n < n_mime_types && i < n_children {
                    match_char = safe_c2rust___bswap_32(
                        *((*cache).buffer.offset(
                            child_offset.wrapping_add((12 as xdg_uint32_t).wrapping_mul(i))
                                as isize,
                        ) as *mut xdg_uint32_t),
                    ) as xdg_unichar_t;
                    if match_char != 0 as xdg_unichar_t {
                        break;
                    }
                    mimetype_offset = safe_c2rust___bswap_32(
                        *((*cache).buffer.offset(
                            child_offset
                                .wrapping_add((12 as xdg_uint32_t).wrapping_mul(i))
                                .wrapping_add(4 as xdg_uint32_t)
                                as isize,
                        ) as *mut xdg_uint32_t),
                    ) as xdg_uint32_t;
                    weight = safe_c2rust___bswap_32(
                        *((*cache).buffer.offset(
                            child_offset
                                .wrapping_add((12 as xdg_uint32_t).wrapping_mul(i))
                                .wrapping_add(8 as xdg_uint32_t)
                                as isize,
                        ) as *mut xdg_uint32_t),
                    ) as ::core::ffi::c_int;
                    case_sensitive = weight & 0x100 as ::core::ffi::c_int;
                    weight = weight & 0xff as ::core::ffi::c_int;
                    if case_sensitive_check != 0 || case_sensitive == 0 {
                        let ref mut fresh5 = (*mime_types.offset(n as isize)).mime;
                        *fresh5 = (*cache).buffer.offset(mimetype_offset as isize);
                        (*mime_types.offset(n as isize)).weight = weight;
                        n += 1;
                    }
                    i = i.wrapping_add(1);
                }
            }
            return n;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_cache_glob_lookup_suffix(
    mut file_name: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut ignore_case: ::core::ffi::c_int,
    mut mime_types: *mut MimeWeight,
    mut n_mime_types: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    n = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while !(*safe_c2rust__caches.offset(i as isize)).is_null() {
        let mut cache: *mut XdgMimeCache = *safe_c2rust__caches.offset(i as isize);
        let mut list_offset: xdg_uint32_t = 0;
        let mut n_entries: xdg_uint32_t = 0;
        let mut offset: xdg_uint32_t = 0;
        if !(*cache).buffer.is_null() {
            list_offset = safe_c2rust___bswap_32(
                *((*cache).buffer.offset(16 as ::core::ffi::c_int as isize) as *mut xdg_uint32_t),
            ) as xdg_uint32_t;
            if !(list_offset & 0x3 as xdg_uint32_t != 0
                || list_offset as size_t > (*cache).size
                || 1 as size_t
                    > (*cache)
                        .size
                        .wrapping_sub(list_offset as size_t)
                        .wrapping_div(8 as size_t))
            {
                n_entries = safe_c2rust___bswap_32(
                    *((*cache).buffer.offset(list_offset as isize) as *mut xdg_uint32_t),
                ) as xdg_uint32_t;
                offset = safe_c2rust___bswap_32(
                    *((*cache)
                        .buffer
                        .offset(list_offset.wrapping_add(4 as xdg_uint32_t) as isize)
                        as *mut xdg_uint32_t),
                ) as xdg_uint32_t;
                if !(offset & 0x3 as xdg_uint32_t != 0
                    || offset as size_t > (*cache).size
                    || n_entries as size_t
                        > (*cache)
                            .size
                            .wrapping_sub(offset as size_t)
                            .wrapping_div(12 as size_t))
                {
                    n += safe_c2rust_cache_glob_node_lookup_suffix(
                        cache,
                        n_entries,
                        offset,
                        file_name,
                        len,
                        ignore_case,
                        mime_types.offset(n as isize),
                        n_mime_types - n,
                    );
                    if n == n_mime_types {
                        break;
                    }
                }
            }
        }
        i += 1;
    }
    return n;
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
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = (if c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'Z' as i32
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
unsafe extern "C" fn safe_c2rust_cache_glob_lookup_file_name(
    mut file_name: *const ::core::ffi::c_char,
    mut mime_types: *mut *const ::core::ffi::c_char,
    mut n_mime_types: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    let mut mimes: [MimeWeight; 10] = [MimeWeight {
        mime: ::core::ptr::null::<::core::ffi::c_char>(),
        weight: 0,
    }; 10];
    let mut n_mimes: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut lower_case: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    '_c2rust_label: {
        if !file_name.is_null() && n_mime_types > 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"file_name != NULL && n_mime_types > 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"../original/gio/xdgmime/xdgmimecache.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                673 as ::core::ffi::c_uint,
                b"int cache_glob_lookup_file_name(const char *, const char **, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    lower_case = safe_c2rust_ascii_tolower(file_name);
    n = safe_c2rust_cache_glob_lookup_literal(lower_case, mime_types, n_mime_types, FALSE);
    if n > 0 as ::core::ffi::c_int {
        free(lower_case as *mut ::core::ffi::c_void);
        return n;
    }
    n = safe_c2rust_cache_glob_lookup_literal(file_name, mime_types, n_mime_types, TRUE);
    if n > 0 as ::core::ffi::c_int {
        free(lower_case as *mut ::core::ffi::c_void);
        return n;
    }
    len = strlen(file_name) as ::core::ffi::c_int;
    n = safe_c2rust_cache_glob_lookup_suffix(
        lower_case,
        len,
        FALSE,
        &raw mut mimes as *mut MimeWeight,
        n_mimes,
    );
    if n < 2 as ::core::ffi::c_int {
        n += safe_c2rust_cache_glob_lookup_suffix(
            file_name,
            len,
            TRUE,
            (&raw mut mimes as *mut MimeWeight).offset(n as isize),
            n_mimes - n,
        );
    }
    if n == 0 as ::core::ffi::c_int {
        n = safe_c2rust_cache_glob_lookup_fnmatch(
            lower_case,
            &raw mut mimes as *mut MimeWeight,
            n_mimes,
            FALSE,
        );
    }
    if n < 2 as ::core::ffi::c_int {
        n += safe_c2rust_cache_glob_lookup_fnmatch(
            file_name,
            (&raw mut mimes as *mut MimeWeight).offset(n as isize),
            n_mimes - n,
            TRUE,
        );
    }
    n = safe_c2rust_filter_out_dupes(&raw mut mimes as *mut MimeWeight, n);
    free(lower_case as *mut ::core::ffi::c_void);
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
pub unsafe extern "C" fn safe_c2rust___gio_xdg_cache_get_max_buffer_extents() -> ::core::ffi::c_int
{
    let mut offset: xdg_uint32_t = 0;
    let mut max_extent: xdg_uint32_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    max_extent = 0 as xdg_uint32_t;
    i = 0 as ::core::ffi::c_int;
    while !(*safe_c2rust__caches.offset(i as isize)).is_null() {
        let mut cache: *mut XdgMimeCache = *safe_c2rust__caches.offset(i as isize);
        if !(*cache).buffer.is_null() {
            offset = safe_c2rust___bswap_32(
                *((*cache).buffer.offset(24 as ::core::ffi::c_int as isize) as *mut xdg_uint32_t),
            ) as xdg_uint32_t;
            if !(offset & 0x3 as xdg_uint32_t != 0
                || offset as size_t > (*cache).size
                || 1 as size_t
                    > (*cache)
                        .size
                        .wrapping_sub(offset as size_t)
                        .wrapping_div(8 as size_t))
            {
                max_extent = (if max_extent as __uint32_t
                    > safe_c2rust___bswap_32(
                        *((*cache)
                            .buffer
                            .offset(offset.wrapping_add(4 as xdg_uint32_t) as isize)
                            as *mut xdg_uint32_t),
                    ) {
                    max_extent as __uint32_t
                } else {
                    safe_c2rust___bswap_32(
                        *((*cache)
                            .buffer
                            .offset(offset.wrapping_add(4 as xdg_uint32_t) as isize)
                            as *mut xdg_uint32_t),
                    )
                }) as xdg_uint32_t;
            }
        }
        i += 1;
    }
    return max_extent as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_cache_get_mime_type_for_data(
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
    mut result_prio: *mut ::core::ffi::c_int,
    mut mime_types: *mut *const ::core::ffi::c_char,
    mut n_mime_types: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut mime_type: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut priority: ::core::ffi::c_int = 0;
    priority = 0 as ::core::ffi::c_int;
    mime_type = ::core::ptr::null::<::core::ffi::c_char>();
    i = 0 as ::core::ffi::c_int;
    while !(*safe_c2rust__caches.offset(i as isize)).is_null() {
        let mut cache: *mut XdgMimeCache = *safe_c2rust__caches.offset(i as isize);
        let mut prio: ::core::ffi::c_int = 0;
        let mut match_0: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if !(*cache).buffer.is_null() {
            match_0 = safe_c2rust_cache_magic_lookup_data(cache, data, len, &raw mut prio);
            if prio > priority {
                priority = prio;
                mime_type = match_0;
            }
        }
        i += 1;
    }
    if !result_prio.is_null() {
        *result_prio = priority;
    }
    if priority > 0 as ::core::ffi::c_int {
        n = 0 as ::core::ffi::c_int;
        while n < n_mime_types {
            if !(*mime_types.offset(n as isize)).is_null()
                && safe_c2rust___gio_xdg_cache_mime_type_subclass(
                    *mime_types.offset(n as isize),
                    mime_type,
                    ::core::ptr::null_mut::<*mut *const ::core::ffi::c_char>(),
                ) != 0
            {
                return *mime_types.offset(n as isize);
            }
            n += 1;
        }
        if n == 0 as ::core::ffi::c_int {
            return mime_type;
        }
    }
    n = 0 as ::core::ffi::c_int;
    while n < n_mime_types {
        if !(*mime_types.offset(n as isize)).is_null() {
            return *mime_types.offset(n as isize);
        }
        n += 1;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_cache_get_mime_type_for_data(
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
    mut result_prio: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    return safe_c2rust_cache_get_mime_type_for_data(
        data,
        len,
        result_prio,
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_cache_get_mime_type_for_file(
    mut file_name: *const ::core::ffi::c_char,
    mut statbuf: *mut stat,
) -> *const ::core::ffi::c_char {
    let mut mime_type: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut mime_types: [*const ::core::ffi::c_char; 10] =
        [::core::ptr::null::<::core::ffi::c_char>(); 10];
    let mut file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut data: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut max_extent: ::core::ffi::c_int = 0;
    let mut bytes_read: ::core::ffi::c_int = 0;
    let mut buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut base_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0;
    if file_name.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    if __gio_xdg_utf8_validate(file_name) == 0 {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    base_name = __gio_xdg_get_base_name(file_name);
    n = safe_c2rust_cache_glob_lookup_file_name(
        base_name,
        &raw mut mime_types as *mut *const ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    );
    if n == 1 as ::core::ffi::c_int {
        return mime_types[0 as ::core::ffi::c_int as usize];
    }
    if statbuf.is_null() {
        if stat(file_name, &raw mut buf) != 0 as ::core::ffi::c_int {
            return &raw const safe_c2rust__gio_xdg_type_unknown as *const ::core::ffi::c_char;
        }
        statbuf = &raw mut buf;
    }
    if (*statbuf).st_size == 0 as __off_t {
        return &raw const safe_c2rust__gio_xdg_type_empty as *const ::core::ffi::c_char;
    }
    if !((*statbuf).st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) {
        return &raw const safe_c2rust__gio_xdg_type_unknown as *const ::core::ffi::c_char;
    }
    max_extent = safe_c2rust___gio_xdg_cache_get_max_buffer_extents();
    data = malloc(max_extent as size_t) as *mut ::core::ffi::c_uchar;
    if data.is_null() {
        return &raw const safe_c2rust__gio_xdg_type_unknown as *const ::core::ffi::c_char;
    }
    file = fopen(file_name, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if file.is_null() {
        free(data as *mut ::core::ffi::c_void);
        return &raw const safe_c2rust__gio_xdg_type_unknown as *const ::core::ffi::c_char;
    }
    bytes_read = fread(
        data as *mut ::core::ffi::c_void,
        1 as size_t,
        max_extent as size_t,
        file,
    ) as ::core::ffi::c_int;
    if ferror(file) != 0 {
        free(data as *mut ::core::ffi::c_void);
        fclose(file);
        return &raw const safe_c2rust__gio_xdg_type_unknown as *const ::core::ffi::c_char;
    }
    mime_type = safe_c2rust_cache_get_mime_type_for_data(
        data as *const ::core::ffi::c_void,
        bytes_read as size_t,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut mime_types as *mut *const ::core::ffi::c_char,
        n,
    );
    if mime_type.is_null() {
        mime_type =
            _xdg_binary_or_text_fallback(data as *const ::core::ffi::c_void, bytes_read as size_t);
    }
    free(data as *mut ::core::ffi::c_void);
    fclose(file);
    return mime_type;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_cache_get_mime_type_from_file_name(
    mut file_name: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut mime_type: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if safe_c2rust_cache_glob_lookup_file_name(
        file_name,
        &raw mut mime_type,
        1 as ::core::ffi::c_int,
    ) != 0
    {
        return mime_type;
    } else {
        return &raw const safe_c2rust__gio_xdg_type_unknown as *const ::core::ffi::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_cache_get_mime_types_from_file_name(
    mut file_name: *const ::core::ffi::c_char,
    mut mime_types: *mut *const ::core::ffi::c_char,
    mut n_mime_types: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return safe_c2rust_cache_glob_lookup_file_name(file_name, mime_types, n_mime_types);
}
unsafe extern "C" fn safe_c2rust_ends_with(
    mut str: *const ::core::ffi::c_char,
    mut suffix: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut length: ::core::ffi::c_int = 0;
    let mut suffix_length: ::core::ffi::c_int = 0;
    length = strlen(str) as ::core::ffi::c_int;
    suffix_length = strlen(suffix) as ::core::ffi::c_int;
    if length < suffix_length {
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(
        str.offset(length as isize)
            .offset(-(suffix_length as isize)),
        suffix,
    ) == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_is_super_type(
    mut mime: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return safe_c2rust_ends_with(mime, b"/*\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_cache_mime_type_subclass(
    mut mime: *const ::core::ffi::c_char,
    mut base: *const ::core::ffi::c_char,
    mut seen: *mut *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut umime: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ubase: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut parent: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut first_seen: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut new_seen: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut j: xdg_uint32_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut min: ::core::ffi::c_int = 0;
    let mut max: ::core::ffi::c_int = 0;
    let mut med: ::core::ffi::c_int = 0;
    let mut cmp: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    umime = safe_c2rust___gio_xdg_cache_unalias_mime_type(mime);
    ubase = safe_c2rust___gio_xdg_cache_unalias_mime_type(base);
    if strcmp(umime, ubase) == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if safe_c2rust_is_super_type(ubase) != 0 && _gio_xdg_media_type_equal(umime, ubase) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        ubase,
        b"text/plain\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && strncmp(
            umime,
            b"text/\0" as *const u8 as *const ::core::ffi::c_char,
            5 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(
        ubase,
        b"application/octet-stream\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && strncmp(
            umime,
            b"inode/\0" as *const u8 as *const ::core::ffi::c_char,
            6 as size_t,
        ) != 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    if seen.is_null() {
        first_seen = calloc(
            1 as size_t,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t,
        ) as *mut *const ::core::ffi::c_char;
        seen = &raw mut first_seen;
    }
    i = 0 as ::core::ffi::c_int;
    's_59: while !(*safe_c2rust__caches.offset(i as isize)).is_null() {
        let mut cache: *mut XdgMimeCache = *safe_c2rust__caches.offset(i as isize);
        let mut list_offset: xdg_uint32_t = 0;
        let mut n_entries: xdg_uint32_t = 0;
        let mut offset: xdg_uint32_t = 0;
        let mut n_parents: xdg_uint32_t = 0;
        let mut parent_offset: xdg_uint32_t = 0;
        if !(*cache).buffer.is_null() {
            list_offset = safe_c2rust___bswap_32(
                *((*cache).buffer.offset(8 as ::core::ffi::c_int as isize) as *mut xdg_uint32_t),
            ) as xdg_uint32_t;
            if !(list_offset & 0x3 as xdg_uint32_t != 0
                || list_offset as size_t > (*cache).size
                || 1 as size_t
                    > (*cache)
                        .size
                        .wrapping_sub(list_offset as size_t)
                        .wrapping_div(4 as size_t))
            {
                n_entries = safe_c2rust___bswap_32(
                    *((*cache).buffer.offset(list_offset as isize) as *mut xdg_uint32_t),
                ) as xdg_uint32_t;
                if !(list_offset.wrapping_add(4 as xdg_uint32_t) & 0x3 as xdg_uint32_t != 0
                    || list_offset.wrapping_add(4 as xdg_uint32_t) as size_t > (*cache).size
                    || n_entries as size_t
                        > (*cache)
                            .size
                            .wrapping_sub(list_offset.wrapping_add(4 as xdg_uint32_t) as size_t)
                            .wrapping_div(8 as size_t))
                {
                    min = 0 as ::core::ffi::c_int;
                    max = n_entries.wrapping_sub(1 as xdg_uint32_t) as ::core::ffi::c_int;
                    's_102: while max >= min {
                        med = (min + max) / 2 as ::core::ffi::c_int;
                        offset = safe_c2rust___bswap_32(
                            *((*cache).buffer.offset(
                                list_offset
                                    .wrapping_add(4 as xdg_uint32_t)
                                    .wrapping_add((8 as ::core::ffi::c_int * med) as xdg_uint32_t)
                                    as isize,
                            ) as *mut xdg_uint32_t),
                        ) as xdg_uint32_t;
                        cmp = strcmp((*cache).buffer.offset(offset as isize), umime);
                        if cmp < 0 as ::core::ffi::c_int {
                            min = med + 1 as ::core::ffi::c_int;
                        } else if cmp > 0 as ::core::ffi::c_int {
                            max = med - 1 as ::core::ffi::c_int;
                        } else {
                            offset = safe_c2rust___bswap_32(
                                *((*cache).buffer.offset(
                                    list_offset
                                        .wrapping_add(4 as xdg_uint32_t)
                                        .wrapping_add(
                                            (8 as ::core::ffi::c_int * med) as xdg_uint32_t,
                                        )
                                        .wrapping_add(4 as xdg_uint32_t)
                                        as isize,
                                ) as *mut xdg_uint32_t),
                            ) as xdg_uint32_t;
                            n_parents = safe_c2rust___bswap_32(
                                *((*cache).buffer.offset(offset as isize) as *mut xdg_uint32_t),
                            ) as xdg_uint32_t;
                            j = 0 as xdg_uint32_t;
                            loop {
                                if !(j < n_parents) {
                                    break 's_102;
                                }
                                parent_offset = safe_c2rust___bswap_32(
                                    *((*cache).buffer.offset(
                                        offset
                                            .wrapping_add(4 as xdg_uint32_t)
                                            .wrapping_add((4 as xdg_uint32_t).wrapping_mul(j))
                                            as isize,
                                    ) as *mut xdg_uint32_t),
                                ) as xdg_uint32_t;
                                parent = (*cache).buffer.offset(parent_offset as isize);
                                k = 0 as ::core::ffi::c_int;
                                loop {
                                    if (*(*seen).offset(k as isize)).is_null() {
                                        current_block = 3275366147856559585;
                                        break;
                                    }
                                    if parent == *(*seen).offset(k as isize) {
                                        current_block = 14818589718467733107;
                                        break;
                                    }
                                    k += 1;
                                }
                                match current_block {
                                    3275366147856559585 => {
                                        new_seen = realloc(
                                            *seen as *mut ::core::ffi::c_void,
                                            ((k + 2 as ::core::ffi::c_int) as size_t).wrapping_mul(
                                                ::core::mem::size_of::<*mut ::core::ffi::c_char>()
                                                    as size_t,
                                            ),
                                        )
                                            as *mut *const ::core::ffi::c_char;
                                        if new_seen.is_null() {
                                            break 's_59;
                                        }
                                        let ref mut fresh0 = *new_seen.offset(k as isize);
                                        *fresh0 = parent;
                                        let ref mut fresh1 = *new_seen
                                            .offset((k + 1 as ::core::ffi::c_int) as isize);
                                        *fresh1 = ::core::ptr::null::<::core::ffi::c_char>();
                                        *seen = new_seen;
                                        if safe_c2rust___gio_xdg_cache_mime_type_subclass(
                                            parent, ubase, seen,
                                        ) != 0
                                        {
                                            ret = 1 as ::core::ffi::c_int;
                                            break 's_59;
                                        }
                                    }
                                    _ => {}
                                }
                                j = j.wrapping_add(1);
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    free(first_seen as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_cache_unalias_mime_type(
    mut mime: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut lookup: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    lookup = safe_c2rust_cache_alias_lookup(mime);
    if !lookup.is_null() {
        return lookup;
    }
    return mime;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_cache_list_mime_parents(
    mut mime: *const ::core::ffi::c_char,
) -> *mut *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut j: xdg_uint32_t = 0;
    let mut k: xdg_uint32_t = 0;
    let mut all_parents: [*mut ::core::ffi::c_char; 128] =
        [::core::ptr::null_mut::<::core::ffi::c_char>(); 128];
    let mut result: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    mime = _gio_xdg_unalias_mime_type(mime);
    p = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while !(*safe_c2rust__caches.offset(i as isize)).is_null() {
        let mut cache: *mut XdgMimeCache = *safe_c2rust__caches.offset(i as isize);
        let mut list_offset: xdg_uint32_t = 0;
        let mut n_entries: xdg_uint32_t = 0;
        if !(*cache).buffer.is_null() {
            list_offset = safe_c2rust___bswap_32(
                *((*cache).buffer.offset(8 as ::core::ffi::c_int as isize) as *mut xdg_uint32_t),
            ) as xdg_uint32_t;
            if !(list_offset & 0x3 as xdg_uint32_t != 0
                || list_offset as size_t > (*cache).size
                || 1 as size_t
                    > (*cache)
                        .size
                        .wrapping_sub(list_offset as size_t)
                        .wrapping_div(4 as size_t))
            {
                n_entries = safe_c2rust___bswap_32(
                    *((*cache).buffer.offset(list_offset as isize) as *mut xdg_uint32_t),
                ) as xdg_uint32_t;
                if !(list_offset.wrapping_add(4 as xdg_uint32_t) & 0x3 as xdg_uint32_t != 0
                    || list_offset.wrapping_add(4 as xdg_uint32_t) as size_t > (*cache).size
                    || n_entries as size_t
                        > (*cache)
                            .size
                            .wrapping_sub(list_offset.wrapping_add(4 as xdg_uint32_t) as size_t)
                            .wrapping_div(8 as size_t))
                {
                    j = 0 as xdg_uint32_t;
                    while j < n_entries {
                        let mut mimetype_offset: xdg_uint32_t = safe_c2rust___bswap_32(
                            *((*cache).buffer.offset(
                                list_offset
                                    .wrapping_add(4 as xdg_uint32_t)
                                    .wrapping_add((8 as xdg_uint32_t).wrapping_mul(j))
                                    as isize,
                            ) as *mut xdg_uint32_t),
                        )
                            as xdg_uint32_t;
                        let mut parents_offset: xdg_uint32_t = safe_c2rust___bswap_32(
                            *((*cache).buffer.offset(
                                list_offset
                                    .wrapping_add(4 as xdg_uint32_t)
                                    .wrapping_add((8 as xdg_uint32_t).wrapping_mul(j))
                                    .wrapping_add(4 as xdg_uint32_t)
                                    as isize,
                            ) as *mut xdg_uint32_t),
                        )
                            as xdg_uint32_t;
                        if strcmp((*cache).buffer.offset(mimetype_offset as isize), mime)
                            == 0 as ::core::ffi::c_int
                        {
                            let mut parent_mime_offset: xdg_uint32_t = 0;
                            let mut n_parents: xdg_uint32_t = safe_c2rust___bswap_32(
                                *((*cache).buffer.offset(parents_offset as isize)
                                    as *mut xdg_uint32_t),
                            )
                                as xdg_uint32_t;
                            k = 0 as xdg_uint32_t;
                            while k < n_parents && p < 127 as ::core::ffi::c_int {
                                parent_mime_offset = safe_c2rust___bswap_32(
                                    *((*cache).buffer.offset(
                                        parents_offset
                                            .wrapping_add(4 as xdg_uint32_t)
                                            .wrapping_add((4 as xdg_uint32_t).wrapping_mul(k))
                                            as isize,
                                    ) as *mut xdg_uint32_t),
                                )
                                    as xdg_uint32_t;
                                l = 0 as ::core::ffi::c_int;
                                while l < p {
                                    if strcmp(
                                        all_parents[l as usize],
                                        (*cache).buffer.offset(parent_mime_offset as isize),
                                    ) == 0 as ::core::ffi::c_int
                                    {
                                        break;
                                    }
                                    l += 1;
                                }
                                if l == p {
                                    let fresh8 = p;
                                    p = p + 1;
                                    all_parents[fresh8 as usize] =
                                        (*cache).buffer.offset(parent_mime_offset as isize);
                                }
                                k = k.wrapping_add(1);
                            }
                            break;
                        } else {
                            j = j.wrapping_add(1);
                        }
                    }
                }
            }
        }
        i += 1;
    }
    let fresh9 = p;
    p = p + 1;
    all_parents[fresh9 as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
    result = malloc(
        (p as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
    ) as *mut *mut ::core::ffi::c_char;
    memcpy(
        result as *mut ::core::ffi::c_void,
        &raw mut all_parents as *mut *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        (p as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
    );
    return result;
}
unsafe extern "C" fn safe_c2rust_cache_lookup_icon(
    mut mime: *const ::core::ffi::c_char,
    mut header: size_t,
) -> *const ::core::ffi::c_char {
    let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut min: ::core::ffi::c_int = 0;
    let mut max: ::core::ffi::c_int = 0;
    let mut mid: ::core::ffi::c_int = 0;
    let mut cmp: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while !(*safe_c2rust__caches.offset(i as isize)).is_null() {
        let mut cache: *mut XdgMimeCache = *safe_c2rust__caches.offset(i as isize);
        let mut list_offset: xdg_uint32_t = 0;
        let mut n_entries: xdg_uint32_t = 0;
        let mut offset: xdg_uint32_t = 0;
        if !(*cache).buffer.is_null() {
            if !(header & 0x3 as size_t != 0
                || header > (*cache).size
                || 1 as size_t > (*cache).size.wrapping_sub(header).wrapping_div(4 as size_t))
            {
                list_offset = safe_c2rust___bswap_32(
                    *((*cache).buffer.offset(header as isize) as *mut xdg_uint32_t),
                ) as xdg_uint32_t;
                if !(list_offset & 0x3 as xdg_uint32_t != 0
                    || list_offset as size_t > (*cache).size
                    || 1 as size_t
                        > (*cache)
                            .size
                            .wrapping_sub(list_offset as size_t)
                            .wrapping_div(4 as size_t))
                {
                    n_entries = safe_c2rust___bswap_32(
                        *((*cache).buffer.offset(list_offset as isize) as *mut xdg_uint32_t),
                    ) as xdg_uint32_t;
                    if !(list_offset.wrapping_add(4 as xdg_uint32_t) & 0x3 as xdg_uint32_t != 0
                        || list_offset.wrapping_add(4 as xdg_uint32_t) as size_t > (*cache).size
                        || n_entries as size_t
                            > (*cache)
                                .size
                                .wrapping_sub(list_offset.wrapping_add(4 as xdg_uint32_t) as size_t)
                                .wrapping_div(8 as size_t))
                    {
                        min = 0 as ::core::ffi::c_int;
                        max = n_entries.wrapping_sub(1 as xdg_uint32_t) as ::core::ffi::c_int;
                        while max >= min {
                            mid = (min + max) / 2 as ::core::ffi::c_int;
                            offset = safe_c2rust___bswap_32(
                                *((*cache).buffer.offset(
                                    list_offset.wrapping_add(4 as xdg_uint32_t).wrapping_add(
                                        (8 as ::core::ffi::c_int * mid) as xdg_uint32_t,
                                    ) as isize,
                                ) as *mut xdg_uint32_t),
                            ) as xdg_uint32_t;
                            ptr = (*cache).buffer.offset(offset as isize);
                            cmp = strcmp(ptr, mime);
                            if cmp < 0 as ::core::ffi::c_int {
                                min = mid + 1 as ::core::ffi::c_int;
                            } else if cmp > 0 as ::core::ffi::c_int {
                                max = mid - 1 as ::core::ffi::c_int;
                            } else {
                                offset = safe_c2rust___bswap_32(
                                    *((*cache).buffer.offset(
                                        list_offset
                                            .wrapping_add(4 as xdg_uint32_t)
                                            .wrapping_add(
                                                (8 as ::core::ffi::c_int * mid) as xdg_uint32_t,
                                            )
                                            .wrapping_add(4 as xdg_uint32_t)
                                            as isize,
                                    ) as *mut xdg_uint32_t),
                                ) as xdg_uint32_t;
                                return (*cache).buffer.offset(offset as isize);
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_cache_get_generic_icon(
    mut mime: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    return safe_c2rust_cache_lookup_icon(mime, 36 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_cache_get_icon(
    mut mime: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    return safe_c2rust_cache_lookup_icon(mime, 32 as size_t);
}
unsafe extern "C" fn safe_c2rust_dump_glob_node(
    mut cache: *mut XdgMimeCache,
    mut offset: xdg_uint32_t,
    mut depth: ::core::ffi::c_int,
) {
    let mut character: xdg_unichar_t = 0;
    let mut mime_offset: xdg_uint32_t = 0;
    let mut n_children: xdg_uint32_t = 0;
    let mut child_offset: xdg_uint32_t = 0;
    let mut k: xdg_uint32_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    character =
        safe_c2rust___bswap_32(*((*cache).buffer.offset(offset as isize) as *mut xdg_uint32_t))
            as xdg_unichar_t;
    mime_offset = safe_c2rust___bswap_32(
        *((*cache)
            .buffer
            .offset(offset.wrapping_add(4 as xdg_uint32_t) as isize)
            as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    n_children = safe_c2rust___bswap_32(
        *((*cache)
            .buffer
            .offset(offset.wrapping_add(8 as xdg_uint32_t) as isize)
            as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    child_offset = safe_c2rust___bswap_32(
        *((*cache)
            .buffer
            .offset(offset.wrapping_add(12 as xdg_uint32_t) as isize)
            as *mut xdg_uint32_t),
    ) as xdg_uint32_t;
    if child_offset & 0x3 as xdg_uint32_t != 0
        || child_offset as size_t > (*cache).size
        || n_children as size_t
            > (*cache)
                .size
                .wrapping_sub(child_offset as size_t)
                .wrapping_div(20 as size_t)
    {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < depth {
        printf(b" \0" as *const u8 as *const ::core::ffi::c_char);
        i += 1;
    }
    printf(
        b"%c\0" as *const u8 as *const ::core::ffi::c_char,
        character,
    );
    if mime_offset != 0 {
        printf(
            b" - %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*cache).buffer.offset(mime_offset as isize),
        );
    }
    printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    if child_offset != 0 {
        k = 0 as xdg_uint32_t;
        while k < n_children {
            safe_c2rust_dump_glob_node(
                cache,
                child_offset.wrapping_add((20 as xdg_uint32_t).wrapping_mul(k)),
                depth + 1 as ::core::ffi::c_int,
            );
            k = k.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_cache_glob_dump() {
    let mut i: xdg_uint32_t = 0;
    let mut j: xdg_uint32_t = 0;
    i = 0 as xdg_uint32_t;
    while !(*safe_c2rust__caches.offset(i as isize)).is_null() {
        let mut cache: *mut XdgMimeCache = *safe_c2rust__caches.offset(i as isize);
        let mut list_offset: xdg_uint32_t = 0;
        let mut n_entries: xdg_uint32_t = 0;
        let mut offset: xdg_uint32_t = 0;
        if !(*cache).buffer.is_null() {
            list_offset = safe_c2rust___bswap_32(
                *((*cache).buffer.offset(16 as ::core::ffi::c_int as isize) as *mut xdg_uint32_t),
            ) as xdg_uint32_t;
            if list_offset & 0x3 as xdg_uint32_t != 0
                || list_offset as size_t > (*cache).size
                || 1 as size_t
                    > (*cache)
                        .size
                        .wrapping_sub(list_offset as size_t)
                        .wrapping_div(8 as size_t)
            {
                return;
            }
            n_entries = safe_c2rust___bswap_32(
                *((*cache).buffer.offset(list_offset as isize) as *mut xdg_uint32_t),
            ) as xdg_uint32_t;
            offset = safe_c2rust___bswap_32(
                *((*cache)
                    .buffer
                    .offset(list_offset.wrapping_add(4 as xdg_uint32_t) as isize)
                    as *mut xdg_uint32_t),
            ) as xdg_uint32_t;
            if offset & 0x3 as xdg_uint32_t != 0
                || offset as size_t > (*cache).size
                || n_entries as size_t
                    > (*cache)
                        .size
                        .wrapping_sub(offset as size_t)
                        .wrapping_div(20 as size_t)
            {
                return;
            }
            j = 0 as xdg_uint32_t;
            while j < n_entries {
                safe_c2rust_dump_glob_node(
                    cache,
                    offset.wrapping_add((20 as xdg_uint32_t).wrapping_mul(j)),
                    0 as ::core::ffi::c_int,
                );
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
}
