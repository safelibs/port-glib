extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_intern_static_string(string: *const gchar) -> *const gchar;
    fn g_once_init_enter_pointer(location: *mut ::core::ffi::c_void) -> gboolean;
    fn g_once_init_leave_pointer(location: *mut ::core::ffi::c_void, result: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_list_free(list: *mut GList);
    fn g_list_remove_link(list: *mut GList, llink: *mut GList) -> *mut GList;
    fn g_list_sort(list: *mut GList, compare_func: GCompareFunc) -> *mut GList;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_random_int_range(begin: gint32, end: gint32) -> gint32;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_boxed_type_register_static(
        name: *const gchar,
        boxed_copy: GBoxedCopyFunc,
        boxed_free: GBoxedFreeFunc,
    ) -> GType;
}
pub type size_t = usize;
pub type guint16 = ::core::ffi::c_ushort;
pub type gint32 = ::core::ffi::c_int;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareFunc = Option<unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
pub type GType = gsize;
pub type GBoxedCopyFunc = Option<unsafe extern "C" fn(gpointer) -> gpointer>;
pub type GBoxedFreeFunc = Option<unsafe extern "C" fn(gpointer) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSrvTarget {
    pub hostname: *mut gchar,
    pub port: guint16,
    pub priority: guint16,
    pub weight: guint16,
}
pub type GSrvTarget = _GSrvTarget;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub do_free_type: Option<unsafe extern "C" fn(*mut GSrvTarget) -> ()>,
    pub do_free_boxed: GBoxedFreeFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub do_copy_type: Option<unsafe extern "C" fn(*mut GSrvTarget) -> *mut GSrvTarget>,
    pub do_const_copy_type: Option<unsafe extern "C" fn(*const GSrvTarget) -> *mut GSrvTarget>,
    pub do_copy_boxed: GBoxedCopyFunc,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_srv_target_get_type() -> GType {
    static mut safe_c2rust_static_g_define_type_id: GType = 0 as GType;
    if ({
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id;
        } else {
        };
        (({
            let mut gapg_temp_newval: GType = 0;
            let mut gapg_temp_atomic: *mut GType = &raw mut safe_c2rust_static_g_define_type_id;
            *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
            gapg_temp_newval
        }) == 0
            && g_once_init_enter_pointer(
                &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            ) != 0) as ::core::ffi::c_int
    }) != 0
    {
        let mut g_define_type_id: GType = safe_c2rust_g_srv_target_get_type_once();
        if 0 as ::core::ffi::c_int != 0 {
            safe_c2rust_static_g_define_type_id = g_define_type_id;
        } else {
        };
        g_once_init_leave_pointer(
            &raw mut safe_c2rust_static_g_define_type_id as *mut ::core::ffi::c_void,
            g_define_type_id as gpointer,
        );
    }
    return safe_c2rust_static_g_define_type_id;
}
#[inline(never)]
unsafe extern "C" fn safe_c2rust_g_srv_target_get_type_once() -> GType {
    let mut _g_register_boxed: Option<
        unsafe extern "C" fn(*const gchar, C2RustUnnamed_0, C2RustUnnamed) -> GType,
    > = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType>,
        Option<unsafe extern "C" fn(*const gchar, C2RustUnnamed_0, C2RustUnnamed) -> GType>,
    >(Some(
        g_boxed_type_register_static
            as unsafe extern "C" fn(*const gchar, GBoxedCopyFunc, GBoxedFreeFunc) -> GType,
    ));
    let mut g_define_type_id: GType = _g_register_boxed.expect("non-null function pointer")(
        g_intern_static_string(b"GSrvTarget\0" as *const u8 as *const gchar),
        C2RustUnnamed_0 {
            do_copy_type: Some(
                safe_c2rust_g_srv_target_copy
                    as unsafe extern "C" fn(*mut GSrvTarget) -> *mut GSrvTarget,
            ),
        },
        C2RustUnnamed {
            do_free_type: Some(
                safe_c2rust_g_srv_target_free as unsafe extern "C" fn(*mut GSrvTarget) -> (),
            ),
        },
    );
    return g_define_type_id;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_srv_target_new(
    mut hostname: *const gchar,
    mut port: guint16,
    mut priority: guint16,
    mut weight: guint16,
) -> *mut GSrvTarget {
    let mut target: *mut GSrvTarget = ({
        let mut __s: gsize = ::core::mem::size_of::<GSrvTarget>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GSrvTarget;
    (*target).hostname =
        safe_c2rust_g_strdup_inline(hostname as *const ::core::ffi::c_char) as *mut gchar;
    (*target).port = port;
    (*target).priority = priority;
    (*target).weight = weight;
    return target;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_srv_target_copy(
    mut target: *mut GSrvTarget,
) -> *mut GSrvTarget {
    return safe_c2rust_g_srv_target_new(
        (*target).hostname,
        (*target).port,
        (*target).priority,
        (*target).weight,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_srv_target_free(mut target: *mut GSrvTarget) {
    g_free((*target).hostname as gpointer);
    g_slice_free1(
        ::core::mem::size_of::<GSrvTarget>() as gsize,
        target as gpointer,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_srv_target_get_hostname(
    mut target: *mut GSrvTarget,
) -> *const gchar {
    return (*target).hostname;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_srv_target_get_port(mut target: *mut GSrvTarget) -> guint16 {
    return (*target).port;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_srv_target_get_priority(
    mut target: *mut GSrvTarget,
) -> guint16 {
    return (*target).priority;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_srv_target_get_weight(
    mut target: *mut GSrvTarget,
) -> guint16 {
    return (*target).weight;
}
unsafe extern "C" fn safe_c2rust_compare_target(
    mut a: gconstpointer,
    mut b: gconstpointer,
) -> gint {
    let mut ta: *mut GSrvTarget = a as *mut GSrvTarget;
    let mut tb: *mut GSrvTarget = b as *mut GSrvTarget;
    if (*ta).priority as ::core::ffi::c_int == (*tb).priority as ::core::ffi::c_int {
        return (*ta).weight as gint - (*tb).weight as gint;
    } else {
        return (*ta).priority as gint - (*tb).priority as gint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_srv_target_list_sort(mut targets: *mut GList) -> *mut GList {
    let mut sum: gint = 0;
    let mut num: gint = 0;
    let mut val: gint = 0;
    let mut priority: gint = 0;
    let mut weight: gint = 0;
    let mut t: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut out: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut tail: *mut GList = ::core::ptr::null_mut::<GList>();
    let mut target: *mut GSrvTarget = ::core::ptr::null_mut::<GSrvTarget>();
    if targets.is_null() {
        return ::core::ptr::null_mut::<GList>();
    }
    if (*targets).next.is_null() {
        target = (*targets).data as *mut GSrvTarget;
        if strcmp(
            (*target).hostname,
            b".\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            safe_c2rust_g_srv_target_free(target);
            g_list_free(targets);
            return ::core::ptr::null_mut::<GList>();
        }
    }
    targets = g_list_sort(
        targets,
        Some(
            safe_c2rust_compare_target
                as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
        ),
    );
    tail = ::core::ptr::null_mut::<GList>();
    out = tail;
    while !targets.is_null() {
        priority = (*((*targets).data as *mut GSrvTarget)).priority as gint;
        num = 0 as ::core::ffi::c_int as gint;
        sum = num;
        t = targets;
        while !t.is_null() {
            target = (*t).data as *mut GSrvTarget;
            if (*target).priority as ::core::ffi::c_int != priority {
                break;
            }
            sum += (*target).weight as ::core::ffi::c_int;
            num += 1;
            t = (*t).next;
        }
        while num != 0 {
            val = g_random_int_range(0 as gint32, sum as gint32 + 1 as gint32) as gint;
            t = targets;
            loop {
                weight = (*((*t).data as *mut GSrvTarget)).weight as gint;
                if weight >= val {
                    break;
                }
                val -= weight;
                t = (*t).next;
            }
            targets = g_list_remove_link(targets, t);
            if out.is_null() {
                out = t;
            } else {
                (*tail).next = t;
            }
            tail = t;
            sum -= weight;
            num -= 1;
        }
    }
    return out;
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
