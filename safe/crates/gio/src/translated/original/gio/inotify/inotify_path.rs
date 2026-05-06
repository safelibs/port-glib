extern "C" {
    pub type _GHashTable;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_list_prepend(list: *mut GList, data: gpointer) -> *mut GList;
    fn g_list_remove(list: *mut GList, data: gconstpointer) -> *mut GList;
    fn g_list_foreach(list: *mut GList, func: GFunc, user_data: gpointer);
    fn g_hash_table_new(hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
    fn g_hash_table_insert(hash_table: *mut GHashTable, key: gpointer, value: gpointer)
        -> gboolean;
    fn g_hash_table_replace(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_hash_table_size(hash_table: *mut GHashTable) -> guint;
    fn g_hash_table_unref(hash_table: *mut GHashTable);
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn g_direct_hash(v: gconstpointer) -> guint;
    fn g_direct_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_strjoin(separator: *const gchar, ...) -> *mut gchar;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_assertion_message_cmpint(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
        arg1: guint64,
        cmp: *const ::core::ffi::c_char,
        arg2: guint64,
        numtype: ::core::ffi::c_char,
    );
    fn _ik_startup(cb: Option<unsafe extern "C" fn(*mut ik_event_t) -> gboolean>) -> gboolean;
    fn _ik_event_free(event: *mut ik_event_t);
    fn _ik_watch(
        path: *const ::core::ffi::c_char,
        mask: guint32,
        err: *mut ::core::ffi::c_int,
    ) -> gint32;
    fn _ik_ignore(path: *const ::core::ffi::c_char, wd: gint32) -> ::core::ffi::c_int;
    fn _im_add(sub: *mut inotify_sub);
}
pub type size_t = usize;
pub type gint32 = ::core::ffi::c_int;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GEqualFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean>;
pub type GFunc = Option<unsafe extern "C" fn(gpointer, gpointer) -> ()>;
pub type GHashFunc = Option<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
pub type GHashTable = _GHashTable;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ik_event_s {
    pub wd: gint32,
    pub mask: guint32,
    pub original_mask: guint32,
    pub cookie: guint32,
    pub len: guint32,
    pub name: *mut ::core::ffi::c_char,
    pub is_second_in_pair: gboolean,
    pub pair: *mut ik_event_s,
    pub timestamp: gint64,
}
pub type ik_event_t = ik_event_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inotify_sub {
    pub dirname: *mut gchar,
    pub filename: *mut gchar,
    pub cancelled: gboolean,
    pub user_data: gpointer,
    pub pair_moves: gboolean,
    pub hardlinks: gboolean,
}
pub type ip_watched_dir_t = ip_watched_dir_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_watched_dir_s {
    pub path: *mut ::core::ffi::c_char,
    pub parent: *mut ip_watched_dir_s,
    pub children: *mut GList,
    pub files_hash: *mut GHashTable,
    pub wd: gint32,
    pub subs: *mut GList,
}
pub type ip_watched_file_t = ip_watched_file_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_watched_file_s {
    pub filename: *mut gchar,
    pub path: *mut gchar,
    pub wd: gint32,
    pub subs: *mut GList,
}
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"ip_unmap_wd\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
pub const IN_MODIFY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const IN_ATTRIB: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const IN_CLOSE_WRITE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const IN_MOVED_FROM: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const IN_MOVED_TO: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const IN_CREATE: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const IN_DELETE: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const IN_DELETE_SELF: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const IN_MOVE_SELF: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const IN_UNMOUNT: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const IN_Q_OVERFLOW: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
pub const IN_IGNORED: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const IN_ONLYDIR: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
#[inline(always)]
unsafe extern "C" fn safe_c2rust_g_strdup_inline(
    mut str: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if 0 != 0 && str.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if 0 != 0 && !str.is_null() && 0 != 0 {
        let len: size_t = (strlen(str) as size_t).wrapping_add(1 as size_t);
        let mut dup_str: *mut ::core::ffi::c_char =
            g_malloc(len as gsize) as *mut ::core::ffi::c_char;
        return memcpy(
            dup_str as *mut ::core::ffi::c_void,
            str as *const ::core::ffi::c_void,
            len,
        ) as *mut ::core::ffi::c_char;
    }
    return g_strdup(str as *const gchar) as *mut ::core::ffi::c_char;
}
pub const IP_INOTIFY_DIR_MASK: ::core::ffi::c_int = IN_MODIFY
    | IN_ATTRIB
    | IN_MOVED_FROM
    | IN_MOVED_TO
    | IN_DELETE
    | IN_CREATE
    | IN_DELETE_SELF
    | IN_UNMOUNT
    | IN_MOVE_SELF
    | IN_CLOSE_WRITE;
pub const IP_INOTIFY_FILE_MASK: ::core::ffi::c_int = IN_MODIFY | IN_ATTRIB | IN_CLOSE_WRITE;
static mut safe_c2rust_ip_debug_enabled: gboolean = FALSE;
static mut safe_c2rust_path_dir_hash: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut safe_c2rust_sub_dir_hash: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut safe_c2rust_wd_dir_hash: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut safe_c2rust_wd_file_hash: *mut GHashTable =
    ::core::ptr::null::<GHashTable>() as *mut GHashTable;
static mut safe_c2rust_event_callback: Option<
    unsafe extern "C" fn(*mut ik_event_t, *mut inotify_sub, gboolean) -> gboolean,
> = None;
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__ip_startup(
    mut cb: Option<unsafe extern "C" fn(*mut ik_event_t, *mut inotify_sub, gboolean) -> gboolean>,
) -> gboolean {
    static mut safe_c2rust_initialized: gboolean = FALSE;
    static mut safe_c2rust_result: gboolean = FALSE;
    if safe_c2rust_initialized == TRUE {
        return safe_c2rust_result;
    }
    safe_c2rust_event_callback = cb;
    safe_c2rust_result = _ik_startup(Some(
        safe_c2rust_ip_event_callback as unsafe extern "C" fn(*mut ik_event_t) -> gboolean,
    ));
    if safe_c2rust_result == 0 {
        return FALSE;
    }
    safe_c2rust_path_dir_hash = g_hash_table_new(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    safe_c2rust_sub_dir_hash = g_hash_table_new(
        Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    safe_c2rust_wd_dir_hash = g_hash_table_new(
        Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    safe_c2rust_wd_file_hash = g_hash_table_new(
        Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_direct_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    safe_c2rust_initialized = TRUE as gboolean;
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_ip_map_path_dir(
    mut path: *const ::core::ffi::c_char,
    mut dir: *mut ip_watched_dir_t,
) {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !path.is_null() && !dir.is_null() {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            135 as ::core::ffi::c_int,
            G_STRFUNC,
            b"path && dir\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_hash_table_insert(
        safe_c2rust_path_dir_hash,
        (*dir).path as gpointer,
        dir as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_ip_map_sub_dir(
    mut sub: *mut inotify_sub,
    mut dir: *mut ip_watched_dir_t,
) {
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !dir.is_null() && !sub.is_null() {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            144 as ::core::ffi::c_int,
            G_STRFUNC,
            b"dir && sub\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_hash_table_insert(safe_c2rust_sub_dir_hash, sub as gpointer, dir as gpointer);
    (*dir).subs = g_list_prepend((*dir).subs, sub as gpointer);
}
unsafe extern "C" fn safe_c2rust_ip_map_wd_dir(mut wd: gint32, mut dir: *mut ip_watched_dir_t) {
    let mut dir_list: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if wd >= 0 as ::core::ffi::c_int && !dir.is_null() {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            155 as ::core::ffi::c_int,
            G_STRFUNC,
            b"wd >= 0 && dir\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    dir_list = g_hash_table_lookup(
        safe_c2rust_wd_dir_hash,
        wd as glong as gpointer as gconstpointer,
    ) as *mut GList;
    dir_list = g_list_prepend(dir_list, dir as gpointer);
    g_hash_table_replace(
        safe_c2rust_wd_dir_hash,
        (*dir).wd as glong as gpointer,
        dir_list as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_ip_map_wd_file(mut wd: gint32, mut file: *mut ip_watched_file_t) {
    let mut file_list: *mut GList = ::core::ptr::null_mut::<GList>();
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if wd >= 0 as ::core::ffi::c_int && !file.is_null() {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            167 as ::core::ffi::c_int,
            G_STRFUNC,
            b"wd >= 0 && file\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    file_list = g_hash_table_lookup(
        safe_c2rust_wd_file_hash,
        wd as glong as gpointer as gconstpointer,
    ) as *mut GList;
    file_list = g_list_prepend(file_list, file as gpointer);
    g_hash_table_replace(
        safe_c2rust_wd_file_hash,
        wd as glong as gpointer,
        file_list as gpointer,
    );
}
unsafe extern "C" fn safe_c2rust_ip_unmap_wd_file(
    mut wd: gint32,
    mut file: *mut ip_watched_file_t,
) {
    let mut file_list: *mut GList = g_hash_table_lookup(
        safe_c2rust_wd_file_hash,
        wd as glong as gpointer as gconstpointer,
    ) as *mut GList;
    if file_list.is_null() {
        return;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if wd >= 0 as ::core::ffi::c_int && !file.is_null() {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            182 as ::core::ffi::c_int,
            G_STRFUNC,
            b"wd >= 0 && file\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    file_list = g_list_remove(file_list, file as gconstpointer);
    if file_list.is_null() {
        g_hash_table_remove(
            safe_c2rust_wd_file_hash,
            wd as glong as gpointer as gconstpointer,
        );
    } else {
        g_hash_table_replace(
            safe_c2rust_wd_file_hash,
            wd as glong as gpointer,
            file_list as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust_ip_watched_file_new(
    mut dirname: *const gchar,
    mut filename: *const gchar,
) -> *mut ip_watched_file_t {
    let mut file: *mut ip_watched_file_t = ::core::ptr::null_mut::<ip_watched_file_t>();
    file = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<ip_watched_file_t>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut ip_watched_file_t;
    (*file).path = g_strjoin(
        b"/\0" as *const u8 as *const gchar,
        dirname,
        filename,
        NULL_0,
    );
    (*file).filename =
        safe_c2rust_g_strdup_inline(filename as *const ::core::ffi::c_char) as *mut gchar;
    (*file).wd = -(1 as ::core::ffi::c_int) as gint32;
    return file;
}
unsafe extern "C" fn safe_c2rust_ip_watched_file_free(mut file: *mut ip_watched_file_t) {
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if (*file).subs.is_null() {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            208 as ::core::ffi::c_int,
            G_STRFUNC,
            b"file->subs == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_free((*file).filename as gpointer);
    g_free((*file).path as gpointer);
    g_free(file as gpointer);
}
unsafe extern "C" fn safe_c2rust_ip_watched_file_add_sub(
    mut file: *mut ip_watched_file_t,
    mut sub: *mut inotify_sub,
) {
    (*file).subs = g_list_prepend((*file).subs, sub as gpointer);
}
unsafe extern "C" fn safe_c2rust_ip_watched_file_start(mut file: *mut ip_watched_file_t) {
    if (*file).wd < 0 as ::core::ffi::c_int {
        let mut err: gint = 0;
        (*file).wd = _ik_watch((*file).path, IP_INOTIFY_FILE_MASK as guint32, &raw mut err);
        if (*file).wd >= 0 as ::core::ffi::c_int {
            safe_c2rust_ip_map_wd_file((*file).wd, file);
        }
    }
}
unsafe extern "C" fn safe_c2rust_ip_watched_file_stop(mut file: *mut ip_watched_file_t) {
    if (*file).wd >= 0 as ::core::ffi::c_int {
        _ik_ignore((*file).path, (*file).wd);
        safe_c2rust_ip_unmap_wd_file((*file).wd, file);
        (*file).wd = -(1 as ::core::ffi::c_int) as gint32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__ip_start_watching(mut sub: *mut inotify_sub) -> gboolean {
    let mut wd: gint32 = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut dir: *mut ip_watched_dir_t = ::core::ptr::null_mut::<ip_watched_dir_t>();
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !sub.is_null() {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            255 as ::core::ffi::c_int,
            G_STRFUNC,
            b"sub\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if (*sub).cancelled == 0 {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            256 as ::core::ffi::c_int,
            G_STRFUNC,
            b"!sub->cancelled\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if !(*sub).dirname.is_null() {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            257 as ::core::ffi::c_int,
            G_STRFUNC,
            b"sub->dirname\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if safe_c2rust_ip_debug_enabled != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Starting to watch %s\n\0" as *const u8 as *const gchar,
            (*sub).dirname,
        );
    }
    dir = g_hash_table_lookup(safe_c2rust_path_dir_hash, (*sub).dirname as gconstpointer)
        as *mut ip_watched_dir_t;
    if dir.is_null() {
        if safe_c2rust_ip_debug_enabled != 0 {
            g_log(
                G_LOG_DOMAIN.as_ptr() as *const gchar,
                G_LOG_LEVEL_WARNING,
                b"Trying to add inotify watch \0" as *const u8 as *const gchar,
            );
        }
        wd = _ik_watch(
            (*sub).dirname,
            (IP_INOTIFY_DIR_MASK | IN_ONLYDIR) as guint32,
            &raw mut err,
        );
        if wd < 0 as ::core::ffi::c_int {
            if safe_c2rust_ip_debug_enabled != 0 {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Failed\n\0" as *const u8 as *const gchar,
                );
            }
            return FALSE;
        } else {
            if safe_c2rust_ip_debug_enabled != 0 {
                g_log(
                    G_LOG_DOMAIN.as_ptr() as *const gchar,
                    G_LOG_LEVEL_WARNING,
                    b"Success\n\0" as *const u8 as *const gchar,
                );
            }
            dir = safe_c2rust_ip_watched_dir_new((*sub).dirname, wd);
            safe_c2rust_ip_map_wd_dir(wd, dir);
            safe_c2rust_ip_map_path_dir((*sub).dirname, dir);
        }
    } else if safe_c2rust_ip_debug_enabled != 0 {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"Already watching\n\0" as *const u8 as *const gchar,
        );
    }
    if (*sub).hardlinks != 0 {
        let mut file: *mut ip_watched_file_t = ::core::ptr::null_mut::<ip_watched_file_t>();
        file = g_hash_table_lookup((*dir).files_hash, (*sub).filename as gconstpointer)
            as *mut ip_watched_file_t;
        if file.is_null() {
            file = safe_c2rust_ip_watched_file_new((*sub).dirname, (*sub).filename);
            g_hash_table_insert(
                (*dir).files_hash,
                (*file).filename as gpointer,
                file as gpointer,
            );
        }
        safe_c2rust_ip_watched_file_add_sub(file, sub);
        safe_c2rust_ip_watched_file_start(file);
    }
    safe_c2rust_ip_map_sub_dir(sub, dir);
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_ip_unmap_path_dir(
    mut path: *const ::core::ffi::c_char,
    mut dir: *mut ip_watched_dir_t,
) {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if !path.is_null() && !dir.is_null() {
            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_17
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            310 as ::core::ffi::c_int,
            G_STRFUNC,
            b"path && dir\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_hash_table_remove(safe_c2rust_path_dir_hash, (*dir).path as gconstpointer);
}
unsafe extern "C" fn safe_c2rust_ip_unmap_wd_dir(mut wd: gint32, mut dir: *mut ip_watched_dir_t) {
    let mut dir_list: *mut GList = g_hash_table_lookup(
        safe_c2rust_wd_dir_hash,
        wd as glong as gpointer as gconstpointer,
    ) as *mut GList;
    if dir_list.is_null() {
        return;
    }
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if wd >= 0 as ::core::ffi::c_int && !dir.is_null() {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            323 as ::core::ffi::c_int,
            G_STRFUNC,
            b"wd >= 0 && dir\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    dir_list = g_list_remove(dir_list, dir as gconstpointer);
    if dir_list.is_null() {
        g_hash_table_remove(
            safe_c2rust_wd_dir_hash,
            (*dir).wd as glong as gpointer as gconstpointer,
        );
    } else {
        g_hash_table_replace(
            safe_c2rust_wd_dir_hash,
            (*dir).wd as glong as gpointer,
            dir_list as gpointer,
        );
    };
}
unsafe extern "C" fn safe_c2rust_ip_unmap_wd(mut wd: gint32) {
    let mut dir_list: *mut GList = g_hash_table_lookup(
        safe_c2rust_wd_dir_hash,
        wd as glong as gpointer as gconstpointer,
    ) as *mut GList;
    if dir_list.is_null() {
        return;
    }
    if ({
        let mut _g_boolean_var_19: ::core::ffi::c_int = 0;
        if wd >= 0 as ::core::ffi::c_int {
            _g_boolean_var_19 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_19 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_19
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            337 as ::core::ffi::c_int,
            G_STRFUNC,
            b"wd >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_hash_table_remove(
        safe_c2rust_wd_dir_hash,
        wd as glong as gpointer as gconstpointer,
    );
    g_list_free(dir_list);
}
unsafe extern "C" fn safe_c2rust_ip_unmap_sub_dir(
    mut sub: *mut inotify_sub,
    mut dir: *mut ip_watched_dir_t,
) {
    if ({
        let mut _g_boolean_var_20: ::core::ffi::c_int = 0;
        if !sub.is_null() && !dir.is_null() {
            _g_boolean_var_20 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_20 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_20
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            346 as ::core::ffi::c_int,
            G_STRFUNC,
            b"sub && dir\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_hash_table_remove(safe_c2rust_sub_dir_hash, sub as gconstpointer);
    (*dir).subs = g_list_remove((*dir).subs, sub as gconstpointer);
    if (*sub).hardlinks != 0 {
        let mut file: *mut ip_watched_file_t = ::core::ptr::null_mut::<ip_watched_file_t>();
        file = g_hash_table_lookup((*dir).files_hash, (*sub).filename as gconstpointer)
            as *mut ip_watched_file_t;
        (*file).subs = g_list_remove((*file).subs, sub as gconstpointer);
        if (*file).subs.is_null() {
            g_hash_table_remove((*dir).files_hash, (*sub).filename as gconstpointer);
            safe_c2rust_ip_watched_file_stop(file);
            safe_c2rust_ip_watched_file_free(file);
        }
    }
}
unsafe extern "C" fn safe_c2rust_ip_unmap_all_subs(mut dir: *mut ip_watched_dir_t) {
    while !(*dir).subs.is_null() {
        safe_c2rust_ip_unmap_sub_dir((*(*dir).subs).data as *mut inotify_sub, dir);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__ip_stop_watching(mut sub: *mut inotify_sub) -> gboolean {
    let mut dir: *mut ip_watched_dir_t = ::core::ptr::null_mut::<ip_watched_dir_t>();
    dir = g_hash_table_lookup(safe_c2rust_sub_dir_hash, sub as gconstpointer)
        as *mut ip_watched_dir_t;
    if dir.is_null() {
        return TRUE;
    }
    safe_c2rust_ip_unmap_sub_dir(sub, dir);
    if (*dir).subs.is_null() {
        _ik_ignore((*dir).path, (*dir).wd);
        safe_c2rust_ip_unmap_wd_dir((*dir).wd, dir);
        safe_c2rust_ip_unmap_path_dir((*dir).path, dir);
        safe_c2rust_ip_watched_dir_free(dir);
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_ip_watched_dir_new(
    mut path: *const ::core::ffi::c_char,
    mut wd: gint32,
) -> *mut ip_watched_dir_t {
    let mut dir: *mut ip_watched_dir_t = ({
        let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
        let mut __s: gsize = ::core::mem::size_of::<ip_watched_dir_t>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc0(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc0(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc0_n(__n, __s);
        }
        __p
    }) as *mut ip_watched_dir_t;
    (*dir).path = safe_c2rust_g_strdup_inline(path);
    (*dir).files_hash = g_hash_table_new(
        Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
        Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
    );
    (*dir).wd = wd;
    return dir;
}
unsafe extern "C" fn safe_c2rust_ip_watched_dir_free(mut dir: *mut ip_watched_dir_t) {
    let mut __n1: gint64 = g_hash_table_size((*dir).files_hash) as gint64;
    let mut __n2: gint64 = 0 as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpint(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            413 as ::core::ffi::c_int,
            G_STRFUNC,
            b"g_hash_table_size (dir->files_hash) == 0\0" as *const u8
                as *const ::core::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const ::core::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as ::core::ffi::c_char,
        );
    }
    if ({
        let mut _g_boolean_var_21: ::core::ffi::c_int = 0;
        if (*dir).subs.is_null() {
            _g_boolean_var_21 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_21 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_21
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            414 as ::core::ffi::c_int,
            G_STRFUNC,
            b"dir->subs == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    g_free((*dir).path as gpointer);
    g_hash_table_unref((*dir).files_hash);
    g_free(dir as gpointer);
}
unsafe extern "C" fn safe_c2rust_ip_wd_delete(mut data: gpointer, mut user_data: gpointer) {
    let mut dir: *mut ip_watched_dir_t = data as *mut ip_watched_dir_t;
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    l = (*dir).subs;
    while !l.is_null() {
        let mut sub: *mut inotify_sub = (*l).data as *mut inotify_sub;
        _im_add(sub);
        l = (*l).next;
    }
    safe_c2rust_ip_unmap_all_subs(dir);
    safe_c2rust_ip_unmap_path_dir((*dir).path, dir);
    safe_c2rust_ip_watched_dir_free(dir);
}
unsafe extern "C" fn safe_c2rust_ip_event_dispatch(
    mut dir_list: *mut GList,
    mut file_list: *mut GList,
    mut event: *mut ik_event_t,
) -> gboolean {
    let mut interesting: gboolean = FALSE;
    let mut l: *mut GList = ::core::ptr::null_mut::<GList>();
    if event.is_null() {
        return FALSE;
    }
    l = dir_list;
    while !l.is_null() {
        let mut subl: *mut GList = ::core::ptr::null_mut::<GList>();
        let mut dir: *mut ip_watched_dir_t = (*l).data as *mut ip_watched_dir_t;
        let mut current_block_14: u64;
        subl = (*dir).subs;
        while !subl.is_null() {
            let mut sub: *mut inotify_sub = (*subl).data as *mut inotify_sub;
            if !(!(*sub).filename.is_null()
                && !(*event).name.is_null()
                && strcmp((*sub).filename, (*event).name) != 0
                && ((*event).pair.is_null()
                    || (*(*event).pair).name.is_null()
                    || strcmp((*sub).filename, (*(*event).pair).name) != 0))
            {
                if !(!(*sub).filename.is_null() && (*event).name.is_null()) {
                    if (*sub).hardlinks != 0 {
                        (*event).mask &= !IP_INOTIFY_FILE_MASK as guint32;
                        if (*event).mask == 0 {
                            current_block_14 = 15240798224410183470;
                        } else {
                            current_block_14 = 12209867499936983673;
                        }
                    } else {
                        current_block_14 = 12209867499936983673;
                    }
                    match current_block_14 {
                        15240798224410183470 => {}
                        _ => {
                            interesting |= safe_c2rust_event_callback
                                .expect("non-null function pointer")(
                                event, sub, FALSE
                            );
                            if (*sub).hardlinks != 0 {
                                let mut file: *mut ip_watched_file_t =
                                    ::core::ptr::null_mut::<ip_watched_file_t>();
                                file = g_hash_table_lookup(
                                    (*dir).files_hash,
                                    (*sub).filename as gconstpointer,
                                ) as *mut ip_watched_file_t;
                                if !file.is_null() {
                                    if (*event).mask & (IN_MOVED_FROM | IN_DELETE) as guint32 != 0 {
                                        safe_c2rust_ip_watched_file_stop(file);
                                    }
                                    if (*event).mask & (IN_MOVED_TO | IN_CREATE) as guint32 != 0 {
                                        safe_c2rust_ip_watched_file_start(file);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            subl = (*subl).next;
        }
        l = (*l).next;
    }
    l = file_list;
    while !l.is_null() {
        let mut file_0: *mut ip_watched_file_t = (*l).data as *mut ip_watched_file_t;
        let mut subl_0: *mut GList = ::core::ptr::null_mut::<GList>();
        subl_0 = (*file_0).subs;
        while !subl_0.is_null() {
            let mut sub_0: *mut inotify_sub = (*subl_0).data as *mut inotify_sub;
            interesting |=
                safe_c2rust_event_callback.expect("non-null function pointer")(event, sub_0, TRUE);
            subl_0 = (*subl_0).next;
        }
        l = (*l).next;
    }
    return interesting;
}
unsafe extern "C" fn safe_c2rust_ip_event_callback(mut event: *mut ik_event_t) -> gboolean {
    let mut interesting: gboolean = FALSE;
    let mut dir_list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut file_list: *mut GList = ::core::ptr::null_mut::<GList>();
    if (*event).mask & (IN_IGNORED | IN_Q_OVERFLOW) as guint32 != 0 {
        _ik_event_free(event);
        return TRUE;
    }
    dir_list = g_hash_table_lookup(
        safe_c2rust_wd_dir_hash,
        (*event).wd as glong as gpointer as gconstpointer,
    ) as *mut GList;
    file_list = g_hash_table_lookup(
        safe_c2rust_wd_file_hash,
        (*event).wd as glong as gpointer as gconstpointer,
    ) as *mut GList;
    if (*event).mask & IP_INOTIFY_DIR_MASK as guint32 != 0 {
        interesting |= safe_c2rust_ip_event_dispatch(dir_list, file_list, event);
    }
    if !(*event).pair.is_null() && (*(*event).pair).wd != (*event).wd {
        dir_list = g_hash_table_lookup(
            safe_c2rust_wd_dir_hash,
            (*(*event).pair).wd as glong as gpointer as gconstpointer,
        ) as *mut GList;
        file_list = g_hash_table_lookup(
            safe_c2rust_wd_file_hash,
            (*(*event).pair).wd as glong as gpointer as gconstpointer,
        ) as *mut GList;
        if (*(*event).pair).mask & IP_INOTIFY_DIR_MASK as guint32 != 0 {
            interesting |= safe_c2rust_ip_event_dispatch(
                dir_list,
                file_list,
                (*event).pair as *mut ik_event_t,
            );
        }
    }
    if (*event).mask & IN_DELETE_SELF as guint32 != 0
        || (*event).mask & IN_MOVE_SELF as guint32 != 0
        || (*event).mask & IN_UNMOUNT as guint32 != 0
    {
        g_list_foreach(
            dir_list,
            Some(safe_c2rust_ip_wd_delete as unsafe extern "C" fn(gpointer, gpointer) -> ()),
            NULL_0,
        );
        safe_c2rust_ip_unmap_wd((*event).wd);
    }
    _ik_event_free(event);
    return interesting;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__ip_get_path_for_wd(
    mut wd: gint32,
) -> *const ::core::ffi::c_char {
    let mut dir_list: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut dir: *mut ip_watched_dir_t = ::core::ptr::null_mut::<ip_watched_dir_t>();
    if ({
        let mut _g_boolean_var_22: ::core::ffi::c_int = 0;
        if wd >= 0 as ::core::ffi::c_int {
            _g_boolean_var_22 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_22 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_22
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN.as_ptr(),
            b"../original/gio/inotify/inotify-path.c\0" as *const u8 as *const ::core::ffi::c_char,
            585 as ::core::ffi::c_int,
            G_STRFUNC,
            b"wd >= 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    dir_list = g_hash_table_lookup(
        safe_c2rust_wd_dir_hash,
        wd as glong as gpointer as gconstpointer,
    ) as *mut GList;
    if !dir_list.is_null() {
        dir = (*dir_list).data as *mut ip_watched_dir_t;
        if !dir.is_null() {
            return (*dir).path;
        }
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GLib-GIO\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
