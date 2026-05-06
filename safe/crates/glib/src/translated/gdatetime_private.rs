extern "C" {
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
    fn g_ptr_array_unref(array: *mut GPtrArray);
    fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_ascii_strtoull(nptr: *const gchar, endptr: *mut *mut gchar, base: guint) -> guint64;
    fn g_strndup(str: *const gchar, n: gsize) -> *mut gchar;
    fn g_atomic_ref_count_init(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_inc(arc: *mut gatomicrefcount);
    fn g_atomic_ref_count_dec(arc: *mut gatomicrefcount) -> gboolean;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
pub type gatomicrefcount = gint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GPtrArray {
    pub pdata: *mut gpointer,
    pub len: guint,
}
pub type GPtrArray = _GPtrArray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GEraDate {
    pub type_0: C2RustUnnamed,
    pub year: ::core::ffi::c_int,
    pub month: ::core::ffi::c_int,
    pub day: ::core::ffi::c_int,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const G_ERA_DATE_MINUS_INFINITY: C2RustUnnamed = 2;
pub const G_ERA_DATE_PLUS_INFINITY: C2RustUnnamed = 1;
pub const G_ERA_DATE_SET: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GEraDescriptionSegment {
    pub ref_count: gatomicrefcount,
    pub direction_multiplier: ::core::ffi::c_int,
    pub offset: guint64,
    pub start_date: GEraDate,
    pub end_date: GEraDate,
    pub era_name: *mut ::core::ffi::c_char,
    pub era_format: *mut ::core::ffi::c_char,
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXINT: ::core::ffi::c_int = INT_MAX;
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn safe_c2rust_g_steal_pointer(mut pp: gpointer) -> gpointer {
    let mut ptr: *mut gpointer = pp as *mut gpointer;
    let mut ref_0: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    ref_0 = *ptr;
    *ptr = NULL as gpointer;
    return ref_0;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_era_date_compare(
    mut date1: *const GEraDate,
    mut date2: *const GEraDate,
) -> ::core::ffi::c_int {
    if (*date1).type_0 as ::core::ffi::c_uint
        == G_ERA_DATE_SET as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*date2).type_0 as ::core::ffi::c_uint
            == G_ERA_DATE_SET as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*date1).year != (*date2).year {
            return (*date1).year - (*date2).year;
        }
        if (*date1).month != (*date2).month {
            return (*date1).month - (*date2).month;
        }
        return (*date1).day - (*date2).day;
    }
    if (*date1).type_0 as ::core::ffi::c_uint == (*date2).type_0 as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_int;
    }
    if (*date1).type_0 as ::core::ffi::c_uint
        == G_ERA_DATE_MINUS_INFINITY as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*date2).type_0 as ::core::ffi::c_uint
            == G_ERA_DATE_PLUS_INFINITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return -(1 as ::core::ffi::c_int);
    }
    if (*date1).type_0 as ::core::ffi::c_uint
        == G_ERA_DATE_PLUS_INFINITY as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*date2).type_0 as ::core::ffi::c_uint
            == G_ERA_DATE_MINUS_INFINITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 1 as ::core::ffi::c_int;
    }
    g_assertion_message_expr(
        G_LOG_DOMAIN.as_ptr(),
        b"../original/glib/gdatetime-private.c\0" as *const u8 as *const ::core::ffi::c_char,
        60 as ::core::ffi::c_int,
        G_STRFUNC,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
unsafe extern "C" fn safe_c2rust_parse_era_date(
    mut str: *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut out_date: *mut GEraDate,
) -> gboolean {
    let mut str_endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut year_multiplier: ::core::ffi::c_int = 0;
    let mut year: guint64 = 0;
    let mut month: guint64 = 0;
    let mut day: guint64 = 0;
    year_multiplier =
        if *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32 {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        };
    if *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
        || *str.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '+' as i32
    {
        str = str.offset(1);
    }
    year = g_ascii_strtoull(
        str as *const gchar,
        &raw mut str_endptr as *mut *mut gchar,
        10 as guint,
    );
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if str_endptr <= endptr {
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
            b"../original/glib/gdatetime-private.c\0" as *const u8 as *const ::core::ffi::c_char,
            77 as ::core::ffi::c_int,
            G_STRFUNC,
            b"str_endptr <= endptr\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if str_endptr == endptr
        || *str_endptr as ::core::ffi::c_int != '/' as i32
        || year > G_MAXINT as guint64
    {
        return FALSE;
    }
    str = str_endptr.offset(1 as ::core::ffi::c_int as isize);
    month = g_ascii_strtoull(
        str as *const gchar,
        &raw mut str_endptr as *mut *mut gchar,
        10 as guint,
    );
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if str_endptr <= endptr {
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
            b"../original/glib/gdatetime-private.c\0" as *const u8 as *const ::core::ffi::c_char,
            83 as ::core::ffi::c_int,
            G_STRFUNC,
            b"str_endptr <= endptr\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if str_endptr == endptr
        || *str_endptr as ::core::ffi::c_int != '/' as i32
        || month < 1 as guint64
        || month > 12 as guint64
    {
        return FALSE;
    }
    str = str_endptr.offset(1 as ::core::ffi::c_int as isize);
    day = g_ascii_strtoull(
        str as *const gchar,
        &raw mut str_endptr as *mut *mut gchar,
        10 as guint,
    );
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if str_endptr <= endptr {
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
            b"../original/glib/gdatetime-private.c\0" as *const u8 as *const ::core::ffi::c_char,
            89 as ::core::ffi::c_int,
            G_STRFUNC,
            b"str_endptr <= endptr\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if str_endptr != endptr || day < 1 as guint64 || day > 31 as guint64 {
        return FALSE;
    }
    (*out_date).type_0 = G_ERA_DATE_SET;
    (*out_date).year = (year_multiplier as guint64).wrapping_mul(year) as ::core::ffi::c_int;
    (*out_date).month = month as ::core::ffi::c_int;
    (*out_date).day = day as ::core::ffi::c_int;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_era_description_segment_ref(
    mut segment: *mut GEraDescriptionSegment,
) -> *mut GEraDescriptionSegment {
    g_atomic_ref_count_inc(&raw mut (*segment).ref_count);
    return segment;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_era_description_segment_unref(
    mut segment: *mut GEraDescriptionSegment,
) {
    if g_atomic_ref_count_dec(&raw mut (*segment).ref_count) != 0 {
        g_free((*segment).era_format as gpointer);
        g_free((*segment).era_name as gpointer);
        g_free(segment as gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust__g_era_description_parse(
    mut desc: *const ::core::ffi::c_char,
) -> *mut GPtrArray {
    let mut current_block: u64;
    let mut segments: *mut GPtrArray = g_ptr_array_new_with_free_func(::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut GEraDescriptionSegment) -> ()>,
        GDestroyNotify,
    >(Some(
        safe_c2rust__g_era_description_segment_unref
            as unsafe extern "C" fn(*mut GEraDescriptionSegment) -> (),
    )));
    let mut p: *const ::core::ffi::c_char = desc;
    loop {
        if !(*p as ::core::ffi::c_int != '\0' as i32) {
            current_block = 8845338526596852646;
            break;
        }
        let mut next_colon: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut segment: *mut GEraDescriptionSegment =
            ::core::ptr::null_mut::<GEraDescriptionSegment>();
        let mut direction: ::core::ffi::c_char = 0;
        let mut offset: guint64 = 0;
        let mut start_date: GEraDate = GEraDate {
            type_0: G_ERA_DATE_SET,
            year: 0,
            month: 0,
            day: 0,
        };
        let mut end_date: GEraDate = GEraDate {
            type_0: G_ERA_DATE_SET,
            year: 0,
            month: 0,
            day: 0,
        };
        let mut era_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut era_format: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let fresh0 = p;
        p = p.offset(1);
        direction = *fresh0;
        if direction as ::core::ffi::c_int != '+' as i32
            && direction as ::core::ffi::c_int != '-' as i32
        {
            current_block = 10473959641540818540;
            break;
        }
        let fresh1 = p;
        p = p.offset(1);
        if *fresh1 as ::core::ffi::c_int != ':' as i32 {
            current_block = 10473959641540818540;
            break;
        }
        next_colon = strchr(p, ':' as i32);
        if next_colon.is_null() {
            current_block = 10473959641540818540;
            break;
        }
        offset = g_ascii_strtoull(
            p as *const gchar,
            &raw mut endptr as *mut *mut gchar,
            10 as guint,
        );
        if endptr != next_colon {
            current_block = 10473959641540818540;
            break;
        }
        p = next_colon.offset(1 as ::core::ffi::c_int as isize);
        next_colon = strchr(p, ':' as i32);
        if next_colon.is_null() {
            current_block = 10473959641540818540;
            break;
        }
        if safe_c2rust_parse_era_date(p, next_colon, &raw mut start_date) == 0 {
            current_block = 10473959641540818540;
            break;
        }
        p = next_colon.offset(1 as ::core::ffi::c_int as isize);
        next_colon = strchr(p, ':' as i32);
        if next_colon.is_null() {
            current_block = 10473959641540818540;
            break;
        }
        if strncmp(
            p,
            b"-*\0" as *const u8 as *const ::core::ffi::c_char,
            2 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            end_date.type_0 = G_ERA_DATE_MINUS_INFINITY;
        } else if strncmp(
            p,
            b"+*\0" as *const u8 as *const ::core::ffi::c_char,
            2 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            end_date.type_0 = G_ERA_DATE_PLUS_INFINITY;
        } else if safe_c2rust_parse_era_date(p, next_colon, &raw mut end_date) == 0 {
            current_block = 10473959641540818540;
            break;
        }
        p = next_colon.offset(1 as ::core::ffi::c_int as isize);
        next_colon = strchr(p, ':' as i32);
        if next_colon.is_null() {
            current_block = 10473959641540818540;
            break;
        }
        if next_colon.offset_from(p) as ::core::ffi::c_long == 0 as ::core::ffi::c_long {
            current_block = 10473959641540818540;
            break;
        }
        era_name = g_strndup(
            p as *const gchar,
            next_colon.offset_from(p) as ::core::ffi::c_long as gsize,
        ) as *mut ::core::ffi::c_char;
        p = next_colon.offset(1 as ::core::ffi::c_int as isize);
        next_colon = strchr(p, ';' as i32);
        if next_colon.is_null() {
            next_colon = p.offset(strlen(p) as isize);
        }
        if next_colon.offset_from(p) as ::core::ffi::c_long == 0 as ::core::ffi::c_long {
            g_free(era_name as gpointer);
            current_block = 10473959641540818540;
            break;
        } else {
            era_format = g_strndup(
                p as *const gchar,
                next_colon.offset_from(p) as ::core::ffi::c_long as gsize,
            ) as *mut ::core::ffi::c_char;
            if *next_colon as ::core::ffi::c_int == ';' as i32 {
                p = next_colon.offset(1 as ::core::ffi::c_int as isize);
            } else {
                p = next_colon;
            }
            segment = ({
                let mut __n: gsize = 1 as ::core::ffi::c_int as gsize;
                let mut __s: gsize = ::core::mem::size_of::<GEraDescriptionSegment>() as gsize;
                let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
                if __s == 1 as gsize {
                    __p = g_malloc0(__n);
                } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
                    __p = g_malloc0(__n.wrapping_mul(__s));
                } else {
                    __p = g_malloc0_n(__n, __s);
                }
                __p
            }) as *mut GEraDescriptionSegment;
            g_atomic_ref_count_init(&raw mut (*segment).ref_count);
            (*segment).offset = offset;
            (*segment).start_date = start_date;
            (*segment).end_date = end_date;
            (*segment).direction_multiplier = (if safe_c2rust__g_era_date_compare(
                &raw mut (*segment).start_date,
                &raw mut (*segment).end_date,
            ) <= 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            }) * (if direction as ::core::ffi::c_int == '-' as i32
            {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            });
            (*segment).era_name = safe_c2rust_g_steal_pointer(&raw mut era_name as gpointer)
                as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            (*segment).era_format = safe_c2rust_g_steal_pointer(&raw mut era_format as gpointer)
                as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            g_ptr_array_add(
                segments,
                safe_c2rust_g_steal_pointer(&raw mut segment as gpointer)
                    as *mut GEraDescriptionSegment as gpointer,
            );
        }
    }
    match current_block {
        8845338526596852646 => {
            return safe_c2rust_g_steal_pointer(&raw mut segments as gpointer) as *mut GPtrArray;
        }
        _ => {
            g_ptr_array_unref(segments);
            return ::core::ptr::null_mut::<GPtrArray>();
        }
    };
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"_g_era_date_compare\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
