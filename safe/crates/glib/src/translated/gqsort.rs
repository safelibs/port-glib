extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GCompareDataFunc =
    Option<unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msort_param {
    pub s: size_t,
    pub var: size_t,
    pub cmp: GCompareDataFunc,
    pub arg: *mut ::core::ffi::c_void,
    pub t: *mut ::core::ffi::c_char,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn safe_c2rust_msort_with_tmp(
    mut p: *const msort_param,
    mut b: *mut ::core::ffi::c_void,
    mut n: size_t,
) {
    let mut b1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut b2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n1: size_t = 0;
    let mut n2: size_t = 0;
    let mut tmp: *mut ::core::ffi::c_char = (*p).t;
    let s: size_t = (*p).s;
    let mut cmp: GCompareDataFunc = (*p).cmp;
    let mut arg: *mut ::core::ffi::c_void = (*p).arg;
    if n <= 1 as size_t {
        return;
    }
    n1 = n.wrapping_div(2 as size_t);
    n2 = n.wrapping_sub(n1);
    b1 = b as *mut ::core::ffi::c_char;
    b2 = (b as *mut ::core::ffi::c_char).offset(n1.wrapping_mul((*p).s) as isize);
    safe_c2rust_msort_with_tmp(p, b1 as *mut ::core::ffi::c_void, n1);
    safe_c2rust_msort_with_tmp(p, b2 as *mut ::core::ffi::c_void, n2);
    match (*p).var {
        0 => {
            while n1 > 0 as size_t && n2 > 0 as size_t {
                if Some(cmp.expect("non-null function pointer")).expect("non-null function pointer")(
                    b1 as gconstpointer,
                    b2 as gconstpointer,
                    arg as gpointer,
                ) <= 0 as ::core::ffi::c_int
                {
                    *(tmp as *mut guint32) = *(b1 as *mut guint32);
                    b1 = b1.offset(::core::mem::size_of::<guint32>() as usize as isize);
                    n1 = n1.wrapping_sub(1);
                } else {
                    *(tmp as *mut guint32) = *(b2 as *mut guint32);
                    b2 = b2.offset(::core::mem::size_of::<guint32>() as usize as isize);
                    n2 = n2.wrapping_sub(1);
                }
                tmp = tmp.offset(::core::mem::size_of::<guint32>() as usize as isize);
            }
        }
        1 => {
            while n1 > 0 as size_t && n2 > 0 as size_t {
                if Some(cmp.expect("non-null function pointer")).expect("non-null function pointer")(
                    b1 as gconstpointer,
                    b2 as gconstpointer,
                    arg as gpointer,
                ) <= 0 as ::core::ffi::c_int
                {
                    *(tmp as *mut guint64) = *(b1 as *mut guint64);
                    b1 = b1.offset(::core::mem::size_of::<guint64>() as usize as isize);
                    n1 = n1.wrapping_sub(1);
                } else {
                    *(tmp as *mut guint64) = *(b2 as *mut guint64);
                    b2 = b2.offset(::core::mem::size_of::<guint64>() as usize as isize);
                    n2 = n2.wrapping_sub(1);
                }
                tmp = tmp.offset(::core::mem::size_of::<guint64>() as usize as isize);
            }
        }
        2 => {
            while n1 > 0 as size_t && n2 > 0 as size_t {
                let mut tmpl: *mut guintptr = tmp as *mut guintptr;
                let mut bl: *mut guintptr = ::core::ptr::null_mut::<guintptr>();
                tmp = tmp.offset(s as isize);
                if Some(cmp.expect("non-null function pointer")).expect("non-null function pointer")(
                    b1 as gconstpointer,
                    b2 as gconstpointer,
                    arg as gpointer,
                ) <= 0 as ::core::ffi::c_int
                {
                    bl = b1 as *mut guintptr;
                    b1 = b1.offset(s as isize);
                    n1 = n1.wrapping_sub(1);
                } else {
                    bl = b2 as *mut guintptr;
                    b2 = b2.offset(s as isize);
                    n2 = n2.wrapping_sub(1);
                }
                while tmpl < tmp as *mut guintptr {
                    let fresh3 = bl;
                    bl = bl.offset(1);
                    let fresh4 = tmpl;
                    tmpl = tmpl.offset(1);
                    *fresh4 = *fresh3;
                }
            }
        }
        3 => {
            while n1 > 0 as size_t && n2 > 0 as size_t {
                if Some(cmp.expect("non-null function pointer")).expect("non-null function pointer")(
                    *(b1 as *mut *const ::core::ffi::c_void),
                    *(b2 as *mut *const ::core::ffi::c_void),
                    arg as gpointer,
                ) <= 0 as ::core::ffi::c_int
                {
                    let ref mut fresh5 = *(tmp as *mut *mut ::core::ffi::c_void);
                    *fresh5 = *(b1 as *mut *mut ::core::ffi::c_void);
                    b1 = b1.offset(
                        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize as isize
                    );
                    n1 = n1.wrapping_sub(1);
                } else {
                    let ref mut fresh6 = *(tmp as *mut *mut ::core::ffi::c_void);
                    *fresh6 = *(b2 as *mut *mut ::core::ffi::c_void);
                    b2 = b2.offset(
                        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize as isize
                    );
                    n2 = n2.wrapping_sub(1);
                }
                tmp = tmp
                    .offset(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize as isize);
            }
        }
        _ => {
            while n1 > 0 as size_t && n2 > 0 as size_t {
                if Some(cmp.expect("non-null function pointer")).expect("non-null function pointer")(
                    b1 as gconstpointer,
                    b2 as gconstpointer,
                    arg as gpointer,
                ) <= 0 as ::core::ffi::c_int
                {
                    memcpy(
                        tmp as *mut ::core::ffi::c_void,
                        b1 as *const ::core::ffi::c_void,
                        s,
                    );
                    tmp = tmp.offset(s as isize);
                    b1 = b1.offset(s as isize);
                    n1 = n1.wrapping_sub(1);
                } else {
                    memcpy(
                        tmp as *mut ::core::ffi::c_void,
                        b2 as *const ::core::ffi::c_void,
                        s,
                    );
                    tmp = tmp.offset(s as isize);
                    b2 = b2.offset(s as isize);
                    n2 = n2.wrapping_sub(1);
                }
            }
        }
    }
    if n1 > 0 as size_t {
        memcpy(
            tmp as *mut ::core::ffi::c_void,
            b1 as *const ::core::ffi::c_void,
            n1.wrapping_mul(s),
        );
    }
    memcpy(
        b,
        (*p).t as *const ::core::ffi::c_void,
        n.wrapping_sub(n2).wrapping_mul(s),
    );
}
unsafe extern "C" fn safe_c2rust_msort_r(
    mut b: *mut ::core::ffi::c_void,
    mut n: size_t,
    mut s: size_t,
    mut cmp: GCompareDataFunc,
    mut arg: *mut ::core::ffi::c_void,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut size: size_t = n.wrapping_mul(s);
    let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: msort_param = msort_param {
        s: 0,
        var: 0,
        cmp: None,
        arg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        t: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    if s > 32 as size_t {
        size = (2 as size_t)
            .wrapping_mul(n)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
            .wrapping_add(s);
    }
    if size < 1024 as size_t {
        alloca_allocations.push(::std::vec::from_elem(0, size as usize));
        p.t = alloca_allocations.last_mut().unwrap().as_mut_ptr().cast() as *mut ::core::ffi::c_char;
    } else {
        tmp = g_malloc(size as gsize) as *mut ::core::ffi::c_char;
        p.t = tmp;
    }
    p.s = s;
    p.var = 4 as size_t;
    p.cmp = cmp;
    p.arg = arg;
    if s > 32 as size_t {
        let mut ip: *mut ::core::ffi::c_char = b as *mut ::core::ffi::c_char;
        let mut tp: *mut *mut ::core::ffi::c_void = p.t.offset(
            n.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t) as isize,
        ) as *mut *mut ::core::ffi::c_void;
        let mut t: *mut *mut ::core::ffi::c_void = tp;
        let mut tmp_storage: *mut ::core::ffi::c_void =
            tp.offset(n as isize) as *mut ::core::ffi::c_void;
        let mut kp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut i: size_t = 0;
        while (t as *mut ::core::ffi::c_void) < tmp_storage {
            let fresh0 = t;
            t = t.offset(1);
            *fresh0 = ip as *mut ::core::ffi::c_void;
            ip = ip.offset(s as isize);
        }
        p.s = ::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize as size_t;
        p.var = 3 as size_t;
        safe_c2rust_msort_with_tmp(
            &raw mut p,
            p.t.offset(
                n.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                    as isize,
            ) as *mut ::core::ffi::c_void,
            n,
        );
        i = 0 as size_t;
        ip = b as *mut ::core::ffi::c_char;
        while i < n {
            kp = *tp.offset(i as isize) as *mut ::core::ffi::c_char;
            if kp != ip {
                let mut j: size_t = i;
                let mut jp: *mut ::core::ffi::c_char = ip;
                memcpy(tmp_storage, ip as *const ::core::ffi::c_void, s);
                loop {
                    let mut k: size_t = (kp.offset_from(b as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long as size_t)
                        .wrapping_div(s);
                    let ref mut fresh1 = *tp.offset(j as isize);
                    *fresh1 = jp as *mut ::core::ffi::c_void;
                    memcpy(
                        jp as *mut ::core::ffi::c_void,
                        kp as *const ::core::ffi::c_void,
                        s,
                    );
                    j = k;
                    jp = kp;
                    kp = *tp.offset(k as isize) as *mut ::core::ffi::c_char;
                    if !(kp != ip) {
                        break;
                    }
                }
                let ref mut fresh2 = *tp.offset(j as isize);
                *fresh2 = jp as *mut ::core::ffi::c_void;
                memcpy(jp as *mut ::core::ffi::c_void, tmp_storage, s);
            }
            i = i.wrapping_add(1);
            ip = ip.offset(s as isize);
        }
    } else {
        if s & (::core::mem::size_of::<guint32>() as size_t).wrapping_sub(1 as size_t)
            == 0 as size_t
            && (b as guintptr).wrapping_rem(4 as ::core::ffi::c_ulong as glong as gsize)
                == 0 as gsize
        {
            if s == ::core::mem::size_of::<guint32>() as usize {
                p.var = 0 as size_t;
            } else if s == ::core::mem::size_of::<guint64>() as usize
                && (b as guintptr).wrapping_rem(8 as ::core::ffi::c_ulong as glong as gsize)
                    == 0 as gsize
            {
                p.var = 1 as size_t;
            } else if s
                & (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                    .wrapping_sub(1 as size_t)
                == 0 as size_t
                && (b as guintptr).wrapping_rem(8 as ::core::ffi::c_ulong as glong as gsize)
                    == 0 as gsize
            {
                p.var = 2 as size_t;
            }
        }
        safe_c2rust_msort_with_tmp(&raw mut p, b, n);
    }
    g_free(tmp as gpointer);
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_qsort_with_data(
    mut pbase: gconstpointer,
    mut total_elems: gint,
    mut size: gsize,
    mut compare_func: GCompareDataFunc,
    mut user_data: gpointer,
) {
    safe_c2rust_msort_r(
        pbase as *mut ::core::ffi::c_void,
        total_elems as size_t,
        size as size_t,
        compare_func,
        user_data as *mut ::core::ffi::c_void,
    );
}
