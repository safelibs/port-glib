extern "C" {
    pub type XdgMimeMagic;
    pub type _XdgMimeCache;
    pub type XdgIconList;
    pub type XdgParentList;
    pub type XdgAliasList;
    pub type XdgGlobHash;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn __gio_xdg_utf8_validate(source: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn __gio_xdg_get_base_name(file_name: *const ::core::ffi::c_char)
        -> *const ::core::ffi::c_char;
    fn _xdg_binary_or_text_fallback(
        data: *const ::core::ffi::c_void,
        len: size_t,
    ) -> *const ::core::ffi::c_char;
    fn __gio_xdg_glob_read_from_file(
        glob_hash: *mut XdgGlobHash,
        file_name: *const ::core::ffi::c_char,
        version_two: ::core::ffi::c_int,
    );
    fn __gio_xdg_hash_new() -> *mut XdgGlobHash;
    fn __gio_xdg_hash_free(glob_hash: *mut XdgGlobHash);
    fn __gio_xdg_hash_lookup_file_name(
        glob_hash: *mut XdgGlobHash,
        text: *const ::core::ffi::c_char,
        mime_types: *mut *const ::core::ffi::c_char,
        n_mime_types: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __gio_xdg_hash_dump(glob_hash: *mut XdgGlobHash);
    fn __gio_xdg_magic_new() -> *mut XdgMimeMagic;
    fn __gio_xdg_magic_read_from_file(
        mime_magic: *mut XdgMimeMagic,
        file_name: *const ::core::ffi::c_char,
    );
    fn __gio_xdg_magic_free(mime_magic: *mut XdgMimeMagic);
    fn __gio_xdg_magic_get_buffer_extents(mime_magic: *mut XdgMimeMagic) -> ::core::ffi::c_int;
    fn __gio_xdg_magic_lookup_data(
        mime_magic: *mut XdgMimeMagic,
        data: *const ::core::ffi::c_void,
        len: size_t,
        result_prio: *mut ::core::ffi::c_int,
        mime_types: *mut *const ::core::ffi::c_char,
        n_mime_types: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn __gio_xdg_alias_read_from_file(
        list: *mut XdgAliasList,
        file_name: *const ::core::ffi::c_char,
    );
    fn __gio_xdg_alias_list_new() -> *mut XdgAliasList;
    fn __gio_xdg_alias_list_free(list: *mut XdgAliasList);
    fn __gio_xdg_alias_list_lookup(
        list: *mut XdgAliasList,
        alias: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn __gio_xdg_alias_list_dump(list: *mut XdgAliasList);
    fn _gio_xdg_icon_read_from_file(list: *mut XdgIconList, file_name: *const ::core::ffi::c_char);
    fn _gio_xdg_icon_list_new() -> *mut XdgIconList;
    fn _gio_xdg_icon_list_free(list: *mut XdgIconList);
    fn _gio_xdg_icon_list_lookup(
        list: *mut XdgIconList,
        mime: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn __gio_xdg_parent_read_from_file(
        list: *mut XdgParentList,
        file_name: *const ::core::ffi::c_char,
    );
    fn __gio_xdg_parent_list_new() -> *mut XdgParentList;
    fn __gio_xdg_parent_list_free(list: *mut XdgParentList);
    fn __gio_xdg_parent_list_lookup(
        list: *mut XdgParentList,
        mime: *const ::core::ffi::c_char,
    ) -> *mut *const ::core::ffi::c_char;
    fn __gio_xdg_parent_list_dump(list: *mut XdgParentList);
    fn __gio_xdg_cache_new_from_file(file_name: *const ::core::ffi::c_char) -> *mut XdgMimeCache;
    fn __gio_xdg_cache_unref(cache: *mut XdgMimeCache);
    fn __gio_xdg_cache_get_mime_type_for_data(
        data: *const ::core::ffi::c_void,
        len: size_t,
        result_prio: *mut ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn __gio_xdg_cache_get_mime_type_for_file(
        file_name: *const ::core::ffi::c_char,
        statbuf: *mut stat,
    ) -> *const ::core::ffi::c_char;
    fn __gio_xdg_cache_get_mime_types_from_file_name(
        file_name: *const ::core::ffi::c_char,
        mime_types: *mut *const ::core::ffi::c_char,
        n_mime_types: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __gio_xdg_cache_get_mime_type_from_file_name(
        file_name: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn __gio_xdg_cache_mime_type_subclass(
        mime_a: *const ::core::ffi::c_char,
        mime_b: *const ::core::ffi::c_char,
        seen: *mut *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn __gio_xdg_cache_list_mime_parents(
        mime: *const ::core::ffi::c_char,
    ) -> *mut *mut ::core::ffi::c_char;
    fn __gio_xdg_cache_unalias_mime_type(
        mime: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn __gio_xdg_cache_get_max_buffer_extents() -> ::core::ffi::c_int;
    fn __gio_xdg_cache_get_icon(mime: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn __gio_xdg_cache_get_generic_icon(
        mime: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    fn __gio_xdg_cache_glob_dump();
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
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
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
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type XdgMimeCallback = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type XdgMimeDestroy = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type XdgMimeCache = _XdgMimeCache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XdgDirTimeList {
    pub mtime: time_t,
    pub directory_name: *mut ::core::ffi::c_char,
    pub checked: ::core::ffi::c_int,
    pub next: *mut XdgDirTimeList,
}
pub const XDG_CHECKED_UNCHECKED: C2RustUnnamed = 0;
pub type XdgDirectoryFunc = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XdgCallbackList {
    pub next: *mut XdgCallbackList,
    pub prev: *mut XdgCallbackList,
    pub callback_id: ::core::ffi::c_int,
    pub callback: XdgMimeCallback,
    pub data: *mut ::core::ffi::c_void,
    pub destroy: XdgMimeDestroy,
}
pub const XDG_CHECKED_VALID: C2RustUnnamed = 1;
pub const XDG_CHECKED_INVALID: C2RustUnnamed = 2;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
static mut safe_c2rust_need_reread: ::core::ffi::c_int = TRUE;
static mut safe_c2rust_last_stat_time: time_t = 0 as time_t;
static mut safe_c2rust_global_hash: *mut XdgGlobHash =
    ::core::ptr::null::<XdgGlobHash>() as *mut XdgGlobHash;
static mut safe_c2rust_global_magic: *mut XdgMimeMagic =
    ::core::ptr::null::<XdgMimeMagic>() as *mut XdgMimeMagic;
static mut safe_c2rust_alias_list: *mut XdgAliasList =
    ::core::ptr::null::<XdgAliasList>() as *mut XdgAliasList;
static mut safe_c2rust_parent_list: *mut XdgParentList =
    ::core::ptr::null::<XdgParentList>() as *mut XdgParentList;
static mut safe_c2rust_dir_time_list: *mut XdgDirTimeList =
    ::core::ptr::null::<XdgDirTimeList>() as *mut XdgDirTimeList;
static mut safe_c2rust_callback_list: *mut XdgCallbackList =
    ::core::ptr::null::<XdgCallbackList>() as *mut XdgCallbackList;
static mut safe_c2rust_icon_list: *mut XdgIconList =
    ::core::ptr::null::<XdgIconList>() as *mut XdgIconList;
static mut safe_c2rust_generic_icon_list: *mut XdgIconList =
    ::core::ptr::null::<XdgIconList>() as *mut XdgIconList;
static mut safe_c2rust_xdg_dirs: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut safe_c2rust__caches: *mut *mut XdgMimeCache =
    ::core::ptr::null::<*mut XdgMimeCache>() as *mut *mut XdgMimeCache;
static mut safe_c2rust_n_caches: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut safe_c2rust__gio_xdg_type_unknown: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"application/octet-stream\0")
};
#[no_mangle]
pub static mut safe_c2rust__gio_xdg_type_empty: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"application/x-zerosize\0")
};
#[no_mangle]
pub static mut safe_c2rust__gio_xdg_type_textplain: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"text/plain\0") };
unsafe extern "C" fn safe_c2rust_xdg_dir_time_list_add(
    mut file_name: *mut ::core::ffi::c_char,
    mut mtime: time_t,
) {
    let mut list: *mut XdgDirTimeList = ::core::ptr::null_mut::<XdgDirTimeList>();
    list = safe_c2rust_dir_time_list;
    while !list.is_null() {
        if strcmp((*list).directory_name, file_name) == 0 as ::core::ffi::c_int {
            free(file_name as *mut ::core::ffi::c_void);
            return;
        }
        list = (*list).next;
    }
    list = calloc(
        1 as size_t,
        ::core::mem::size_of::<XdgDirTimeList>() as size_t,
    ) as *mut XdgDirTimeList;
    (*list).checked = XDG_CHECKED_UNCHECKED as ::core::ffi::c_int;
    (*list).directory_name = file_name;
    (*list).mtime = mtime;
    (*list).next = safe_c2rust_dir_time_list;
    safe_c2rust_dir_time_list = list;
}
unsafe extern "C" fn safe_c2rust_xdg_dir_time_list_free(mut list: *mut XdgDirTimeList) {
    let mut next: *mut XdgDirTimeList = ::core::ptr::null_mut::<XdgDirTimeList>();
    while !list.is_null() {
        next = (*list).next;
        free((*list).directory_name as *mut ::core::ffi::c_void);
        free(list as *mut ::core::ffi::c_void);
        list = next;
    }
}
unsafe extern "C" fn safe_c2rust_xdg_mime_init_from_directory(
    mut directory: *const ::core::ffi::c_char,
    mut user_data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut file_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
    '_c2rust_label: {
        if !directory.is_null() {
        } else {
            __assert_fail(
                b"directory != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"../original/gio/xdgmime/xdgmime.c\0" as *const u8 as *const ::core::ffi::c_char,
                135 as ::core::ffi::c_uint,
                b"int xdg_mime_init_from_directory(const char *, void *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    file_name = malloc(
        strlen(directory)
            .wrapping_add(strlen(
                b"/mime.cache\0" as *const u8 as *const ::core::ffi::c_char,
            ))
            .wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    strcpy(file_name, directory);
    strcat(
        file_name,
        b"/mime.cache\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if stat(file_name, &raw mut st) == 0 as ::core::ffi::c_int {
        let mut cache: *mut XdgMimeCache = __gio_xdg_cache_new_from_file(file_name);
        if !cache.is_null() {
            safe_c2rust_xdg_dir_time_list_add(file_name, st.st_mtim.tv_sec as time_t);
            safe_c2rust__caches = realloc(
                safe_c2rust__caches as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<*mut XdgMimeCache>() as size_t)
                    .wrapping_mul((safe_c2rust_n_caches + 2 as ::core::ffi::c_int) as size_t),
            ) as *mut *mut XdgMimeCache;
            let ref mut fresh0 = *safe_c2rust__caches.offset(safe_c2rust_n_caches as isize);
            *fresh0 = cache;
            let ref mut fresh1 = *safe_c2rust__caches
                .offset((safe_c2rust_n_caches + 1 as ::core::ffi::c_int) as isize);
            *fresh1 = ::core::ptr::null_mut::<XdgMimeCache>();
            safe_c2rust_n_caches += 1;
            return FALSE;
        }
    }
    free(file_name as *mut ::core::ffi::c_void);
    file_name = malloc(
        strlen(directory)
            .wrapping_add(strlen(
                b"/globs2\0" as *const u8 as *const ::core::ffi::c_char,
            ))
            .wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    strcpy(file_name, directory);
    strcat(
        file_name,
        b"/globs2\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if stat(file_name, &raw mut st) == 0 as ::core::ffi::c_int {
        __gio_xdg_glob_read_from_file(safe_c2rust_global_hash, file_name, TRUE);
        safe_c2rust_xdg_dir_time_list_add(file_name, st.st_mtim.tv_sec as time_t);
    } else {
        free(file_name as *mut ::core::ffi::c_void);
        file_name = malloc(
            strlen(directory)
                .wrapping_add(strlen(
                    b"/globs\0" as *const u8 as *const ::core::ffi::c_char,
                ))
                .wrapping_add(1 as size_t),
        ) as *mut ::core::ffi::c_char;
        strcpy(file_name, directory);
        strcat(
            file_name,
            b"/globs\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if stat(file_name, &raw mut st) == 0 as ::core::ffi::c_int {
            __gio_xdg_glob_read_from_file(safe_c2rust_global_hash, file_name, FALSE);
            safe_c2rust_xdg_dir_time_list_add(file_name, st.st_mtim.tv_sec as time_t);
        } else {
            free(file_name as *mut ::core::ffi::c_void);
        }
    }
    file_name = malloc(
        strlen(directory)
            .wrapping_add(strlen(
                b"/magic\0" as *const u8 as *const ::core::ffi::c_char,
            ))
            .wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    strcpy(file_name, directory);
    strcat(
        file_name,
        b"/magic\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if stat(file_name, &raw mut st) == 0 as ::core::ffi::c_int {
        __gio_xdg_magic_read_from_file(safe_c2rust_global_magic, file_name);
        safe_c2rust_xdg_dir_time_list_add(file_name, st.st_mtim.tv_sec as time_t);
    } else {
        free(file_name as *mut ::core::ffi::c_void);
    }
    file_name = malloc(
        strlen(directory)
            .wrapping_add(strlen(
                b"/aliases\0" as *const u8 as *const ::core::ffi::c_char,
            ))
            .wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    strcpy(file_name, directory);
    strcat(
        file_name,
        b"/aliases\0" as *const u8 as *const ::core::ffi::c_char,
    );
    __gio_xdg_alias_read_from_file(safe_c2rust_alias_list, file_name);
    free(file_name as *mut ::core::ffi::c_void);
    file_name = malloc(
        strlen(directory)
            .wrapping_add(strlen(
                b"/subclasses\0" as *const u8 as *const ::core::ffi::c_char,
            ))
            .wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    strcpy(file_name, directory);
    strcat(
        file_name,
        b"/subclasses\0" as *const u8 as *const ::core::ffi::c_char,
    );
    __gio_xdg_parent_read_from_file(safe_c2rust_parent_list, file_name);
    free(file_name as *mut ::core::ffi::c_void);
    file_name = malloc(
        strlen(directory)
            .wrapping_add(strlen(
                b"/icons\0" as *const u8 as *const ::core::ffi::c_char,
            ))
            .wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    strcpy(file_name, directory);
    strcat(
        file_name,
        b"/icons\0" as *const u8 as *const ::core::ffi::c_char,
    );
    _gio_xdg_icon_read_from_file(safe_c2rust_icon_list, file_name);
    free(file_name as *mut ::core::ffi::c_void);
    file_name = malloc(
        strlen(directory)
            .wrapping_add(strlen(
                b"/generic-icons\0" as *const u8 as *const ::core::ffi::c_char,
            ))
            .wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    strcpy(file_name, directory);
    strcat(
        file_name,
        b"/generic-icons\0" as *const u8 as *const ::core::ffi::c_char,
    );
    _gio_xdg_icon_read_from_file(safe_c2rust_generic_icon_list, file_name);
    free(file_name as *mut ::core::ffi::c_void);
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_xdg_init_dirs() {
    let mut xdg_data_home: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut home: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut xdg_data_dirs: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut n_dirs: size_t = 0 as size_t;
    let mut i: size_t = 0;
    let mut current_dir: size_t = 0;
    '_c2rust_label: {
        if safe_c2rust_xdg_dirs.is_null() {
        } else {
            __assert_fail(
                b"xdg_dirs == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"../original/gio/xdgmime/xdgmime.c\0" as *const u8 as *const ::core::ffi::c_char,
                224 as ::core::ffi::c_uint,
                b"void xdg_init_dirs(void)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    xdg_data_home = getenv(b"XDG_DATA_HOME\0" as *const u8 as *const ::core::ffi::c_char);
    home = getenv(b"HOME\0" as *const u8 as *const ::core::ffi::c_char);
    xdg_data_dirs = getenv(b"XDG_DATA_DIRS\0" as *const u8 as *const ::core::ffi::c_char);
    if xdg_data_dirs.is_null() {
        xdg_data_dirs =
            b"/usr/local/share/:/usr/share/\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if !xdg_data_home.is_null() || !home.is_null() {
        n_dirs = n_dirs.wrapping_add(1);
    }
    n_dirs = n_dirs.wrapping_add(1);
    i = 0 as size_t;
    while *xdg_data_dirs.offset(i as isize) as ::core::ffi::c_int != '\0' as i32 {
        if *xdg_data_dirs.offset(i as isize) as ::core::ffi::c_int == ':' as i32 {
            n_dirs = n_dirs.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    safe_c2rust_xdg_dirs = calloc(
        n_dirs.wrapping_add(1 as size_t),
        ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t,
    ) as *mut *mut ::core::ffi::c_char;
    current_dir = 0 as size_t;
    if !xdg_data_home.is_null() {
        let mut mime_subdir: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        mime_subdir = malloc(
            strlen(xdg_data_home)
                .wrapping_add(strlen(
                    b"/mime/\0" as *const u8 as *const ::core::ffi::c_char,
                ))
                .wrapping_add(1 as size_t),
        ) as *mut ::core::ffi::c_char;
        strcpy(mime_subdir, xdg_data_home);
        strcat(
            mime_subdir,
            b"/mime/\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let fresh2 = current_dir;
        current_dir = current_dir.wrapping_add(1);
        let ref mut fresh3 = *safe_c2rust_xdg_dirs.offset(fresh2 as isize);
        *fresh3 = mime_subdir;
    } else if !home.is_null() {
        let mut guessed_xdg_home: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        guessed_xdg_home = malloc(
            strlen(home)
                .wrapping_add(strlen(
                    b"/.local/share/mime/\0" as *const u8 as *const ::core::ffi::c_char,
                ))
                .wrapping_add(1 as size_t),
        ) as *mut ::core::ffi::c_char;
        strcpy(guessed_xdg_home, home);
        strcat(
            guessed_xdg_home,
            b"/.local/share/mime/\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let fresh4 = current_dir;
        current_dir = current_dir.wrapping_add(1);
        let ref mut fresh5 = *safe_c2rust_xdg_dirs.offset(fresh4 as isize);
        *fresh5 = guessed_xdg_home;
    }
    ptr = xdg_data_dirs;
    while *ptr as ::core::ffi::c_int != '\0' as i32 {
        let mut end_ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut dir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut len: ::core::ffi::c_int = 0;
        end_ptr = ptr;
        while *end_ptr as ::core::ffi::c_int != ':' as i32
            && *end_ptr as ::core::ffi::c_int != '\0' as i32
        {
            end_ptr = end_ptr.offset(1);
        }
        if end_ptr == ptr {
            ptr = ptr.offset(1);
        } else {
            if *end_ptr as ::core::ffi::c_int == ':' as i32 {
                len = end_ptr.offset_from(ptr) as ::core::ffi::c_long as ::core::ffi::c_int;
            } else {
                len = (end_ptr.offset_from(ptr) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                    as ::core::ffi::c_int;
            }
            dir = malloc(
                (len as size_t)
                    .wrapping_add(strlen(
                        b"/mime/\0" as *const u8 as *const ::core::ffi::c_char,
                    ))
                    .wrapping_add(1 as size_t),
            ) as *mut ::core::ffi::c_char;
            strncpy(dir, ptr, len as size_t);
            *dir.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
            strcat(dir, b"/mime/\0" as *const u8 as *const ::core::ffi::c_char);
            let fresh6 = current_dir;
            current_dir = current_dir.wrapping_add(1);
            let ref mut fresh7 = *safe_c2rust_xdg_dirs.offset(fresh6 as isize);
            *fresh7 = dir;
            ptr = end_ptr;
        }
    }
    let ref mut fresh8 = *safe_c2rust_xdg_dirs.offset(current_dir as isize);
    *fresh8 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    safe_c2rust_need_reread = TRUE;
}
unsafe extern "C" fn safe_c2rust_xdg_run_command_on_dirs(
    mut func: XdgDirectoryFunc,
    mut user_data: *mut ::core::ffi::c_void,
) {
    let mut i: size_t = 0;
    if safe_c2rust_xdg_dirs.is_null() {
        safe_c2rust_xdg_init_dirs();
    }
    i = 0 as size_t;
    while !(*safe_c2rust_xdg_dirs.offset(i as isize)).is_null() {
        if func.expect("non-null function pointer")(
            *safe_c2rust_xdg_dirs.offset(i as isize),
            user_data,
        ) != 0
        {
            return;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_xdg_mime_set_dirs(
    mut dirs: *const *const ::core::ffi::c_char,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while !safe_c2rust_xdg_dirs.is_null() && !(*safe_c2rust_xdg_dirs.offset(i as isize)).is_null() {
        free(*safe_c2rust_xdg_dirs.offset(i as isize) as *mut ::core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    free(safe_c2rust_xdg_dirs as *mut ::core::ffi::c_void);
    safe_c2rust_xdg_dirs = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    if !dirs.is_null() {
        i = 0 as size_t;
        while !(*dirs.offset(i as isize)).is_null() {
            i = i.wrapping_add(1);
        }
        safe_c2rust_xdg_dirs = calloc(
            i.wrapping_add(1 as size_t),
            ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t,
        ) as *mut *mut ::core::ffi::c_char;
        i = 0 as size_t;
        while !(*dirs.offset(i as isize)).is_null() {
            let ref mut fresh11 = *safe_c2rust_xdg_dirs.offset(i as isize);
            *fresh11 = strdup(*dirs.offset(i as isize));
            i = i.wrapping_add(1);
        }
        let ref mut fresh12 = *safe_c2rust_xdg_dirs.offset(i as isize);
        *fresh12 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    safe_c2rust_need_reread = TRUE;
}
unsafe extern "C" fn safe_c2rust_xdg_check_file(
    mut file_path: *const ::core::ffi::c_char,
    mut exists: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
    if stat(file_path, &raw mut st) == 0 as ::core::ffi::c_int {
        let mut list: *mut XdgDirTimeList = ::core::ptr::null_mut::<XdgDirTimeList>();
        if !exists.is_null() {
            *exists = TRUE;
        }
        list = safe_c2rust_dir_time_list;
        while !list.is_null() {
            if strcmp((*list).directory_name, file_path) == 0 {
                if st.st_mtim.tv_sec == (*list).mtime {
                    (*list).checked = XDG_CHECKED_VALID as ::core::ffi::c_int;
                } else {
                    (*list).checked = XDG_CHECKED_INVALID as ::core::ffi::c_int;
                }
                return ((*list).checked != XDG_CHECKED_VALID as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            }
            list = (*list).next;
        }
        return TRUE;
    }
    if !exists.is_null() {
        *exists = FALSE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_xdg_check_dir(
    mut directory: *const ::core::ffi::c_char,
    mut user_data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut invalid: ::core::ffi::c_int = 0;
    let mut exists: ::core::ffi::c_int = 0;
    let mut file_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut invalid_dir_list: *mut ::core::ffi::c_int = user_data as *mut ::core::ffi::c_int;
    '_c2rust_label: {
        if !directory.is_null() {
        } else {
            __assert_fail(
                b"directory != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"../original/gio/xdgmime/xdgmime.c\0" as *const u8 as *const ::core::ffi::c_char,
                401 as ::core::ffi::c_uint,
                b"int xdg_check_dir(const char *, void *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    file_name = malloc(
        strlen(directory)
            .wrapping_add(strlen(
                b"/mime.cache\0" as *const u8 as *const ::core::ffi::c_char,
            ))
            .wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    strcpy(file_name, directory);
    strcat(
        file_name,
        b"/mime.cache\0" as *const u8 as *const ::core::ffi::c_char,
    );
    invalid = safe_c2rust_xdg_check_file(file_name, &raw mut exists);
    free(file_name as *mut ::core::ffi::c_void);
    if invalid != 0 {
        *invalid_dir_list = TRUE;
        return TRUE;
    } else if exists != 0 {
        return FALSE;
    }
    file_name = malloc(
        strlen(directory)
            .wrapping_add(strlen(
                b"/globs\0" as *const u8 as *const ::core::ffi::c_char,
            ))
            .wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    strcpy(file_name, directory);
    strcat(
        file_name,
        b"/globs\0" as *const u8 as *const ::core::ffi::c_char,
    );
    invalid = safe_c2rust_xdg_check_file(file_name, ::core::ptr::null_mut::<::core::ffi::c_int>());
    free(file_name as *mut ::core::ffi::c_void);
    if invalid != 0 {
        *invalid_dir_list = TRUE;
        return TRUE;
    }
    file_name = malloc(
        strlen(directory)
            .wrapping_add(strlen(
                b"/magic\0" as *const u8 as *const ::core::ffi::c_char,
            ))
            .wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    strcpy(file_name, directory);
    strcat(
        file_name,
        b"/magic\0" as *const u8 as *const ::core::ffi::c_char,
    );
    invalid = safe_c2rust_xdg_check_file(file_name, ::core::ptr::null_mut::<::core::ffi::c_int>());
    free(file_name as *mut ::core::ffi::c_void);
    if invalid != 0 {
        *invalid_dir_list = TRUE;
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_xdg_check_dirs() -> ::core::ffi::c_int {
    let mut list: *mut XdgDirTimeList = ::core::ptr::null_mut::<XdgDirTimeList>();
    let mut invalid_dir_list: ::core::ffi::c_int = FALSE;
    list = safe_c2rust_dir_time_list;
    while !list.is_null() {
        (*list).checked = XDG_CHECKED_UNCHECKED as ::core::ffi::c_int;
        list = (*list).next;
    }
    safe_c2rust_xdg_run_command_on_dirs(
        Some(
            safe_c2rust_xdg_check_dir
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut invalid_dir_list as *mut ::core::ffi::c_void,
    );
    if invalid_dir_list != 0 {
        return TRUE;
    }
    list = safe_c2rust_dir_time_list;
    while !list.is_null() {
        if (*list).checked != XDG_CHECKED_VALID as ::core::ffi::c_int {
            return TRUE;
        }
        list = (*list).next;
    }
    return FALSE;
}
unsafe extern "C" fn safe_c2rust_xdg_check_time_and_dirs() -> ::core::ffi::c_int {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut current_time: time_t = 0;
    let mut retval: ::core::ffi::c_int = FALSE;
    gettimeofday(&raw mut tv, NULL);
    current_time = tv.tv_sec as time_t;
    if current_time >= safe_c2rust_last_stat_time + 5 as time_t {
        retval = safe_c2rust_xdg_check_dirs();
        safe_c2rust_last_stat_time = current_time;
    }
    return retval;
}
unsafe extern "C" fn safe_c2rust_xdg_mime_init() {
    if safe_c2rust_xdg_check_time_and_dirs() != 0 {
        safe_c2rust__gio_xdg_shutdown();
    }
    if safe_c2rust_need_reread != 0 {
        safe_c2rust_global_hash = __gio_xdg_hash_new();
        safe_c2rust_global_magic = __gio_xdg_magic_new();
        safe_c2rust_alias_list = __gio_xdg_alias_list_new();
        safe_c2rust_parent_list = __gio_xdg_parent_list_new();
        safe_c2rust_icon_list = _gio_xdg_icon_list_new();
        safe_c2rust_generic_icon_list = _gio_xdg_icon_list_new();
        safe_c2rust_xdg_run_command_on_dirs(
            Some(
                safe_c2rust_xdg_mime_init_from_directory
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
        );
        safe_c2rust_need_reread = FALSE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_get_mime_type_for_data(
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
    mut result_prio: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut mime_type: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if len == 0 as size_t {
        if !result_prio.is_null() {
            *result_prio = 100 as ::core::ffi::c_int;
        }
        return &raw const safe_c2rust__gio_xdg_type_empty as *const ::core::ffi::c_char;
    }
    safe_c2rust_xdg_mime_init();
    if !safe_c2rust__caches.is_null() {
        mime_type = __gio_xdg_cache_get_mime_type_for_data(data, len, result_prio);
    } else {
        mime_type = __gio_xdg_magic_lookup_data(
            safe_c2rust_global_magic,
            data,
            len,
            result_prio,
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
        );
    }
    if !mime_type.is_null() {
        return mime_type;
    }
    return _xdg_binary_or_text_fallback(data, len);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_get_mime_type_for_file(
    mut file_name: *const ::core::ffi::c_char,
    mut statbuf: *mut stat,
) -> *const ::core::ffi::c_char {
    let mut mime_type: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut mime_types: [*const ::core::ffi::c_char; 5] =
        [::core::ptr::null::<::core::ffi::c_char>(); 5];
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
    safe_c2rust_xdg_mime_init();
    if !safe_c2rust__caches.is_null() {
        return __gio_xdg_cache_get_mime_type_for_file(file_name, statbuf);
    }
    base_name = __gio_xdg_get_base_name(file_name);
    n = __gio_xdg_hash_lookup_file_name(
        safe_c2rust_global_hash,
        base_name,
        &raw mut mime_types as *mut *const ::core::ffi::c_char,
        5 as ::core::ffi::c_int,
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
    if !((*statbuf).st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) {
        return &raw const safe_c2rust__gio_xdg_type_unknown as *const ::core::ffi::c_char;
    }
    max_extent = __gio_xdg_magic_get_buffer_extents(safe_c2rust_global_magic);
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
    mime_type = __gio_xdg_magic_lookup_data(
        safe_c2rust_global_magic,
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
pub unsafe extern "C" fn safe_c2rust__gio_xdg_get_mime_type_from_file_name(
    mut file_name: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut mime_type: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    safe_c2rust_xdg_mime_init();
    if !safe_c2rust__caches.is_null() {
        return __gio_xdg_cache_get_mime_type_from_file_name(file_name);
    }
    if __gio_xdg_hash_lookup_file_name(
        safe_c2rust_global_hash,
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
pub unsafe extern "C" fn safe_c2rust__gio_xdg_get_mime_types_from_file_name(
    mut file_name: *const ::core::ffi::c_char,
    mut mime_types: *mut *const ::core::ffi::c_char,
    mut n_mime_types: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    safe_c2rust_xdg_mime_init();
    if !safe_c2rust__caches.is_null() {
        return __gio_xdg_cache_get_mime_types_from_file_name(file_name, mime_types, n_mime_types);
    }
    return __gio_xdg_hash_lookup_file_name(
        safe_c2rust_global_hash,
        file_name,
        mime_types,
        n_mime_types,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_is_valid_mime_type(
    mut mime_type: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return __gio_xdg_utf8_validate(mime_type);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_shutdown() {
    let mut list: *mut XdgCallbackList = ::core::ptr::null_mut::<XdgCallbackList>();
    if !safe_c2rust_dir_time_list.is_null() {
        safe_c2rust_xdg_dir_time_list_free(safe_c2rust_dir_time_list);
        safe_c2rust_dir_time_list = ::core::ptr::null_mut::<XdgDirTimeList>();
    }
    if !safe_c2rust_global_hash.is_null() {
        __gio_xdg_hash_free(safe_c2rust_global_hash);
        safe_c2rust_global_hash = ::core::ptr::null_mut::<XdgGlobHash>();
    }
    if !safe_c2rust_global_magic.is_null() {
        __gio_xdg_magic_free(safe_c2rust_global_magic);
        safe_c2rust_global_magic = ::core::ptr::null_mut::<XdgMimeMagic>();
    }
    if !safe_c2rust_alias_list.is_null() {
        __gio_xdg_alias_list_free(safe_c2rust_alias_list);
        safe_c2rust_alias_list = ::core::ptr::null_mut::<XdgAliasList>();
    }
    if !safe_c2rust_parent_list.is_null() {
        __gio_xdg_parent_list_free(safe_c2rust_parent_list);
        safe_c2rust_parent_list = ::core::ptr::null_mut::<XdgParentList>();
    }
    if !safe_c2rust_icon_list.is_null() {
        _gio_xdg_icon_list_free(safe_c2rust_icon_list);
        safe_c2rust_icon_list = ::core::ptr::null_mut::<XdgIconList>();
    }
    if !safe_c2rust_generic_icon_list.is_null() {
        _gio_xdg_icon_list_free(safe_c2rust_generic_icon_list);
        safe_c2rust_generic_icon_list = ::core::ptr::null_mut::<XdgIconList>();
    }
    if !safe_c2rust__caches.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < safe_c2rust_n_caches {
            __gio_xdg_cache_unref(*safe_c2rust__caches.offset(i as isize));
            i += 1;
        }
        free(safe_c2rust__caches as *mut ::core::ffi::c_void);
        safe_c2rust__caches = ::core::ptr::null_mut::<*mut XdgMimeCache>();
        safe_c2rust_n_caches = 0 as ::core::ffi::c_int;
    }
    list = safe_c2rust_callback_list;
    while !list.is_null() {
        (*list).callback.expect("non-null function pointer")((*list).data);
        list = (*list).next;
    }
    safe_c2rust_need_reread = TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_get_max_buffer_extents() -> ::core::ffi::c_int {
    safe_c2rust_xdg_mime_init();
    if !safe_c2rust__caches.is_null() {
        return __gio_xdg_cache_get_max_buffer_extents();
    }
    return __gio_xdg_magic_get_buffer_extents(safe_c2rust_global_magic);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_unalias_mime_type(
    mut mime_type: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut lookup: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !safe_c2rust__caches.is_null() {
        return __gio_xdg_cache_unalias_mime_type(mime_type);
    }
    lookup = __gio_xdg_alias_list_lookup(safe_c2rust_alias_list, mime_type);
    if !lookup.is_null() {
        return lookup;
    }
    return mime_type;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_unalias_mime_type(
    mut mime_type: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    safe_c2rust_xdg_mime_init();
    return safe_c2rust___gio_xdg_unalias_mime_type(mime_type);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_mime_type_equal(
    mut mime_a: *const ::core::ffi::c_char,
    mut mime_b: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut unalias_a: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut unalias_b: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    unalias_a = safe_c2rust___gio_xdg_unalias_mime_type(mime_a);
    unalias_b = safe_c2rust___gio_xdg_unalias_mime_type(mime_b);
    if strcmp(unalias_a, unalias_b) == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_mime_type_equal(
    mut mime_a: *const ::core::ffi::c_char,
    mut mime_b: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    safe_c2rust_xdg_mime_init();
    return safe_c2rust___gio_xdg_mime_type_equal(mime_a, mime_b);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_media_type_equal(
    mut mime_a: *const ::core::ffi::c_char,
    mut mime_b: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut sep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sep = strchr(mime_a, '/' as i32);
    if !sep.is_null()
        && strncmp(
            mime_a,
            mime_b,
            (sep.offset_from(mime_a) as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn safe_c2rust_xdg_mime_is_super_type(
    mut mime: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return safe_c2rust_ends_with(mime, b"/*\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust___gio_xdg_mime_type_subclass(
    mut mime: *const ::core::ffi::c_char,
    mut base: *const ::core::ffi::c_char,
    mut seen: *mut *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut umime: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ubase: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut parent: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut parents: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut first_seen: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut new_seen: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !safe_c2rust__caches.is_null() {
        return __gio_xdg_cache_mime_type_subclass(
            mime,
            base,
            ::core::ptr::null_mut::<*mut *const ::core::ffi::c_char>(),
        );
    }
    umime = safe_c2rust___gio_xdg_unalias_mime_type(mime);
    ubase = safe_c2rust___gio_xdg_unalias_mime_type(base);
    if strcmp(umime, ubase) == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if safe_c2rust_xdg_mime_is_super_type(ubase) != 0
        && safe_c2rust__gio_xdg_media_type_equal(umime, ubase) != 0
    {
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
    parents = __gio_xdg_parent_list_lookup(safe_c2rust_parent_list, umime);
    while !parents.is_null() && !(*parents).is_null() {
        parent = *parents;
        i = 0 as ::core::ffi::c_int;
        loop {
            if (*(*seen).offset(i as isize)).is_null() {
                current_block = 17833034027772472439;
                break;
            }
            if parent == *(*seen).offset(i as isize) {
                current_block = 10599921512955367680;
                break;
            }
            i += 1;
        }
        match current_block {
            17833034027772472439 => {
                new_seen = realloc(
                    *seen as *mut ::core::ffi::c_void,
                    ((i + 2 as ::core::ffi::c_int) as size_t)
                        .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
                ) as *mut *const ::core::ffi::c_char;
                if new_seen.is_null() {
                    break;
                }
                let ref mut fresh9 = *new_seen.offset(i as isize);
                *fresh9 = parent;
                let ref mut fresh10 = *new_seen.offset((i + 1 as ::core::ffi::c_int) as isize);
                *fresh10 = ::core::ptr::null::<::core::ffi::c_char>();
                *seen = new_seen;
                if safe_c2rust___gio_xdg_mime_type_subclass(parent, ubase, seen) != 0 {
                    ret = 1 as ::core::ffi::c_int;
                    break;
                }
            }
            _ => {}
        }
        parents = parents.offset(1);
    }
    free(first_seen as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_mime_type_subclass(
    mut mime: *const ::core::ffi::c_char,
    mut base: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    safe_c2rust_xdg_mime_init();
    return safe_c2rust___gio_xdg_mime_type_subclass(
        mime,
        base,
        ::core::ptr::null_mut::<*mut *const ::core::ffi::c_char>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_list_mime_parents(
    mut mime: *const ::core::ffi::c_char,
) -> *mut *mut ::core::ffi::c_char {
    let mut parents: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut result: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    safe_c2rust_xdg_mime_init();
    if !safe_c2rust__caches.is_null() {
        return __gio_xdg_cache_list_mime_parents(mime);
    }
    parents = safe_c2rust__gio_xdg_get_mime_parents(mime);
    if parents.is_null() {
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    i = 0 as ::core::ffi::c_int;
    while !(*parents.offset(i as isize)).is_null() {
        i += 1;
    }
    n = ((i + 1 as ::core::ffi::c_int) as usize)
        .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
        as ::core::ffi::c_int;
    result = malloc(n as size_t) as *mut *mut ::core::ffi::c_char;
    memcpy(
        result as *mut ::core::ffi::c_void,
        parents as *const ::core::ffi::c_void,
        n as size_t,
    );
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_get_mime_parents(
    mut mime: *const ::core::ffi::c_char,
) -> *mut *const ::core::ffi::c_char {
    let mut umime: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    safe_c2rust_xdg_mime_init();
    umime = safe_c2rust___gio_xdg_unalias_mime_type(mime);
    return __gio_xdg_parent_list_lookup(safe_c2rust_parent_list, umime);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_dump() {
    safe_c2rust_xdg_mime_init();
    printf(b"*** ALIASES ***\n\n\0" as *const u8 as *const ::core::ffi::c_char);
    __gio_xdg_alias_list_dump(safe_c2rust_alias_list);
    printf(b"\n*** PARENTS ***\n\n\0" as *const u8 as *const ::core::ffi::c_char);
    __gio_xdg_parent_list_dump(safe_c2rust_parent_list);
    printf(b"\n*** CACHE ***\n\n\0" as *const u8 as *const ::core::ffi::c_char);
    __gio_xdg_hash_dump(safe_c2rust_global_hash);
    printf(b"\n*** GLOBS ***\n\n\0" as *const u8 as *const ::core::ffi::c_char);
    __gio_xdg_hash_dump(safe_c2rust_global_hash);
    printf(b"\n*** GLOBS REVERSE TREE ***\n\n\0" as *const u8 as *const ::core::ffi::c_char);
    __gio_xdg_cache_glob_dump();
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_register_reload_callback(
    mut callback: XdgMimeCallback,
    mut data: *mut ::core::ffi::c_void,
    mut destroy: XdgMimeDestroy,
) -> ::core::ffi::c_int {
    let mut list_el: *mut XdgCallbackList = ::core::ptr::null_mut::<XdgCallbackList>();
    static mut safe_c2rust_callback_id: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    list_el = calloc(
        1 as size_t,
        ::core::mem::size_of::<XdgCallbackList>() as size_t,
    ) as *mut XdgCallbackList;
    (*list_el).callback_id = safe_c2rust_callback_id;
    (*list_el).callback = callback;
    (*list_el).data = data;
    (*list_el).destroy = destroy;
    (*list_el).next = safe_c2rust_callback_list;
    if !(*list_el).next.is_null() {
        (*(*list_el).next).prev = list_el;
    }
    safe_c2rust_callback_list = list_el;
    safe_c2rust_callback_id += 1;
    return safe_c2rust_callback_id - 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_remove_callback(mut callback_id: ::core::ffi::c_int) {
    let mut list: *mut XdgCallbackList = ::core::ptr::null_mut::<XdgCallbackList>();
    list = safe_c2rust_callback_list;
    while !list.is_null() {
        if (*list).callback_id == callback_id {
            if !(*list).next.is_null() {
                (*list).next = (*list).prev;
            }
            if !(*list).prev.is_null() {
                (*(*list).prev).next = (*list).next;
            } else {
                safe_c2rust_callback_list = (*list).next;
            }
            (*list).destroy.expect("non-null function pointer")((*list).data);
            free(list as *mut ::core::ffi::c_void);
            return;
        }
        list = (*list).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_get_icon(
    mut mime: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    safe_c2rust_xdg_mime_init();
    if !safe_c2rust__caches.is_null() {
        return __gio_xdg_cache_get_icon(mime);
    }
    return _gio_xdg_icon_list_lookup(safe_c2rust_icon_list, mime);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__gio_xdg_get_generic_icon(
    mut mime: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    safe_c2rust_xdg_mime_init();
    if !safe_c2rust__caches.is_null() {
        return __gio_xdg_cache_get_generic_icon(mime);
    }
    return _gio_xdg_icon_list_lookup(safe_c2rust_generic_icon_list, mime);
}
