// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::c2rust_bitfields;
use ::core::arch::asm;
extern "C" {
    pub type _GData;
    pub type _GVariant;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static mut ffi_type_void: ffi_type;
    static mut ffi_type_uint32: ffi_type;
    static mut ffi_type_sint32: ffi_type;
    static mut ffi_type_uint64: ffi_type;
    static mut ffi_type_sint64: ffi_type;
    static mut ffi_type_float: ffi_type;
    static mut ffi_type_double: ffi_type;
    static mut ffi_type_pointer: ffi_type;
    fn ffi_prep_cif(
        cif: *mut ffi_cif,
        abi: ffi_abi,
        nargs: ::core::ffi::c_uint,
        rtype: *mut ffi_type,
        atypes: *mut *mut ffi_type,
    ) -> ffi_status;
    fn ffi_call(
        cif: *mut ffi_cif,
        fn_0: Option<unsafe extern "C" fn() -> ()>,
        rvalue: *mut ::core::ffi::c_void,
        avalue: *mut *mut ::core::ffi::c_void,
    );
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc0(n_bytes: gsize) -> gpointer;
    fn g_realloc_n(mem: gpointer, n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
    fn g_variant_unref(value: *mut GVariant);
    fn g_variant_ref_sink(value: *mut GVariant) -> *mut GVariant;
    fn g_log(log_domain: *const gchar, log_level: GLogLevelFlags, format: *const gchar, ...);
    fn g_return_if_fail_warning(
        log_domain: *const ::core::ffi::c_char,
        pretty_function: *const ::core::ffi::c_char,
        expression: *const ::core::ffi::c_char,
    );
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
    fn g_type_name(type_0: GType) -> *const gchar;
    fn g_type_interface_peek(instance_class: gpointer, iface_type: GType) -> gpointer;
    fn g_type_interface_instantiatable_prerequisite(interface_type: GType) -> GType;
    fn g_type_fundamental(type_id: GType) -> GType;
    fn g_type_test_flags(type_0: GType, flags: guint) -> gboolean;
    fn g_boxed_copy(boxed_type: GType, src_boxed: gconstpointer) -> gpointer;
    fn g_boxed_free(boxed_type: GType, boxed: gpointer);
    fn g_value_take_boxed(value: *mut GValue, v_boxed: gconstpointer);
    fn g_value_peek_pointer(value: *const GValue) -> gpointer;
    fn g_param_spec_ref(pspec: *mut GParamSpec) -> *mut GParamSpec;
    fn g_param_spec_unref(pspec: *mut GParamSpec);
    fn g_value_take_param(value: *mut GValue, param: *mut GParamSpec);
    fn g_object_ref(object: gpointer) -> gpointer;
    fn g_object_unref(object: gpointer);
    fn g_value_take_object(value: *mut GValue, v_object: gpointer);
    fn g_value_set_enum(value: *mut GValue, v_enum: gint);
    fn g_value_get_enum(value: *const GValue) -> gint;
    fn g_value_set_flags(value: *mut GValue, v_flags: guint);
    fn g_value_get_flags(value: *const GValue) -> guint;
    fn g_value_set_schar(value: *mut GValue, v_char: gint8);
    fn g_value_set_uchar(value: *mut GValue, v_uchar: guchar);
    fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    fn g_value_set_int(value: *mut GValue, v_int: gint);
    fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    fn g_value_set_long(value: *mut GValue, v_long: glong);
    fn g_value_set_ulong(value: *mut GValue, v_ulong: gulong);
    fn g_value_set_int64(value: *mut GValue, v_int64: gint64);
    fn g_value_set_uint64(value: *mut GValue, v_uint64: guint64);
    fn g_value_set_float(value: *mut GValue, v_float: gfloat);
    fn g_value_set_double(value: *mut GValue, v_double: gdouble);
    fn g_value_set_pointer(value: *mut GValue, v_pointer: gpointer);
    fn g_value_take_variant(value: *mut GValue, variant: *mut GVariant);
    fn g_value_take_string(value: *mut GValue, v_string: *mut gchar);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type uintptr_t = usize;
pub type va_list = __builtin_va_list;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const VG_USERREQ__INNER_THREADS: C2RustUnnamed = 6402;
pub const VG_USERREQ__VEX_INIT_FOR_IRI: C2RustUnnamed = 6401;
pub const VG_USERREQ__CHANGE_ERR_DISABLEMENT: C2RustUnnamed = 6145;
pub const VG_USERREQ__MAP_IP_TO_SRCLOC: C2RustUnnamed = 5889;
pub const VG_USERREQ__LOAD_PDB_DEBUGINFO: C2RustUnnamed = 5633;
pub const VG_USERREQ__STACK_CHANGE: C2RustUnnamed = 5379;
pub const VG_USERREQ__STACK_DEREGISTER: C2RustUnnamed = 5378;
pub const VG_USERREQ__STACK_REGISTER: C2RustUnnamed = 5377;
pub const VG_USERREQ__PRINTF_BACKTRACE_VALIST_BY_REF: C2RustUnnamed = 5124;
pub const VG_USERREQ__PRINTF_VALIST_BY_REF: C2RustUnnamed = 5123;
pub const VG_USERREQ__PRINTF_BACKTRACE: C2RustUnnamed = 5122;
pub const VG_USERREQ__PRINTF: C2RustUnnamed = 5121;
pub const VG_USERREQ__MEMPOOL_EXISTS: C2RustUnnamed = 4874;
pub const VG_USERREQ__MEMPOOL_CHANGE: C2RustUnnamed = 4873;
pub const VG_USERREQ__MOVE_MEMPOOL: C2RustUnnamed = 4872;
pub const VG_USERREQ__MEMPOOL_TRIM: C2RustUnnamed = 4871;
pub const VG_USERREQ__MEMPOOL_FREE: C2RustUnnamed = 4870;
pub const VG_USERREQ__MEMPOOL_ALLOC: C2RustUnnamed = 4869;
pub const VG_USERREQ__DESTROY_MEMPOOL: C2RustUnnamed = 4868;
pub const VG_USERREQ__CREATE_MEMPOOL: C2RustUnnamed = 4867;
pub const VG_USERREQ__FREELIKE_BLOCK: C2RustUnnamed = 4866;
pub const VG_USERREQ__RESIZEINPLACE_BLOCK: C2RustUnnamed = 4875;
pub const VG_USERREQ__MALLOCLIKE_BLOCK: C2RustUnnamed = 4865;
pub const VG_USERREQ__GDB_MONITOR_COMMAND: C2RustUnnamed = 4610;
pub const VG_USERREQ__COUNT_ERRORS: C2RustUnnamed = 4609;
pub const VG_USERREQ__CLIENT_CALL3: C2RustUnnamed = 4356;
pub const VG_USERREQ__CLIENT_CALL2: C2RustUnnamed = 4355;
pub const VG_USERREQ__CLIENT_CALL1: C2RustUnnamed = 4354;
pub const VG_USERREQ__CLIENT_CALL0: C2RustUnnamed = 4353;
pub const VG_USERREQ__DISCARD_TRANSLATIONS: C2RustUnnamed = 4098;
pub const VG_USERREQ__RUNNING_ON_VALGRIND: C2RustUnnamed = 4097;
pub type size_t = usize;
pub type ffi_arg = ::core::ffi::c_ulong;
pub type ffi_abi = ::core::ffi::c_uint;
pub const FFI_DEFAULT_ABI: ffi_abi = 2;
pub const FFI_LAST_ABI: ffi_abi = 5;
pub const FFI_GNUW64: ffi_abi = 4;
pub const FFI_EFI64: ffi_abi = 3;
pub const FFI_WIN64: ffi_abi = 3;
pub const FFI_UNIX64: ffi_abi = 2;
pub const FFI_FIRST_ABI: ffi_abi = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ffi_type {
    pub size: size_t,
    pub alignment: ::core::ffi::c_ushort,
    pub type_0: ::core::ffi::c_ushort,
    pub elements: *mut *mut _ffi_type,
}
pub type ffi_type = _ffi_type;
pub type ffi_status = ::core::ffi::c_uint;
pub const FFI_BAD_ARGTYPE: ffi_status = 3;
pub const FFI_BAD_ABI: ffi_status = 2;
pub const FFI_BAD_TYPEDEF: ffi_status = 1;
pub const FFI_OK: ffi_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffi_cif {
    pub abi: ffi_abi,
    pub nargs: ::core::ffi::c_uint,
    pub arg_types: *mut *mut ffi_type,
    pub rtype: *mut ffi_type,
    pub bytes: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_uint,
}
pub type gint8 = ::core::ffi::c_schar;
pub type guint8 = ::core::ffi::c_uchar;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type guintptr = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type glong = ::core::ffi::c_long;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type gulong = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
pub type gfloat = ::core::ffi::c_float;
pub type gdouble = ::core::ffi::c_double;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GData = _GData;
pub type GVariant = _GVariant;
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
pub type GType = gsize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [C2RustUnnamed_0; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub v_int: gint,
    pub v_uint: guint,
    pub v_long: glong,
    pub v_ulong: gulong,
    pub v_int64: gint64,
    pub v_uint64: guint64,
    pub v_float: gfloat,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
}
pub type GValue = _GValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeClass {
    pub g_type: GType,
}
pub type GTypeClass = _GTypeClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}
pub type GTypeInstance = _GTypeInstance;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const G_TYPE_FLAG_DEEP_DERIVABLE: C2RustUnnamed_1 = 8;
pub const G_TYPE_FLAG_DERIVABLE: C2RustUnnamed_1 = 4;
pub const G_TYPE_FLAG_INSTANTIATABLE: C2RustUnnamed_1 = 2;
pub const G_TYPE_FLAG_CLASSED: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _GClosure {
    #[bitfield(name = "ref_count", ty = "guint", bits = "0..=14")]
    #[bitfield(name = "meta_marshal_nouse", ty = "guint", bits = "15..=15")]
    #[bitfield(name = "n_guards", ty = "guint", bits = "16..=16")]
    #[bitfield(name = "n_fnotifiers", ty = "guint", bits = "17..=18")]
    #[bitfield(name = "n_inotifiers", ty = "guint", bits = "19..=26")]
    #[bitfield(name = "in_inotify", ty = "guint", bits = "27..=27")]
    #[bitfield(name = "floating", ty = "guint", bits = "28..=28")]
    #[bitfield(name = "derivative_flag", ty = "guint", bits = "29..=29")]
    #[bitfield(name = "in_marshal", ty = "guint", bits = "30..=30")]
    #[bitfield(name = "is_invalid", ty = "guint", bits = "31..=31")]
    pub ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub marshal: Option<
        unsafe extern "C" fn(
            *mut GClosure,
            *mut GValue,
            guint,
            *const GValue,
            gpointer,
            gpointer,
        ) -> (),
    >,
    pub data: gpointer,
    pub notifiers: *mut GClosureNotifyData,
}
pub type GClosureNotifyData = _GClosureNotifyData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GClosureNotifyData {
    pub data: gpointer,
    pub notify: GClosureNotify,
}
pub type GClosureNotify = Option<unsafe extern "C" fn(gpointer, *mut GClosure) -> ()>;
pub type GClosure = _GClosure;
pub type GCallback = Option<unsafe extern "C" fn() -> ()>;
pub type GClosureMarshal = Option<
    unsafe extern "C" fn(
        *mut GClosure,
        *mut GValue,
        guint,
        *const GValue,
        gpointer,
        gpointer,
    ) -> (),
>;
pub type GVaClosureMarshal = Option<
    unsafe extern "C" fn(
        *mut GClosure,
        *mut GValue,
        gpointer,
        ::core::ffi::VaList,
        gpointer,
        ::core::ffi::c_int,
        *mut GType,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GCClosure {
    pub closure: GClosure,
    pub callback: gpointer,
}
pub type GCClosure = _GCClosure;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ClosureInt {
    pub closure: GClosure,
    pub vint: gint,
}
const CLOSURE_REF_COUNT_MASK: guint = ((1 as guint) << 15 as ::core::ffi::c_int) - 1 as guint;
const CLOSURE_IN_INOTIFY_MASK: guint = (1 as guint) << 27 as ::core::ffi::c_int;
const CLOSURE_FLOATING_MASK: guint = (1 as guint) << 28 as ::core::ffi::c_int;
const CLOSURE_IN_MARSHAL_MASK: guint = (1 as guint) << 30 as ::core::ffi::c_int;
const CLOSURE_INVALID_MASK: guint = (1 as guint) << 31 as ::core::ffi::c_int;
#[inline]
unsafe fn closure_state_word(mut closure: *mut GClosure) -> guint {
    crate::translated::compat::atomic_load_seqcst(closure as *mut guint)
}
#[inline]
unsafe fn closure_ref_count_now(mut closure: *mut GClosure) -> guint {
    closure_state_word(closure) & CLOSURE_REF_COUNT_MASK
}
#[inline]
unsafe fn closure_is_floating_now(mut closure: *mut GClosure) -> guint {
    (closure_state_word(closure) & CLOSURE_FLOATING_MASK != 0) as ::core::ffi::c_int as guint
}
#[inline]
unsafe fn closure_is_invalid_now(mut closure: *mut GClosure) -> guint {
    (closure_state_word(closure) & CLOSURE_INVALID_MASK != 0) as ::core::ffi::c_int as guint
}
#[inline]
unsafe fn closure_in_marshal_now(mut closure: *mut GClosure) -> guint {
    (closure_state_word(closure) & CLOSURE_IN_MARSHAL_MASK != 0) as ::core::ffi::c_int as guint
}
#[inline]
unsafe fn closure_in_inotify_now(mut closure: *mut GClosure) -> guint {
    (closure_state_word(closure) & CLOSURE_IN_INOTIFY_MASK != 0) as ::core::ffi::c_int as guint
}
pub type GRealClosure = _GRealClosure;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GRealClosure {
    pub meta_marshal: GClosureMarshal,
    pub meta_marshal_data: gpointer,
    pub va_meta_marshal: GVaClosureMarshal,
    pub va_marshal: GVaClosureMarshal,
    pub closure: GClosure,
}
pub const FNOTIFY: C2RustUnnamed_2 = 0;
pub const POST_NOTIFY: C2RustUnnamed_2 = 3;
pub const PRE_NOTIFY: C2RustUnnamed_2 = 2;
pub const INOTIFY: C2RustUnnamed_2 = 1;
pub type GParamSpec = _GParamSpec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GParamSpec {
    pub g_type_instance: GTypeInstance,
    pub name: *const gchar,
    pub flags: GParamFlags,
    pub value_type: GType,
    pub owner_type: GType,
    pub _nick: *mut gchar,
    pub _blurb: *mut gchar,
    pub qdata: *mut GData,
    pub ref_count: guint,
    pub param_id: guint,
}
pub type GParamFlags = ::core::ffi::c_int;
pub const G_PARAM_DEPRECATED: GParamFlags = -2147483648;
pub const G_PARAM_EXPLICIT_NOTIFY: GParamFlags = 1073741824;
pub const G_PARAM_STATIC_BLURB: GParamFlags = 128;
pub const G_PARAM_STATIC_NICK: GParamFlags = 64;
pub const G_PARAM_PRIVATE: GParamFlags = 32;
pub const G_PARAM_STATIC_NAME: GParamFlags = 32;
pub const G_PARAM_LAX_VALIDATION: GParamFlags = 16;
pub const G_PARAM_CONSTRUCT_ONLY: GParamFlags = 8;
pub const G_PARAM_CONSTRUCT: GParamFlags = 4;
pub const G_PARAM_READWRITE: GParamFlags = 3;
pub const G_PARAM_WRITABLE: GParamFlags = 2;
pub const G_PARAM_READABLE: GParamFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union va_arg_storage {
    pub _gpointer: gpointer,
    pub _float: ::core::ffi::c_float,
    pub _double: ::core::ffi::c_double,
    pub _gint: gint,
    pub _guint: guint,
    pub _glong: glong,
    pub _gulong: gulong,
    pub _gint64: gint64,
    pub _guint64: guint64,
}
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
#[inline(always)]
unsafe extern "C" fn g_strdup_inline(
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
pub unsafe extern "C" fn g_closure_new_simple(
    mut sizeof_closure: guint,
    mut data: gpointer,
) -> *mut GClosure {
    let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    let mut private_size: gint = 0;
    let mut allocated: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if sizeof_closure as usize >= ::core::mem::size_of::<GClosure>() as usize {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_new_simple\0" as *const u8 as *const ::core::ffi::c_char,
            b"sizeof_closure >= sizeof (GClosure)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    private_size = (::core::mem::size_of::<GRealClosure>() as usize)
        .wrapping_sub(::core::mem::size_of::<GClosure>() as usize) as gint;
    if ({
        let mut _zzq_args: [uintptr_t; 6] = [0; 6];
        let mut _zzq_result: ::core::ffi::c_ulong = 0;
        ::core::ptr::write_volatile(
            &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            VG_USERREQ__RUNNING_ON_VALGRIND as ::core::ffi::c_int as uintptr_t,
        );
        ::core::ptr::write_volatile(
            &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            0 as ::core::ffi::c_int as uintptr_t,
        );
        ::core::ptr::write_volatile(
            &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            0 as ::core::ffi::c_int as uintptr_t,
        );
        ::core::ptr::write_volatile(
            &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            0 as ::core::ffi::c_int as uintptr_t,
        );
        ::core::ptr::write_volatile(
            &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            0 as ::core::ffi::c_int as uintptr_t,
        );
        ::core::ptr::write_volatile(
            &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            0 as ::core::ffi::c_int as uintptr_t,
        );
        let fresh4 = &mut _zzq_result;
        let fresh5;
        let fresh6 = 0 as ::core::ffi::c_int;
        asm!(
            "rolq $3,  %rdi ; rolq $13, %rdi\n", "\trolq $61, %rdi ; rolq $51, %rdi\n",
            "\txchgq %rbx,%rbx\n", inlateout("dx")
            c2rust_asm_casts::AsmCast::cast_in(fresh4, fresh6) => fresh5, inlateout("ax")
            (& raw mut _zzq_args as * mut uintptr_t).offset(0 as ::core::ffi::c_int as
            isize) as * mut uintptr_t => _, options(att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh6, fresh5);
        _zzq_result
    }) as ::core::ffi::c_uint
        != 0
    {
        private_size = (private_size as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<gpointer>() as usize as ::core::ffi::c_ulong)
            as gint as gint;
        allocated = g_malloc0(
            ((private_size as guint).wrapping_add(sizeof_closure) as gsize)
                .wrapping_add(::core::mem::size_of::<gpointer>() as gsize),
        ) as *mut gchar;
        let ref mut fresh7 = *(allocated
            .offset(private_size as isize)
            .offset(sizeof_closure as isize) as *mut gpointer);
        *fresh7 =
            allocated.offset(::core::mem::size_of::<gpointer>() as usize as isize) as gpointer;
        ({
            let mut _zzq_args: [uintptr_t; 6] = [0; 6];
            let mut _zzq_result: ::core::ffi::c_ulong = 0;
            ::core::ptr::write_volatile(
                &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                VG_USERREQ__MALLOCLIKE_BLOCK as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                allocated.offset(private_size as isize) as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                (sizeof_closure as usize).wrapping_add(::core::mem::size_of::<gpointer>() as usize)
                    as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            let fresh8 = &mut _zzq_result;
            let fresh9;
            let fresh10 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh8, fresh10) =>
                fresh9, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t).offset(0
                as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
        });
        ({
            let mut _zzq_args: [uintptr_t; 6] = [0; 6];
            let mut _zzq_result: ::core::ffi::c_ulong = 0;
            ::core::ptr::write_volatile(
                &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                VG_USERREQ__MALLOCLIKE_BLOCK as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                allocated.offset(::core::mem::size_of::<gpointer>() as usize as isize) as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                (private_size as usize).wrapping_sub(::core::mem::size_of::<gpointer>() as usize)
                    as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            let fresh11 = &mut _zzq_result;
            let fresh12;
            let fresh13 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh11, fresh13) =>
                fresh12, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t)
                .offset(0 as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh11, fresh13, fresh12);
        });
    } else {
        allocated =
            g_malloc0((private_size as guint).wrapping_add(sizeof_closure) as gsize) as *mut gchar;
    }
    closure = allocated.offset(private_size as isize) as *mut GClosure;
    let mut cunion: *mut ClosureInt = closure as *mut ClosureInt;
    let mut new_int: gint = 0;
    let mut old_int: gint = 0;
    let mut success: gint = 0;
    loop {
        let mut tmp: ClosureInt = ClosureInt {
            closure: _GClosure {
                ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                c2rust_padding: [0; 4],
                marshal: None,
                data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
            },
        };
        old_int = (*cunion).vint;
        tmp.vint = old_int;
        tmp.closure.set_ref_count(1 as guint as guint);
        new_int = tmp.vint;
        success = ({
            let mut gaicae_oldval: gint = old_int;
            if 0 as ::core::ffi::c_int != 0 {
                (*cunion).vint;
            } else {
            };
            let fresh14 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut (*cunion).vint,
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                new_int,
            );
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh14.0;
            if fresh14.1 as ::core::ffi::c_int != 0 {
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        }) as gint;
        if !(success == 0 && 0 as ::core::ffi::c_int == 0) {
            break;
        }
    }
    let mut cunion_0: *mut ClosureInt = closure as *mut ClosureInt;
    let mut new_int_0: gint = 0;
    let mut old_int_0: gint = 0;
    let mut success_0: gint = 0;
    loop {
        let mut tmp_0: ClosureInt = ClosureInt {
            closure: _GClosure {
                ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                c2rust_padding: [0; 4],
                marshal: None,
                data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
            },
        };
        old_int_0 = (*cunion_0).vint;
        tmp_0.vint = old_int_0;
        tmp_0
            .closure
            .set_floating((0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as guint as guint);
        new_int_0 = tmp_0.vint;
        success_0 = ({
            let mut gaicae_oldval: gint = old_int_0;
            if 0 as ::core::ffi::c_int != 0 {
                (*cunion_0).vint;
            } else {
            };
            let fresh15 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut (*cunion_0).vint,
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                new_int_0,
            );
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh15.0;
            if fresh15.1 as ::core::ffi::c_int != 0 {
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        }) as gint;
        if !(success_0 == 0 && 0 as ::core::ffi::c_int == 0) {
            break;
        }
    }
    (*closure).data = data;
    return closure;
}
#[inline]
unsafe extern "C" fn closure_invoke_notifiers(mut closure: *mut GClosure, mut notify_type: guint) {
    let mut ndata: *mut GClosureNotifyData = ::core::ptr::null_mut::<GClosureNotifyData>();
    let mut i: guint = 0;
    let mut offs: guint = 0;
    match notify_type {
        0 => {
            while (*closure).n_fnotifiers() != 0 {
                let mut n: guint = 0;
                let mut cunion: *mut ClosureInt = closure as *mut ClosureInt;
                let mut new_int: gint = 0;
                let mut old_int: gint = 0;
                let mut success: gint = 0;
                loop {
                    let mut tmp: ClosureInt = ClosureInt {
                        closure: _GClosure {
                            ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                            c2rust_padding: [0; 4],
                            marshal: None,
                            data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                            notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
                        },
                    };
                    old_int = (*cunion).vint;
                    tmp.vint = old_int;
                    tmp.closure.set_n_fnotifiers(
                        tmp.closure.n_fnotifiers() - 1 as ::core::ffi::c_int as guint,
                    );
                    n = tmp.closure.n_fnotifiers();
                    new_int = tmp.vint;
                    success = ({
                        let mut gaicae_oldval: gint = old_int;
                        if 0 as ::core::ffi::c_int != 0 {
                            (*cunion).vint;
                        } else {
                        };
                        let fresh30 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                            &raw mut (*cunion).vint,
                            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                            new_int,
                        );
                        *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) =
                            fresh30.0;
                        if fresh30.1 as ::core::ffi::c_int != 0 {
                            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }
                    }) as gint;
                    if !(success == 0 && 0 as ::core::ffi::c_int == 0) {
                        break;
                    }
                }
                ndata = (*closure)
                    .notifiers
                    .offset(
                        (((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                            as isize,
                    )
                    .offset(n as isize);
                (*closure).marshal =
                    ::core::mem::transmute::<GClosureNotify, GClosureMarshal>((*ndata).notify)
                        as Option<
                            unsafe extern "C" fn(
                                *mut GClosure,
                                *mut GValue,
                                guint,
                                *const GValue,
                                gpointer,
                                gpointer,
                            ) -> (),
                        >;
                (*closure).data = (*ndata).data;
                (*ndata).notify.expect("non-null function pointer")((*ndata).data, closure);
            }
            (*closure).marshal = None;
            (*closure).data = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
        }
        1 => {
            let mut cunion_0: *mut ClosureInt = closure as *mut ClosureInt;
            let mut new_int_0: gint = 0;
            let mut old_int_0: gint = 0;
            let mut success_0: gint = 0;
            loop {
                let mut tmp_0: ClosureInt = ClosureInt {
                    closure: _GClosure {
                        ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                        c2rust_padding: [0; 4],
                        marshal: None,
                        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
                    },
                };
                old_int_0 = (*cunion_0).vint;
                tmp_0.vint = old_int_0;
                tmp_0.closure.set_in_inotify(
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as guint as guint,
                );
                new_int_0 = tmp_0.vint;
                success_0 = ({
                    let mut gaicae_oldval: gint = old_int_0;
                    if 0 as ::core::ffi::c_int != 0 {
                        (*cunion_0).vint;
                    } else {
                    };
                    let fresh31 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                        &raw mut (*cunion_0).vint,
                        *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                        new_int_0,
                    );
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh31.0;
                    if fresh31.1 as ::core::ffi::c_int != 0 {
                        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }
                }) as gint;
                if !(success_0 == 0 && 0 as ::core::ffi::c_int == 0) {
                    break;
                }
            }
            while (*closure).n_inotifiers() != 0 {
                let mut n_0: guint = 0;
                let mut cunion_1: *mut ClosureInt = closure as *mut ClosureInt;
                let mut new_int_1: gint = 0;
                let mut old_int_1: gint = 0;
                let mut success_1: gint = 0;
                loop {
                    let mut tmp_1: ClosureInt = ClosureInt {
                        closure: _GClosure {
                            ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                            c2rust_padding: [0; 4],
                            marshal: None,
                            data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                            notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
                        },
                    };
                    old_int_1 = (*cunion_1).vint;
                    tmp_1.vint = old_int_1;
                    tmp_1.closure.set_n_inotifiers(
                        tmp_1.closure.n_inotifiers() - 1 as ::core::ffi::c_int as guint,
                    );
                    n_0 = tmp_1.closure.n_inotifiers();
                    new_int_1 = tmp_1.vint;
                    success_1 = ({
                        let mut gaicae_oldval: gint = old_int_1;
                        if 0 as ::core::ffi::c_int != 0 {
                            (*cunion_1).vint;
                        } else {
                        };
                        let fresh32 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                            &raw mut (*cunion_1).vint,
                            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                            new_int_1,
                        );
                        *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) =
                            fresh32.0;
                        if fresh32.1 as ::core::ffi::c_int != 0 {
                            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }
                    }) as gint;
                    if !(success_1 == 0 && 0 as ::core::ffi::c_int == 0) {
                        break;
                    }
                }
                ndata = (*closure)
                    .notifiers
                    .offset(
                        (((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                            as isize,
                    )
                    .offset((*closure).n_fnotifiers() as ::core::ffi::c_int as isize)
                    .offset(n_0 as isize);
                (*closure).marshal =
                    ::core::mem::transmute::<GClosureNotify, GClosureMarshal>((*ndata).notify)
                        as Option<
                            unsafe extern "C" fn(
                                *mut GClosure,
                                *mut GValue,
                                guint,
                                *const GValue,
                                gpointer,
                                gpointer,
                            ) -> (),
                        >;
                (*closure).data = (*ndata).data;
                (*ndata).notify.expect("non-null function pointer")((*ndata).data, closure);
            }
            (*closure).marshal = None;
            (*closure).data = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
            let mut cunion_2: *mut ClosureInt = closure as *mut ClosureInt;
            let mut new_int_2: gint = 0;
            let mut old_int_2: gint = 0;
            let mut success_2: gint = 0;
            loop {
                let mut tmp_2: ClosureInt = ClosureInt {
                    closure: _GClosure {
                        ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                        c2rust_padding: [0; 4],
                        marshal: None,
                        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
                    },
                };
                old_int_2 = (*cunion_2).vint;
                tmp_2.vint = old_int_2;
                tmp_2.closure.set_in_inotify(0 as guint as guint);
                new_int_2 = tmp_2.vint;
                success_2 = ({
                    let mut gaicae_oldval: gint = old_int_2;
                    if 0 as ::core::ffi::c_int != 0 {
                        (*cunion_2).vint;
                    } else {
                    };
                    let fresh33 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                        &raw mut (*cunion_2).vint,
                        *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                        new_int_2,
                    );
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh33.0;
                    if fresh33.1 as ::core::ffi::c_int != 0 {
                        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }
                }) as gint;
                if !(success_2 == 0 && 0 as ::core::ffi::c_int == 0) {
                    break;
                }
            }
        }
        2 => {
            i = (*closure).n_guards();
            offs = 0 as guint;
            loop {
                let fresh34 = i;
                i = i.wrapping_sub(1);
                if !(fresh34 != 0) {
                    break;
                }
                ndata = (*closure)
                    .notifiers
                    .offset(offs as isize)
                    .offset(i as isize);
                (*ndata).notify.expect("non-null function pointer")((*ndata).data, closure);
            }
        }
        3 => {
            i = (*closure).n_guards();
            offs = i;
            loop {
                let fresh35 = i;
                i = i.wrapping_sub(1);
                if !(fresh35 != 0) {
                    break;
                }
                ndata = (*closure)
                    .notifiers
                    .offset(offs as isize)
                    .offset(i as isize);
                (*ndata).notify.expect("non-null function pointer")((*ndata).data, closure);
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn g_closure_set_meta_va_marshal(
    mut closure: *mut GClosure,
    mut va_meta_marshal: GVaClosureMarshal,
) {
    let mut real_closure: *mut GRealClosure = ::core::ptr::null_mut::<GRealClosure>();
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_set_meta_va_marshal\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if va_meta_marshal.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_set_meta_va_marshal\0" as *const u8 as *const ::core::ffi::c_char,
            b"va_meta_marshal != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).is_invalid() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_set_meta_va_marshal\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->is_invalid == FALSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).in_marshal() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_set_meta_va_marshal\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->in_marshal == FALSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    real_closure = (closure as *mut guint8).offset(-(32 as ::core::ffi::c_ulong as glong) as isize)
        as gpointer as *mut GRealClosure;
    if (*real_closure).meta_marshal.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_set_meta_va_marshal\0" as *const u8 as *const ::core::ffi::c_char,
            b"real_closure->meta_marshal != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*real_closure).va_meta_marshal = va_meta_marshal;
}
#[no_mangle]
pub unsafe extern "C" fn g_closure_set_meta_marshal(
    mut closure: *mut GClosure,
    mut marshal_data: gpointer,
    mut meta_marshal: GClosureMarshal,
) {
    let mut real_closure: *mut GRealClosure = ::core::ptr::null_mut::<GRealClosure>();
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_set_meta_marshal\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if meta_marshal.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_set_meta_marshal\0" as *const u8 as *const ::core::ffi::c_char,
            b"meta_marshal != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).is_invalid() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_set_meta_marshal\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->is_invalid == FALSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).in_marshal() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_set_meta_marshal\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->in_marshal == FALSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    real_closure = (closure as *mut guint8).offset(-(32 as ::core::ffi::c_ulong as glong) as isize)
        as gpointer as *mut GRealClosure;
    if (*real_closure).meta_marshal.is_none() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_set_meta_marshal\0" as *const u8 as *const ::core::ffi::c_char,
            b"real_closure->meta_marshal == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    (*real_closure).meta_marshal = meta_marshal;
    (*real_closure).meta_marshal_data = marshal_data;
}
#[no_mangle]
pub unsafe extern "C" fn g_closure_add_marshal_guards(
    mut closure: *mut GClosure,
    mut pre_marshal_data: gpointer,
    mut pre_marshal_notify: GClosureNotify,
    mut post_marshal_data: gpointer,
    mut post_marshal_notify: GClosureNotify,
) {
    let mut i: guint = 0;
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_add_marshal_guards\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if pre_marshal_notify.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_add_marshal_guards\0" as *const u8 as *const ::core::ffi::c_char,
            b"pre_marshal_notify != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if post_marshal_notify.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_add_marshal_guards\0" as *const u8 as *const ::core::ffi::c_char,
            b"post_marshal_notify != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).is_invalid() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_add_marshal_guards\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->is_invalid == FALSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).in_marshal() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_add_marshal_guards\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->in_marshal == FALSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ((*closure).n_guards() as ::core::ffi::c_int)
        < ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_add_marshal_guards\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->n_guards < CLOSURE_MAX_N_GUARDS\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*closure).notifiers = g_realloc_n(
        (*closure).notifiers as gpointer,
        ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
            + (*closure).n_fnotifiers() as ::core::ffi::c_int
            + (*closure).n_inotifiers() as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) as gsize,
        ::core::mem::size_of::<GClosureNotifyData>() as gsize,
    ) as *mut GClosureNotifyData;
    if (*closure).n_inotifiers() != 0 {
        *(*closure).notifiers.offset(
            ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                + (*closure).n_fnotifiers() as ::core::ffi::c_int
                + (*closure).n_inotifiers() as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as isize,
        ) = *(*closure).notifiers.offset(
            ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                + (*closure).n_fnotifiers() as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int) as isize,
        );
    }
    if (*closure).n_inotifiers() as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
        *(*closure).notifiers.offset(
            ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                + (*closure).n_fnotifiers() as ::core::ffi::c_int
                + (*closure).n_inotifiers() as ::core::ffi::c_int) as isize,
        ) = *(*closure).notifiers.offset(
            ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                + (*closure).n_fnotifiers() as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as isize,
        );
    }
    if (*closure).n_fnotifiers() != 0 {
        *(*closure).notifiers.offset(
            ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                + (*closure).n_fnotifiers() as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as isize,
        ) = *(*closure).notifiers.offset(
            ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                + 0 as ::core::ffi::c_int) as isize,
        );
    }
    if (*closure).n_fnotifiers() as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
        *(*closure).notifiers.offset(
            ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                + (*closure).n_fnotifiers() as ::core::ffi::c_int) as isize,
        ) = *(*closure).notifiers.offset(
            ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                + 1 as ::core::ffi::c_int) as isize,
        );
    }
    if (*closure).n_guards() != 0 {
        *(*closure).notifiers.offset(
            ((*closure).n_guards() as ::core::ffi::c_int
                + (*closure).n_guards() as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as isize,
        ) = *(*closure).notifiers.offset((*closure).n_guards() as isize);
    }
    i = (*closure).n_guards();
    let ref mut fresh42 = (*(*closure).notifiers.offset(i as isize)).data;
    *fresh42 = pre_marshal_data;
    let ref mut fresh43 = (*(*closure).notifiers.offset(i as isize)).notify;
    *fresh43 = pre_marshal_notify;
    let ref mut fresh44 = (*(*closure)
        .notifiers
        .offset(i.wrapping_add(1 as guint) as isize))
    .data;
    *fresh44 = post_marshal_data;
    let ref mut fresh45 = (*(*closure)
        .notifiers
        .offset(i.wrapping_add(1 as guint) as isize))
    .notify;
    *fresh45 = post_marshal_notify;
    let mut cunion: *mut ClosureInt = closure as *mut ClosureInt;
    let mut new_int: gint = 0;
    let mut old_int: gint = 0;
    let mut success: gint = 0;
    loop {
        let mut tmp: ClosureInt = ClosureInt {
            closure: _GClosure {
                ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                c2rust_padding: [0; 4],
                marshal: None,
                data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
            },
        };
        old_int = (*cunion).vint;
        tmp.vint = old_int;
        tmp.closure
            .set_n_guards(tmp.closure.n_guards() + 1 as ::core::ffi::c_int as guint);
        new_int = tmp.vint;
        success = ({
            let mut gaicae_oldval: gint = old_int;
            if 0 as ::core::ffi::c_int != 0 {
                (*cunion).vint;
            } else {
            };
            let fresh46 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut (*cunion).vint,
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                new_int,
            );
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh46.0;
            if fresh46.1 as ::core::ffi::c_int != 0 {
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        }) as gint;
        if !(success == 0 && 0 as ::core::ffi::c_int == 0) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_closure_add_finalize_notifier(
    mut closure: *mut GClosure,
    mut notify_data: gpointer,
    mut notify_func: GClosureNotify,
) {
    let mut i: guint = 0;
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_add_finalize_notifier\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if notify_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_add_finalize_notifier\0" as *const u8 as *const ::core::ffi::c_char,
            b"notify_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ((*closure).n_fnotifiers() as ::core::ffi::c_int)
        < ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_add_finalize_notifier\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->n_fnotifiers < CLOSURE_MAX_N_FNOTIFIERS\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*closure).notifiers = g_realloc_n(
        (*closure).notifiers as gpointer,
        ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
            + (*closure).n_fnotifiers() as ::core::ffi::c_int
            + (*closure).n_inotifiers() as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as gsize,
        ::core::mem::size_of::<GClosureNotifyData>() as gsize,
    ) as *mut GClosureNotifyData;
    if (*closure).n_inotifiers() != 0 {
        *(*closure).notifiers.offset(
            ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                + (*closure).n_fnotifiers() as ::core::ffi::c_int
                + (*closure).n_inotifiers() as ::core::ffi::c_int) as isize,
        ) = *(*closure).notifiers.offset(
            ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                + (*closure).n_fnotifiers() as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int) as isize,
        );
    }
    i = ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
        + (*closure).n_fnotifiers() as ::core::ffi::c_int) as guint;
    let ref mut fresh1 = (*(*closure).notifiers.offset(i as isize)).data;
    *fresh1 = notify_data;
    let ref mut fresh2 = (*(*closure).notifiers.offset(i as isize)).notify;
    *fresh2 = notify_func;
    let mut cunion: *mut ClosureInt = closure as *mut ClosureInt;
    let mut new_int: gint = 0;
    let mut old_int: gint = 0;
    let mut success: gint = 0;
    loop {
        let mut tmp: ClosureInt = ClosureInt {
            closure: _GClosure {
                ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                c2rust_padding: [0; 4],
                marshal: None,
                data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
            },
        };
        old_int = (*cunion).vint;
        tmp.vint = old_int;
        tmp.closure
            .set_n_fnotifiers(tmp.closure.n_fnotifiers() + 1 as ::core::ffi::c_int as guint);
        new_int = tmp.vint;
        success = ({
            let mut gaicae_oldval: gint = old_int;
            if 0 as ::core::ffi::c_int != 0 {
                (*cunion).vint;
            } else {
            };
            let fresh3 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut (*cunion).vint,
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                new_int,
            );
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh3.0;
            if fresh3.1 as ::core::ffi::c_int != 0 {
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        }) as gint;
        if !(success == 0 && 0 as ::core::ffi::c_int == 0) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_closure_add_invalidate_notifier(
    mut closure: *mut GClosure,
    mut notify_data: gpointer,
    mut notify_func: GClosureNotify,
) {
    let mut i: guint = 0;
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_add_invalidate_notifier\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if notify_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_add_invalidate_notifier\0" as *const u8 as *const ::core::ffi::c_char,
            b"notify_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).is_invalid() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_add_invalidate_notifier\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->is_invalid == FALSE\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ((*closure).n_inotifiers() as ::core::ffi::c_int)
        < ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_add_invalidate_notifier\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->n_inotifiers < CLOSURE_MAX_N_INOTIFIERS\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    (*closure).notifiers = g_realloc_n(
        (*closure).notifiers as gpointer,
        ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
            + (*closure).n_fnotifiers() as ::core::ffi::c_int
            + (*closure).n_inotifiers() as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as gsize,
        ::core::mem::size_of::<GClosureNotifyData>() as gsize,
    ) as *mut GClosureNotifyData;
    i = ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
        + (*closure).n_fnotifiers() as ::core::ffi::c_int
        + (*closure).n_inotifiers() as ::core::ffi::c_int) as guint;
    let ref mut fresh38 = (*(*closure).notifiers.offset(i as isize)).data;
    *fresh38 = notify_data;
    let ref mut fresh39 = (*(*closure).notifiers.offset(i as isize)).notify;
    *fresh39 = notify_func;
    let mut cunion: *mut ClosureInt = closure as *mut ClosureInt;
    let mut new_int: gint = 0;
    let mut old_int: gint = 0;
    let mut success: gint = 0;
    loop {
        let mut tmp: ClosureInt = ClosureInt {
            closure: _GClosure {
                ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                c2rust_padding: [0; 4],
                marshal: None,
                data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
            },
        };
        old_int = (*cunion).vint;
        tmp.vint = old_int;
        tmp.closure
            .set_n_inotifiers(tmp.closure.n_inotifiers() + 1 as ::core::ffi::c_int as guint);
        new_int = tmp.vint;
        success = ({
            let mut gaicae_oldval: gint = old_int;
            if 0 as ::core::ffi::c_int != 0 {
                (*cunion).vint;
            } else {
            };
            let fresh40 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut (*cunion).vint,
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                new_int,
            );
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh40.0;
            if fresh40.1 as ::core::ffi::c_int != 0 {
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        }) as gint;
        if !(success == 0 && 0 as ::core::ffi::c_int == 0) {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn closure_try_remove_inotify(
    mut closure: *mut GClosure,
    mut notify_data: gpointer,
    mut notify_func: GClosureNotify,
) -> gboolean {
    let mut ndata: *mut GClosureNotifyData = ::core::ptr::null_mut::<GClosureNotifyData>();
    let mut nlast: *mut GClosureNotifyData = ::core::ptr::null_mut::<GClosureNotifyData>();
    nlast = (*closure)
        .notifiers
        .offset(
            ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                + (*closure).n_fnotifiers() as ::core::ffi::c_int
                + (*closure).n_inotifiers() as ::core::ffi::c_int) as isize,
        )
        .offset(-(1 as ::core::ffi::c_int as isize));
    ndata = nlast
        .offset(1 as ::core::ffi::c_int as isize)
        .offset(-((*closure).n_inotifiers() as ::core::ffi::c_int as isize));
    while ndata <= nlast {
        if (*ndata).notify == notify_func && (*ndata).data == notify_data {
            let mut cunion: *mut ClosureInt = closure as *mut ClosureInt;
            let mut new_int: gint = 0;
            let mut old_int: gint = 0;
            let mut success: gint = 0;
            loop {
                let mut tmp: ClosureInt = ClosureInt {
                    closure: _GClosure {
                        ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                        c2rust_padding: [0; 4],
                        marshal: None,
                        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
                    },
                };
                old_int = (*cunion).vint;
                tmp.vint = old_int;
                tmp.closure.set_n_inotifiers(
                    tmp.closure.n_inotifiers() - 1 as ::core::ffi::c_int as guint,
                );
                new_int = tmp.vint;
                success = ({
                    let mut gaicae_oldval: gint = old_int;
                    if 0 as ::core::ffi::c_int != 0 {
                        (*cunion).vint;
                    } else {
                    };
                    let fresh41 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                        &raw mut (*cunion).vint,
                        *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                        new_int,
                    );
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh41.0;
                    if fresh41.1 as ::core::ffi::c_int != 0 {
                        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }
                }) as gint;
                if !(success == 0 && 0 as ::core::ffi::c_int == 0) {
                    break;
                }
            }
            if ndata < nlast {
                *ndata = *nlast;
            }
            return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
        }
        ndata = ndata.offset(1);
    }
    return 0 as gboolean;
}
#[inline]
unsafe extern "C" fn closure_try_remove_fnotify(
    mut closure: *mut GClosure,
    mut notify_data: gpointer,
    mut notify_func: GClosureNotify,
) -> gboolean {
    let mut ndata: *mut GClosureNotifyData = ::core::ptr::null_mut::<GClosureNotifyData>();
    let mut nlast: *mut GClosureNotifyData = ::core::ptr::null_mut::<GClosureNotifyData>();
    nlast = (*closure)
        .notifiers
        .offset(
            ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                + (*closure).n_fnotifiers() as ::core::ffi::c_int
                + (*closure).n_inotifiers() as ::core::ffi::c_int) as isize,
        )
        .offset(-((*closure).n_inotifiers() as ::core::ffi::c_int as isize))
        .offset(-(1 as ::core::ffi::c_int as isize));
    ndata = nlast
        .offset(1 as ::core::ffi::c_int as isize)
        .offset(-((*closure).n_fnotifiers() as ::core::ffi::c_int as isize));
    while ndata <= nlast {
        if (*ndata).notify == notify_func && (*ndata).data == notify_data {
            let mut cunion: *mut ClosureInt = closure as *mut ClosureInt;
            let mut new_int: gint = 0;
            let mut old_int: gint = 0;
            let mut success: gint = 0;
            loop {
                let mut tmp: ClosureInt = ClosureInt {
                    closure: _GClosure {
                        ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                        c2rust_padding: [0; 4],
                        marshal: None,
                        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
                    },
                };
                old_int = (*cunion).vint;
                tmp.vint = old_int;
                tmp.closure.set_n_fnotifiers(
                    tmp.closure.n_fnotifiers() - 1 as ::core::ffi::c_int as guint,
                );
                new_int = tmp.vint;
                success = ({
                    let mut gaicae_oldval: gint = old_int;
                    if 0 as ::core::ffi::c_int != 0 {
                        (*cunion).vint;
                    } else {
                    };
                    let fresh37 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                        &raw mut (*cunion).vint,
                        *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                        new_int,
                    );
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh37.0;
                    if fresh37.1 as ::core::ffi::c_int != 0 {
                        (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }
                }) as gint;
                if !(success == 0 && 0 as ::core::ffi::c_int == 0) {
                    break;
                }
            }
            if ndata < nlast {
                *ndata = *nlast;
            }
            if (*closure).n_inotifiers() != 0 {
                *(*closure).notifiers.offset(
                    ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                        + (*closure).n_fnotifiers() as ::core::ffi::c_int)
                        as isize,
                ) = *(*closure).notifiers.offset(
                    ((((*closure).n_guards() as ::core::ffi::c_int) << 1 as ::core::ffi::c_long)
                        + (*closure).n_fnotifiers() as ::core::ffi::c_int
                        + (*closure).n_inotifiers() as ::core::ffi::c_int)
                        as isize,
                );
            }
            return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
        }
        ndata = ndata.offset(1);
    }
    return 0 as gboolean;
}
#[no_mangle]
pub unsafe extern "C" fn g_closure_ref(mut closure: *mut GClosure) -> *mut GClosure {
    let mut new_ref_count: guint = 0;
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    if closure_ref_count_now(closure) as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    if (closure_ref_count_now(closure) as ::core::ffi::c_int)
        < ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->ref_count < CLOSURE_MAX_REF_COUNT\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    let mut cunion: *mut ClosureInt = closure as *mut ClosureInt;
    let mut new_int: gint = 0;
    let mut old_int: gint = 0;
    let mut success: gint = 0;
    loop {
        let mut tmp: ClosureInt = ClosureInt {
            closure: _GClosure {
                ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                c2rust_padding: [0; 4],
                marshal: None,
                data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
            },
        };
        old_int = (*cunion).vint;
        tmp.vint = old_int;
        tmp.closure
            .set_ref_count(tmp.closure.ref_count() + 1 as ::core::ffi::c_int as guint);
        new_ref_count = tmp.closure.ref_count();
        new_int = tmp.vint;
        success = ({
            let mut gaicae_oldval: gint = old_int;
            if 0 as ::core::ffi::c_int != 0 {
                (*cunion).vint;
            } else {
            };
            let fresh18 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut (*cunion).vint,
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                new_int,
            );
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh18.0;
            if fresh18.1 as ::core::ffi::c_int != 0 {
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        }) as gint;
        if !(success == 0 && 0 as ::core::ffi::c_int == 0) {
            break;
        }
    }
    if new_ref_count > 1 as guint {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_ref\0" as *const u8 as *const ::core::ffi::c_char,
            b"new_ref_count > 1\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    return closure;
}
#[no_mangle]
pub unsafe extern "C" fn g_closure_invalidate(mut closure: *mut GClosure) {
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_invalidate\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if closure_is_invalid_now(closure) == 0 {
        let mut was_invalid: gboolean = 0;
        g_closure_ref(closure);
        let mut cunion: *mut ClosureInt = closure as *mut ClosureInt;
        let mut new_int: gint = 0;
        let mut old_int: gint = 0;
        let mut success: gint = 0;
        loop {
            let mut tmp: ClosureInt = ClosureInt {
                closure: _GClosure {
                    ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                    c2rust_padding: [0; 4],
                    marshal: None,
                    data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
                },
            };
            old_int = (*cunion).vint;
            tmp.vint = old_int;
            was_invalid = tmp.closure.is_invalid() as gboolean;
            tmp.closure.set_is_invalid(
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as guint as guint,
            );
            new_int = tmp.vint;
            success = ({
                let mut gaicae_oldval: gint = old_int;
                if 0 as ::core::ffi::c_int != 0 {
                    (*cunion).vint;
                } else {
                };
                let fresh36 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*cunion).vint,
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                    new_int,
                );
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh36.0;
                if fresh36.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) as gint;
            if !(success == 0 && 0 as ::core::ffi::c_int == 0) {
                break;
            }
        }
        if was_invalid == 0 {
            closure_invoke_notifiers(closure, INOTIFY as ::core::ffi::c_int as guint);
        }
        g_closure_unref(closure);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_closure_unref(mut closure: *mut GClosure) {
    let mut new_ref_count: guint = 0;
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_unref\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if closure_ref_count_now(closure) as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_unref\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if closure_ref_count_now(closure) as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        g_closure_invalidate(closure);
    }
    let mut cunion: *mut ClosureInt = closure as *mut ClosureInt;
    let mut new_int: gint = 0;
    let mut old_int: gint = 0;
    let mut success: gint = 0;
    loop {
        let mut tmp: ClosureInt = ClosureInt {
            closure: _GClosure {
                ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                c2rust_padding: [0; 4],
                marshal: None,
                data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
            },
        };
        old_int = (*cunion).vint;
        tmp.vint = old_int;
        tmp.closure
            .set_ref_count(tmp.closure.ref_count() - 1 as ::core::ffi::c_int as guint);
        new_ref_count = tmp.closure.ref_count();
        new_int = tmp.vint;
        success = ({
            let mut gaicae_oldval: gint = old_int;
            if 0 as ::core::ffi::c_int != 0 {
                (*cunion).vint;
            } else {
            };
            let fresh20 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut (*cunion).vint,
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                new_int,
            );
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh20.0;
            if fresh20.1 as ::core::ffi::c_int != 0 {
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        }) as gint;
        if !(success == 0 && 0 as ::core::ffi::c_int == 0) {
            break;
        }
    }
    if new_ref_count == 0 as guint {
        closure_invoke_notifiers(closure, FNOTIFY as ::core::ffi::c_int as guint);
        g_free((*closure).notifiers as gpointer);
        if ({
            let mut _zzq_args: [uintptr_t; 6] = [0; 6];
            let mut _zzq_result: ::core::ffi::c_ulong = 0;
            ::core::ptr::write_volatile(
                &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                VG_USERREQ__RUNNING_ON_VALGRIND as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            ::core::ptr::write_volatile(
                &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                0 as ::core::ffi::c_int as uintptr_t,
            );
            let fresh21 = &mut _zzq_result;
            let fresh22;
            let fresh23 = 0 as ::core::ffi::c_int;
            asm!(
                "rolq $3,  %rdi ; rolq $13, %rdi\n",
                "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh21, fresh23) =>
                fresh22, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t)
                .offset(0 as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                options(att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
            _zzq_result
        }) as ::core::ffi::c_uint
            != 0
        {
            let mut allocated: *mut gchar = ::core::ptr::null_mut::<gchar>();
            allocated = (closure as *mut guint8)
                .offset(-(32 as ::core::ffi::c_ulong as glong) as isize)
                as gpointer as *mut GRealClosure as *mut gchar;
            allocated = allocated.offset(-(::core::mem::size_of::<gpointer>() as usize as isize));
            g_free(allocated as gpointer);
            ({
                let mut _zzq_args: [uintptr_t; 6] = [0; 6];
                let mut _zzq_result: ::core::ffi::c_ulong = 0;
                ::core::ptr::write_volatile(
                    &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                    VG_USERREQ__FREELIKE_BLOCK as ::core::ffi::c_int as uintptr_t,
                );
                ::core::ptr::write_volatile(
                    &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                    allocated.offset(::core::mem::size_of::<gpointer>() as usize as isize)
                        as uintptr_t,
                );
                ::core::ptr::write_volatile(
                    &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                    0 as ::core::ffi::c_int as uintptr_t,
                );
                ::core::ptr::write_volatile(
                    &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                    0 as ::core::ffi::c_int as uintptr_t,
                );
                ::core::ptr::write_volatile(
                    &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                    0 as ::core::ffi::c_int as uintptr_t,
                );
                ::core::ptr::write_volatile(
                    &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                    0 as ::core::ffi::c_int as uintptr_t,
                );
                let fresh24 = &mut _zzq_result;
                let fresh25;
                let fresh26 = 0 as ::core::ffi::c_int;
                asm!(
                    "rolq $3,  %rdi ; rolq $13, %rdi\n",
                    "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                    inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh24, fresh26)
                    => fresh25, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t)
                    .offset(0 as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                    options(att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh24, fresh26, fresh25);
            });
            ({
                let mut _zzq_args: [uintptr_t; 6] = [0; 6];
                let mut _zzq_result: ::core::ffi::c_ulong = 0;
                ::core::ptr::write_volatile(
                    &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                    VG_USERREQ__FREELIKE_BLOCK as ::core::ffi::c_int as uintptr_t,
                );
                ::core::ptr::write_volatile(
                    &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                    closure as uintptr_t,
                );
                ::core::ptr::write_volatile(
                    &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                    0 as ::core::ffi::c_int as uintptr_t,
                );
                ::core::ptr::write_volatile(
                    &mut _zzq_args[3 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                    0 as ::core::ffi::c_int as uintptr_t,
                );
                ::core::ptr::write_volatile(
                    &mut _zzq_args[4 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                    0 as ::core::ffi::c_int as uintptr_t,
                );
                ::core::ptr::write_volatile(
                    &mut _zzq_args[5 as ::core::ffi::c_int as usize] as *mut uintptr_t,
                    0 as ::core::ffi::c_int as uintptr_t,
                );
                let fresh27 = &mut _zzq_result;
                let fresh28;
                let fresh29 = 0 as ::core::ffi::c_int;
                asm!(
                    "rolq $3,  %rdi ; rolq $13, %rdi\n",
                    "\trolq $61, %rdi ; rolq $51, %rdi\n", "\txchgq %rbx,%rbx\n",
                    inlateout("dx") c2rust_asm_casts::AsmCast::cast_in(fresh27, fresh29)
                    => fresh28, inlateout("ax") (& raw mut _zzq_args as * mut uintptr_t)
                    .offset(0 as ::core::ffi::c_int as isize) as * mut uintptr_t => _,
                    options(att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh27, fresh29, fresh28);
            });
        } else {
            g_free(
                (closure as *mut guint8).offset(-(32 as ::core::ffi::c_ulong as glong) as isize)
                    as gpointer as *mut GRealClosure as gpointer,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_closure_sink(mut closure: *mut GClosure) {
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_sink\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if closure_ref_count_now(closure) as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_sink\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure->ref_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if closure_is_floating_now(closure) != 0 {
        let mut was_floating: gboolean = 0;
        let mut cunion: *mut ClosureInt = closure as *mut ClosureInt;
        let mut new_int: gint = 0;
        let mut old_int: gint = 0;
        let mut success: gint = 0;
        loop {
            let mut tmp: ClosureInt = ClosureInt {
                closure: _GClosure {
                    ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                    c2rust_padding: [0; 4],
                    marshal: None,
                    data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
                },
            };
            old_int = (*cunion).vint;
            tmp.vint = old_int;
            was_floating = tmp.closure.floating() as gboolean;
            tmp.closure.set_floating(0 as guint as guint);
            new_int = tmp.vint;
            success = ({
                let mut gaicae_oldval: gint = old_int;
                if 0 as ::core::ffi::c_int != 0 {
                    (*cunion).vint;
                } else {
                };
                let fresh19 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*cunion).vint,
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                    new_int,
                );
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh19.0;
                if fresh19.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) as gint;
            if !(success == 0 && 0 as ::core::ffi::c_int == 0) {
                break;
            }
        }
        if was_floating != 0 {
            g_closure_unref(closure);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_closure_remove_invalidate_notifier(
    mut closure: *mut GClosure,
    mut notify_data: gpointer,
    mut notify_func: GClosureNotify,
) {
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_remove_invalidate_notifier\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if notify_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_remove_invalidate_notifier\0" as *const u8 as *const ::core::ffi::c_char,
            b"notify_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if closure_is_invalid_now(closure) as ::core::ffi::c_int != 0
        && closure_in_inotify_now(closure) as ::core::ffi::c_int != 0
        && ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >((*closure).marshal)
            == ::core::mem::transmute::<GClosureNotify, gpointer>(notify_func)
        && (*closure).data == notify_data
    {
        (*closure).marshal = None;
    } else if closure_try_remove_inotify(closure, notify_data, notify_func) == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gclosure.c:753: unable to remove uninstalled invalidation notifier: %p (%p)\0"
                as *const u8 as *const gchar,
            notify_func,
            notify_data,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_closure_remove_finalize_notifier(
    mut closure: *mut GClosure,
    mut notify_data: gpointer,
    mut notify_func: GClosureNotify,
) {
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_remove_finalize_notifier\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if notify_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_remove_finalize_notifier\0" as *const u8 as *const ::core::ffi::c_char,
            b"notify_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if closure_is_invalid_now(closure) as ::core::ffi::c_int != 0
        && closure_in_inotify_now(closure) == 0
        && ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
            >,
            gpointer,
        >((*closure).marshal)
            == ::core::mem::transmute::<GClosureNotify, gpointer>(notify_func)
        && (*closure).data == notify_data
    {
        (*closure).marshal = None;
    } else if closure_try_remove_fnotify(closure, notify_data, notify_func) == 0 {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gclosure.c:781: unable to remove uninstalled finalization notifier: %p (%p)\0"
                as *const u8 as *const gchar,
            notify_func,
            notify_data,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_closure_invoke(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
) {
    let mut real_closure: *mut GRealClosure = ::core::ptr::null_mut::<GRealClosure>();
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_invoke\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    real_closure = (closure as *mut guint8).offset(-(32 as ::core::ffi::c_ulong as glong) as isize)
        as gpointer as *mut GRealClosure;
    g_closure_ref(closure);
    if closure_is_invalid_now(closure) == 0 {
        let mut marshal: GClosureMarshal = None;
        let mut marshal_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut in_marshal: gboolean = closure_in_marshal_now(closure) as gboolean;
        if (*closure).marshal.is_some() || (*real_closure).meta_marshal.is_some() {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"g_closure_invoke\0" as *const u8 as *const ::core::ffi::c_char,
                b"closure->marshal || real_closure->meta_marshal\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut cunion: *mut ClosureInt = closure as *mut ClosureInt;
        let mut new_int: gint = 0;
        let mut old_int: gint = 0;
        let mut success: gint = 0;
        loop {
            let mut tmp: ClosureInt = ClosureInt {
                closure: _GClosure {
                    ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                    c2rust_padding: [0; 4],
                    marshal: None,
                    data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
                },
            };
            old_int = (*cunion).vint;
            tmp.vint = old_int;
            tmp.closure.set_in_marshal(
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as guint as guint,
            );
            new_int = tmp.vint;
            success = ({
                let mut gaicae_oldval: gint = old_int;
                if 0 as ::core::ffi::c_int != 0 {
                    (*cunion).vint;
                } else {
                };
                let fresh47 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*cunion).vint,
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                    new_int,
                );
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh47.0;
                if fresh47.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) as gint;
            if !(success == 0 && 0 as ::core::ffi::c_int == 0) {
                break;
            }
        }
        if (*real_closure).meta_marshal.is_some() {
            marshal_data = (*real_closure).meta_marshal_data;
            marshal = (*real_closure).meta_marshal;
        } else {
            marshal_data = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
            marshal = (*closure).marshal as GClosureMarshal;
        }
        if in_marshal == 0 {
            closure_invoke_notifiers(closure, PRE_NOTIFY as ::core::ffi::c_int as guint);
        }
        marshal.expect("non-null function pointer")(
            closure,
            return_value,
            n_param_values,
            param_values,
            invocation_hint,
            marshal_data,
        );
        if in_marshal == 0 {
            closure_invoke_notifiers(closure, POST_NOTIFY as ::core::ffi::c_int as guint);
        }
        let mut cunion_0: *mut ClosureInt = closure as *mut ClosureInt;
        let mut new_int_0: gint = 0;
        let mut old_int_0: gint = 0;
        let mut success_0: gint = 0;
        loop {
            let mut tmp_0: ClosureInt = ClosureInt {
                closure: _GClosure {
                    ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                    c2rust_padding: [0; 4],
                    marshal: None,
                    data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
                },
            };
            old_int_0 = (*cunion_0).vint;
            tmp_0.vint = old_int_0;
            tmp_0.closure.set_in_marshal(in_marshal as guint as guint);
            new_int_0 = tmp_0.vint;
            success_0 = ({
                let mut gaicae_oldval: gint = old_int_0;
                if 0 as ::core::ffi::c_int != 0 {
                    (*cunion_0).vint;
                } else {
                };
                let fresh48 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*cunion_0).vint,
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                    new_int_0,
                );
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh48.0;
                if fresh48.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) as gint;
            if !(success_0 == 0 && 0 as ::core::ffi::c_int == 0) {
                break;
            }
        }
    }
    g_closure_unref(closure);
}
#[no_mangle]
pub unsafe extern "C" fn _g_closure_supports_invoke_va(mut closure: *mut GClosure) -> gboolean {
    let mut real_closure: *mut GRealClosure = ::core::ptr::null_mut::<GRealClosure>();
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"_g_closure_supports_invoke_va\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return 0 as gboolean;
    }
    real_closure = (closure as *mut guint8).offset(-(32 as ::core::ffi::c_ulong as glong) as isize)
        as gpointer as *mut GRealClosure;
    return ((*real_closure).va_marshal.is_some()
        && ((*real_closure).meta_marshal.is_none() || (*real_closure).va_meta_marshal.is_some()))
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _g_closure_invoke_va(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut real_closure: *mut GRealClosure = ::core::ptr::null_mut::<GRealClosure>();
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"_g_closure_invoke_va\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    real_closure = (closure as *mut guint8).offset(-(32 as ::core::ffi::c_ulong as glong) as isize)
        as gpointer as *mut GRealClosure;
    g_closure_ref(closure);
    if closure_is_invalid_now(closure) == 0 {
        let mut marshal: GVaClosureMarshal = None;
        let mut marshal_data: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut in_marshal: gboolean = closure_in_marshal_now(closure) as gboolean;
        if (*closure).marshal.is_some() || (*real_closure).meta_marshal.is_some() {
        } else {
            g_return_if_fail_warning(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"_g_closure_invoke_va\0" as *const u8 as *const ::core::ffi::c_char,
                b"closure->marshal || real_closure->meta_marshal\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        let mut cunion: *mut ClosureInt = closure as *mut ClosureInt;
        let mut new_int: gint = 0;
        let mut old_int: gint = 0;
        let mut success: gint = 0;
        loop {
            let mut tmp: ClosureInt = ClosureInt {
                closure: _GClosure {
                    ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                    c2rust_padding: [0; 4],
                    marshal: None,
                    data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
                },
            };
            old_int = (*cunion).vint;
            tmp.vint = old_int;
            tmp.closure.set_in_marshal(
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as guint as guint,
            );
            new_int = tmp.vint;
            success = ({
                let mut gaicae_oldval: gint = old_int;
                if 0 as ::core::ffi::c_int != 0 {
                    (*cunion).vint;
                } else {
                };
                let fresh71 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*cunion).vint,
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                    new_int,
                );
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh71.0;
                if fresh71.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) as gint;
            if !(success == 0 && 0 as ::core::ffi::c_int == 0) {
                break;
            }
        }
        if (*real_closure).va_meta_marshal.is_some() {
            marshal_data = (*real_closure).meta_marshal_data;
            marshal = (*real_closure).va_meta_marshal;
        } else {
            marshal_data = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
            marshal = (*real_closure).va_marshal;
        }
        if in_marshal == 0 {
            closure_invoke_notifiers(closure, PRE_NOTIFY as ::core::ffi::c_int as guint);
        }
        marshal.expect("non-null function pointer")(
            closure,
            return_value,
            instance,
            args,
            marshal_data,
            n_params,
            param_types,
        );
        if in_marshal == 0 {
            closure_invoke_notifiers(closure, POST_NOTIFY as ::core::ffi::c_int as guint);
        }
        let mut cunion_0: *mut ClosureInt = closure as *mut ClosureInt;
        let mut new_int_0: gint = 0;
        let mut old_int_0: gint = 0;
        let mut success_0: gint = 0;
        loop {
            let mut tmp_0: ClosureInt = ClosureInt {
                closure: _GClosure {
                    ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                    c2rust_padding: [0; 4],
                    marshal: None,
                    data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
                },
            };
            old_int_0 = (*cunion_0).vint;
            tmp_0.vint = old_int_0;
            tmp_0.closure.set_in_marshal(in_marshal as guint as guint);
            new_int_0 = tmp_0.vint;
            success_0 = ({
                let mut gaicae_oldval: gint = old_int_0;
                if 0 as ::core::ffi::c_int != 0 {
                    (*cunion_0).vint;
                } else {
                };
                let fresh72 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                    &raw mut (*cunion_0).vint,
                    *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                    new_int_0,
                );
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh72.0;
                if fresh72.1 as ::core::ffi::c_int != 0 {
                    (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            }) as gint;
            if !(success_0 == 0 && 0 as ::core::ffi::c_int == 0) {
                break;
            }
        }
    }
    g_closure_unref(closure);
}
#[no_mangle]
pub unsafe extern "C" fn g_closure_set_marshal(
    mut closure: *mut GClosure,
    mut marshal: GClosureMarshal,
) {
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_set_marshal\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if marshal.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_closure_set_marshal\0" as *const u8 as *const ::core::ffi::c_char,
            b"marshal != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*closure).marshal.is_some() && (*closure).marshal != marshal {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"attempt to override closure->marshal (%p) with new marshal (%p)\0" as *const u8
                as *const gchar,
            (*closure).marshal,
            marshal,
        );
    } else {
        (*closure).marshal = marshal
            as Option<
                unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
            >;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _g_closure_set_va_marshal(
    mut closure: *mut GClosure,
    mut marshal: GVaClosureMarshal,
) {
    let mut real_closure: *mut GRealClosure = ::core::ptr::null_mut::<GRealClosure>();
    if !closure.is_null() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"_g_closure_set_va_marshal\0" as *const u8 as *const ::core::ffi::c_char,
            b"closure != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if marshal.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"_g_closure_set_va_marshal\0" as *const u8 as *const ::core::ffi::c_char,
            b"marshal != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    real_closure = (closure as *mut guint8).offset(-(32 as ::core::ffi::c_ulong as glong) as isize)
        as gpointer as *mut GRealClosure;
    if (*real_closure).va_marshal.is_some() && (*real_closure).va_marshal != marshal {
        g_log(
            b"GLib-GObject\0" as *const u8 as *const gchar,
            G_LOG_LEVEL_CRITICAL,
            b"attempt to override closure->va_marshal (%p) with new marshal (%p)\0" as *const u8
                as *const gchar,
            (*real_closure).va_marshal,
            marshal,
        );
    } else {
        (*real_closure).va_marshal = marshal;
    };
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_new(
    mut callback_func: GCallback,
    mut user_data: gpointer,
    mut destroy_data: GClosureNotify,
) -> *mut GClosure {
    let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    if callback_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_new\0" as *const u8 as *const ::core::ffi::c_char,
            b"callback_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    closure = g_closure_new_simple(::core::mem::size_of::<GCClosure>() as guint, user_data);
    if destroy_data.is_some() {
        g_closure_add_finalize_notifier(closure, user_data, destroy_data);
    }
    let ref mut fresh0 = (*(closure as *mut GCClosure)).callback;
    *fresh0 = ::core::mem::transmute::<GCallback, gpointer>(callback_func);
    return closure;
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_new_swap(
    mut callback_func: GCallback,
    mut user_data: gpointer,
    mut destroy_data: GClosureNotify,
) -> *mut GClosure {
    let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    if callback_func.is_some() {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_cclosure_new_swap\0" as *const u8 as *const ::core::ffi::c_char,
            b"callback_func != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    closure = g_closure_new_simple(::core::mem::size_of::<GCClosure>() as guint, user_data);
    if destroy_data.is_some() {
        g_closure_add_finalize_notifier(closure, user_data, destroy_data);
    }
    let ref mut fresh16 = (*(closure as *mut GCClosure)).callback;
    *fresh16 = ::core::mem::transmute::<GCallback, gpointer>(callback_func);
    let mut cunion: *mut ClosureInt = closure as *mut ClosureInt;
    let mut new_int: gint = 0;
    let mut old_int: gint = 0;
    let mut success: gint = 0;
    loop {
        let mut tmp: ClosureInt = ClosureInt {
            closure: _GClosure {
                ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [0; 4],
                c2rust_padding: [0; 4],
                marshal: None,
                data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                notifiers: ::core::ptr::null_mut::<GClosureNotifyData>(),
            },
        };
        old_int = (*cunion).vint;
        tmp.vint = old_int;
        tmp.closure.set_derivative_flag(
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as guint as guint,
        );
        new_int = tmp.vint;
        success = ({
            let mut gaicae_oldval: gint = old_int;
            if 0 as ::core::ffi::c_int != 0 {
                (*cunion).vint;
            } else {
            };
            let fresh17 = crate::translated::compat::atomic_cxchg_seqcst_seqcst(
                &raw mut (*cunion).vint,
                *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint),
                new_int,
            );
            *(&raw mut gaicae_oldval as *mut ::core::ffi::c_void as *mut gint) = fresh17.0;
            if fresh17.1 as ::core::ffi::c_int != 0 {
                (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }
        }) as gint;
        if !(success == 0 && 0 as ::core::ffi::c_int == 0) {
            break;
        }
    }
    return closure;
}
unsafe extern "C" fn g_type_class_meta_marshal(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut class: *mut GTypeClass = ::core::ptr::null_mut::<GTypeClass>();
    let mut callback: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut offset: guint = marshal_data as gulong as guint;
    class = (*(g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
        as *mut GTypeInstance))
        .g_class;
    callback =
        *((class as *mut guint8).offset(offset as glong as isize) as gpointer as *mut gpointer);
    if !callback.is_null() {
        (*closure).marshal.expect("non-null function pointer")(
            closure,
            return_value,
            n_param_values,
            param_values,
            invocation_hint,
            callback,
        );
    }
}
unsafe extern "C" fn g_type_class_meta_marshalv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut real_closure: *mut GRealClosure = ::core::ptr::null_mut::<GRealClosure>();
    let mut class: *mut GTypeClass = ::core::ptr::null_mut::<GTypeClass>();
    let mut callback: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut offset: guint = marshal_data as gulong as guint;
    real_closure = (closure as *mut guint8).offset(-(32 as ::core::ffi::c_ulong as glong) as isize)
        as gpointer as *mut GRealClosure;
    class = (*(instance as *mut GTypeInstance)).g_class;
    callback =
        *((class as *mut guint8).offset(offset as glong as isize) as gpointer as *mut gpointer);
    if !callback.is_null() {
        (*real_closure)
            .va_marshal
            .expect("non-null function pointer")(
            closure,
            return_value,
            instance,
            args,
            callback,
            n_params,
            param_types,
        );
    }
}
unsafe extern "C" fn g_type_iface_meta_marshal(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut class: *mut GTypeClass = ::core::ptr::null_mut::<GTypeClass>();
    let mut callback: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut itype: GType = (*closure).data as GType;
    let mut offset: guint = marshal_data as gulong as guint;
    class = g_type_interface_peek(
        (*(g_value_peek_pointer(param_values.offset(0 as ::core::ffi::c_int as isize))
            as *mut GTypeInstance))
            .g_class as gpointer,
        itype,
    ) as *mut GTypeClass;
    callback =
        *((class as *mut guint8).offset(offset as glong as isize) as gpointer as *mut gpointer);
    if !callback.is_null() {
        (*closure).marshal.expect("non-null function pointer")(
            closure,
            return_value,
            n_param_values,
            param_values,
            invocation_hint,
            callback,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn _g_closure_is_void(
    mut closure: *mut GClosure,
    mut instance: gpointer,
) -> gboolean {
    let mut real_closure: *mut GRealClosure = ::core::ptr::null_mut::<GRealClosure>();
    let mut class: *mut GTypeClass = ::core::ptr::null_mut::<GTypeClass>();
    let mut callback: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut itype: GType = 0;
    let mut offset: guint = 0;
    if (*closure).is_invalid() != 0 {
        return (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    }
    real_closure = (closure as *mut guint8).offset(-(32 as ::core::ffi::c_ulong as glong) as isize)
        as gpointer as *mut GRealClosure;
    if (*real_closure).meta_marshal
        == Some(
            g_type_iface_meta_marshal
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        )
    {
        itype = (*closure).data as GType;
        offset = (*real_closure).meta_marshal_data as gulong as guint;
        class = g_type_interface_peek(
            (*(instance as *mut GTypeInstance)).g_class as gpointer,
            itype,
        ) as *mut GTypeClass;
        callback =
            *((class as *mut guint8).offset(offset as glong as isize) as gpointer as *mut gpointer);
        return (callback == ::core::ptr::null_mut::<::core::ffi::c_void>()) as ::core::ffi::c_int;
    } else if (*real_closure).meta_marshal
        == Some(
            g_type_class_meta_marshal
                as unsafe extern "C" fn(
                    *mut GClosure,
                    *mut GValue,
                    guint,
                    *const GValue,
                    gpointer,
                    gpointer,
                ) -> (),
        )
    {
        offset = (*real_closure).meta_marshal_data as gulong as guint;
        class = (*(instance as *mut GTypeInstance)).g_class;
        callback =
            *((class as *mut guint8).offset(offset as glong as isize) as gpointer as *mut gpointer);
        return (callback == ::core::ptr::null_mut::<::core::ffi::c_void>()) as ::core::ffi::c_int;
    }
    return 0 as gboolean;
}
unsafe extern "C" fn g_type_iface_meta_marshalv(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut real_closure: *mut GRealClosure = ::core::ptr::null_mut::<GRealClosure>();
    let mut class: *mut GTypeClass = ::core::ptr::null_mut::<GTypeClass>();
    let mut callback: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut itype: GType = (*closure).data as GType;
    let mut offset: guint = marshal_data as gulong as guint;
    real_closure = (closure as *mut guint8).offset(-(32 as ::core::ffi::c_ulong as glong) as isize)
        as gpointer as *mut GRealClosure;
    class = g_type_interface_peek(
        (*(instance as *mut GTypeInstance)).g_class as gpointer,
        itype,
    ) as *mut GTypeClass;
    callback =
        *((class as *mut guint8).offset(offset as glong as isize) as gpointer as *mut gpointer);
    if !callback.is_null() {
        (*real_closure)
            .va_marshal
            .expect("non-null function pointer")(
            closure,
            return_value,
            instance,
            args,
            callback,
            n_params,
            param_types,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_signal_type_cclosure_new(
    mut itype: GType,
    mut struct_offset: guint,
) -> *mut GClosure {
    let mut closure: *mut GClosure = ::core::ptr::null_mut::<GClosure>();
    if g_type_test_flags(itype, G_TYPE_FLAG_CLASSED as ::core::ffi::c_int as guint) != 0
        || g_type_fundamental(itype)
            == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_type_cclosure_new\0" as *const u8 as *const ::core::ffi::c_char,
            b"G_TYPE_IS_CLASSED (itype) || G_TYPE_IS_INTERFACE (itype)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    if struct_offset as usize >= ::core::mem::size_of::<GTypeClass>() as usize {
    } else {
        g_return_if_fail_warning(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"g_signal_type_cclosure_new\0" as *const u8 as *const ::core::ffi::c_char,
            b"struct_offset >= sizeof (GTypeClass)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GClosure>();
    }
    closure = g_closure_new_simple(
        ::core::mem::size_of::<GClosure>() as guint,
        itype as gpointer,
    );
    if g_type_fundamental(itype) == ((2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
    {
        g_closure_set_meta_marshal(
            closure,
            struct_offset as gulong as gpointer,
            Some(
                g_type_iface_meta_marshal
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ),
        );
        g_closure_set_meta_va_marshal(
            closure,
            Some(
                g_type_iface_meta_marshalv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ),
        );
    } else {
        g_closure_set_meta_marshal(
            closure,
            struct_offset as gulong as gpointer,
            Some(
                g_type_class_meta_marshal
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        guint,
                        *const GValue,
                        gpointer,
                        gpointer,
                    ) -> (),
            ),
        );
        g_closure_set_meta_va_marshal(
            closure,
            Some(
                g_type_class_meta_marshalv
                    as unsafe extern "C" fn(
                        *mut GClosure,
                        *mut GValue,
                        gpointer,
                        ::core::ffi::VaList,
                        gpointer,
                        ::core::ffi::c_int,
                        *mut GType,
                    ) -> (),
            ),
        );
    }
    return closure;
}
unsafe extern "C" fn value_to_ffi_type(
    mut gvalue: *const GValue,
    mut value: *mut gpointer,
    mut enum_tmpval: *mut gint,
    mut tmpval_used: *mut gboolean,
) -> *mut ffi_type {
    let mut rettype: *mut ffi_type = ::core::ptr::null_mut::<ffi_type>();
    let mut type_0: GType = g_type_fundamental((*(gvalue as *mut GValue)).g_type);
    if type_0 != ((0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gclosure.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1201 as ::core::ffi::c_int,
            b"value_to_ffi_type\0" as *const u8 as *const ::core::ffi::c_char,
            b"type != G_TYPE_INVALID\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !enum_tmpval.is_null() {
        if !tmpval_used.is_null() {
        } else {
            g_assertion_message_expr(
                b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gclosure.c\0"
                    as *const u8 as *const ::core::ffi::c_char,
                1205 as ::core::ffi::c_int,
                b"value_to_ffi_type\0" as *const u8 as *const ::core::ffi::c_char,
                b"tmpval_used != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        *tmpval_used = 0 as ::core::ffi::c_int as gboolean;
    }
    match type_0 {
        20 | 12 | 24 => {
            rettype = &raw mut ffi_type_sint32;
            *value = &raw const (*(&raw const (*gvalue).data as *const C2RustUnnamed_0)
                .offset(0 as ::core::ffi::c_int as isize))
            .v_int as gpointer;
        }
        48 => {
            if !enum_tmpval.is_null() {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gclosure.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1224 as ::core::ffi::c_int,
                    b"value_to_ffi_type\0" as *const u8 as *const ::core::ffi::c_char,
                    b"enum_tmpval != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            rettype = &raw mut ffi_type_sint32;
            *enum_tmpval = g_value_get_enum(gvalue);
            *value = enum_tmpval as gpointer;
            *tmpval_used = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        }
        52 => {
            if !enum_tmpval.is_null() {
            } else {
                g_assertion_message_expr(
                    b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gclosure.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1231 as ::core::ffi::c_int,
                    b"value_to_ffi_type\0" as *const u8 as *const ::core::ffi::c_char,
                    b"enum_tmpval != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            rettype = &raw mut ffi_type_uint32;
            *enum_tmpval = g_value_get_flags(gvalue) as gint;
            *value = enum_tmpval as gpointer;
            *tmpval_used = (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as gboolean;
        }
        16 | 28 => {
            rettype = &raw mut ffi_type_uint32;
            *value = &raw const (*(&raw const (*gvalue).data as *const C2RustUnnamed_0)
                .offset(0 as ::core::ffi::c_int as isize))
            .v_uint as gpointer;
        }
        64 | 80 | 72 | 76 | 68 | 8 | 84 => {
            rettype = &raw mut ffi_type_pointer;
            *value = &raw const (*(&raw const (*gvalue).data as *const C2RustUnnamed_0)
                .offset(0 as ::core::ffi::c_int as isize))
            .v_pointer as gpointer;
        }
        56 => {
            rettype = &raw mut ffi_type_float;
            *value = &raw const (*(&raw const (*gvalue).data as *const C2RustUnnamed_0)
                .offset(0 as ::core::ffi::c_int as isize))
            .v_float as gpointer;
        }
        60 => {
            rettype = &raw mut ffi_type_double;
            *value = &raw const (*(&raw const (*gvalue).data as *const C2RustUnnamed_0)
                .offset(0 as ::core::ffi::c_int as isize))
            .v_double as gpointer;
        }
        32 => {
            rettype = &raw mut ffi_type_sint64;
            *value = &raw const (*(&raw const (*gvalue).data as *const C2RustUnnamed_0)
                .offset(0 as ::core::ffi::c_int as isize))
            .v_long as gpointer;
        }
        36 => {
            rettype = &raw mut ffi_type_uint64;
            *value = &raw const (*(&raw const (*gvalue).data as *const C2RustUnnamed_0)
                .offset(0 as ::core::ffi::c_int as isize))
            .v_ulong as gpointer;
        }
        40 => {
            rettype = &raw mut ffi_type_sint64;
            *value = &raw const (*(&raw const (*gvalue).data as *const C2RustUnnamed_0)
                .offset(0 as ::core::ffi::c_int as isize))
            .v_int64 as gpointer;
        }
        44 => {
            rettype = &raw mut ffi_type_uint64;
            *value = &raw const (*(&raw const (*gvalue).data as *const C2RustUnnamed_0)
                .offset(0 as ::core::ffi::c_int as isize))
            .v_uint64 as gpointer;
        }
        _ => {
            rettype = &raw mut ffi_type_pointer;
            *value = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"value_to_ffi_type: Unsupported fundamental type: %s\0" as *const u8
                    as *const gchar,
                g_type_name(type_0),
            );
        }
    }
    return rettype;
}
unsafe extern "C" fn value_from_ffi_type(mut gvalue: *mut GValue, mut value: *mut gpointer) {
    let mut int_val: *mut ffi_arg = value as *mut ffi_arg;
    let mut type_0: GType = 0;
    type_0 = (*gvalue).g_type;
    let mut current_block_22: u64;
    loop {
        match g_type_fundamental(type_0) {
            24 => {
                g_value_set_int(gvalue, *int_val as gint);
                current_block_22 = 11913429853522160501;
                break;
            }
            56 => {
                g_value_set_float(gvalue, *(value as *mut gfloat));
                current_block_22 = 11913429853522160501;
                break;
            }
            60 => {
                g_value_set_double(gvalue, *(value as *mut gdouble));
                current_block_22 = 11913429853522160501;
                break;
            }
            20 => {
                g_value_set_boolean(gvalue, *int_val as gboolean);
                current_block_22 = 11913429853522160501;
                break;
            }
            64 => {
                g_value_take_string(gvalue, *(value as *mut *mut gchar));
                current_block_22 = 11913429853522160501;
                break;
            }
            12 => {
                g_value_set_schar(gvalue, *int_val as gint8);
                current_block_22 = 11913429853522160501;
                break;
            }
            16 => {
                g_value_set_uchar(gvalue, *int_val as guchar);
                current_block_22 = 11913429853522160501;
                break;
            }
            28 => {
                g_value_set_uint(gvalue, *int_val as guint);
                current_block_22 = 11913429853522160501;
                break;
            }
            68 => {
                g_value_set_pointer(gvalue, *value);
                current_block_22 = 11913429853522160501;
                break;
            }
            32 => {
                g_value_set_long(gvalue, *int_val as glong);
                current_block_22 = 11913429853522160501;
                break;
            }
            36 => {
                g_value_set_ulong(gvalue, *int_val);
                current_block_22 = 11913429853522160501;
                break;
            }
            40 => {
                g_value_set_int64(gvalue, *int_val as gint64);
                current_block_22 = 11913429853522160501;
                break;
            }
            44 => {
                g_value_set_uint64(gvalue, *int_val);
                current_block_22 = 11913429853522160501;
                break;
            }
            72 => {
                g_value_take_boxed(gvalue, *value as gconstpointer);
                current_block_22 = 11913429853522160501;
                break;
            }
            48 => {
                g_value_set_enum(gvalue, *int_val as gint);
                current_block_22 = 11913429853522160501;
                break;
            }
            52 => {
                g_value_set_flags(gvalue, *int_val as guint);
                current_block_22 = 11913429853522160501;
                break;
            }
            76 => {
                g_value_take_param(gvalue, *value as *mut GParamSpec);
                current_block_22 = 11913429853522160501;
                break;
            }
            80 => {
                g_value_take_object(gvalue, *value);
                current_block_22 = 11913429853522160501;
                break;
            }
            84 => {
                g_value_take_variant(gvalue, *value as *mut GVariant);
                current_block_22 = 11913429853522160501;
                break;
            }
            8 => {
                type_0 = g_type_interface_instantiatable_prerequisite(type_0);
                if !(type_0 != 0) {
                    current_block_22 = 17496712291145032193;
                    break;
                }
            }
            _ => {
                current_block_22 = 17496712291145032193;
                break;
            }
        }
    }
    match current_block_22 {
        17496712291145032193 => {
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"value_from_ffi_type: Unsupported fundamental type %s for type %s\0" as *const u8
                    as *const gchar,
                g_type_name(g_type_fundamental((*gvalue).g_type)),
                g_type_name((*gvalue).g_type),
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn va_to_ffi_type(
    mut gtype: GType,
    mut va: *mut ::core::ffi::VaList<'_>,
    mut storage: *mut va_arg_storage,
) -> *mut ffi_type {
    let mut rettype: *mut ffi_type = ::core::ptr::null_mut::<ffi_type>();
    let mut type_0: GType = g_type_fundamental(gtype);
    if type_0 != ((0 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gclosure.c\0" as *const u8
                as *const ::core::ffi::c_char,
            1384 as ::core::ffi::c_int,
            b"va_to_ffi_type\0" as *const u8 as *const ::core::ffi::c_char,
            b"type != G_TYPE_INVALID\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    match type_0 {
        20 | 12 | 24 | 48 => {
            rettype = &raw mut ffi_type_sint32;
            (*storage)._gint = (*va).arg::<gint>();
        }
        16 | 28 | 52 => {
            rettype = &raw mut ffi_type_uint32;
            (*storage)._guint = (*va).arg::<guint>();
        }
        64 | 80 | 72 | 76 | 68 | 8 | 84 => {
            rettype = &raw mut ffi_type_pointer;
            (*storage)._gpointer = (*va).arg::<gpointer>();
        }
        56 => {
            rettype = &raw mut ffi_type_float;
            (*storage)._float = (*va).arg::<::core::ffi::c_double>() as ::core::ffi::c_float;
        }
        60 => {
            rettype = &raw mut ffi_type_double;
            (*storage)._double = (*va).arg::<::core::ffi::c_double>();
        }
        32 => {
            rettype = &raw mut ffi_type_sint64;
            (*storage)._glong = (*va).arg::<glong>();
        }
        36 => {
            rettype = &raw mut ffi_type_uint64;
            (*storage)._gulong = (*va).arg::<gulong>();
        }
        40 => {
            rettype = &raw mut ffi_type_sint64;
            (*storage)._gint64 = (*va).arg::<gint64>();
        }
        44 => {
            rettype = &raw mut ffi_type_uint64;
            (*storage)._guint64 = (*va).arg::<guint64>();
        }
        _ => {
            rettype = &raw mut ffi_type_pointer;
            (*storage)._guint64 = 0 as guint64;
            g_log(
                b"GLib-GObject\0" as *const u8 as *const gchar,
                G_LOG_LEVEL_CRITICAL,
                b"va_to_ffi_type: Unsupported fundamental type: %s\0" as *const u8 as *const gchar,
                g_type_name(type_0),
            );
        }
    }
    return rettype;
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_generic(
    mut closure: *mut GClosure,
    mut return_gvalue: *mut GValue,
    mut n_param_values: guint,
    mut param_values: *const GValue,
    mut invocation_hint: gpointer,
    mut marshal_data: gpointer,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut rtype: *mut ffi_type = ::core::ptr::null_mut::<ffi_type>();
    let mut rvalue: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut n_args: ::core::ffi::c_int = 0;
    let mut atypes: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut args: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut i: ::core::ffi::c_int = 0;
    let mut cif: ffi_cif = ffi_cif {
        abi: 0 as ffi_abi,
        nargs: 0,
        arg_types: ::core::ptr::null_mut::<*mut ffi_type>(),
        rtype: ::core::ptr::null_mut::<ffi_type>(),
        bytes: 0,
        flags: 0,
    };
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut enum_tmpval: *mut gint = ::core::ptr::null_mut::<gint>();
    let mut tmpval_used: gboolean = 0 as gboolean;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        ::core::mem::size_of::<gint>() as usize,
    ));
    enum_tmpval = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut gint;
    if !return_gvalue.is_null() && (*return_gvalue).g_type != 0 {
        rtype = value_to_ffi_type(
            return_gvalue,
            &raw mut rvalue,
            enum_tmpval,
            &raw mut tmpval_used,
        );
    } else {
        rtype = &raw mut ffi_type_void;
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (if (*rtype).size > ::core::mem::size_of::<ffi_arg>() as usize {
            (*rtype).size
        } else {
            ::core::mem::size_of::<ffi_arg>() as size_t
        }) as usize,
    ));
    rvalue = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void;
    n_args = n_param_values.wrapping_add(1 as guint) as ::core::ffi::c_int;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (::core::mem::size_of::<*mut ffi_type>() as usize).wrapping_mul(n_args as usize) as usize,
    ));
    atypes = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ffi_type;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (::core::mem::size_of::<gpointer>() as usize).wrapping_mul(n_args as usize) as usize,
    ));
    args = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_void;
    if tmpval_used != 0 {
        alloca_allocations.push(::std::vec::from_elem(
            0,
            ::core::mem::size_of::<gint>() as usize,
        ));
        enum_tmpval = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut gint;
    }
    if (*closure).derivative_flag() != 0 {
        let ref mut fresh49 = *atypes.offset((n_args - 1 as ::core::ffi::c_int) as isize);
        *fresh49 = value_to_ffi_type(
            param_values.offset(0 as ::core::ffi::c_int as isize),
            args.offset((n_args - 1 as ::core::ffi::c_int) as isize) as *mut gpointer,
            enum_tmpval,
            &raw mut tmpval_used,
        );
        let ref mut fresh50 = *atypes.offset(0 as ::core::ffi::c_int as isize);
        *fresh50 = &raw mut ffi_type_pointer;
        let ref mut fresh51 = *args.offset(0 as ::core::ffi::c_int as isize);
        *fresh51 = &raw mut (*closure).data as *mut ::core::ffi::c_void;
    } else {
        let ref mut fresh52 = *atypes.offset(0 as ::core::ffi::c_int as isize);
        *fresh52 = value_to_ffi_type(
            param_values.offset(0 as ::core::ffi::c_int as isize),
            args.offset(0 as ::core::ffi::c_int as isize) as *mut gpointer,
            enum_tmpval,
            &raw mut tmpval_used,
        );
        let ref mut fresh53 = *atypes.offset((n_args - 1 as ::core::ffi::c_int) as isize);
        *fresh53 = &raw mut ffi_type_pointer;
        let ref mut fresh54 = *args.offset((n_args - 1 as ::core::ffi::c_int) as isize);
        *fresh54 = &raw mut (*closure).data as *mut ::core::ffi::c_void;
    }
    i = 1 as ::core::ffi::c_int;
    while i < n_args - 1 as ::core::ffi::c_int {
        if tmpval_used != 0 {
            alloca_allocations.push(::std::vec::from_elem(
                0,
                ::core::mem::size_of::<gint>() as usize,
            ));
            enum_tmpval = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut gint;
        }
        let ref mut fresh55 = *atypes.offset(i as isize);
        *fresh55 = value_to_ffi_type(
            param_values.offset(i as isize),
            args.offset(i as isize) as *mut gpointer,
            enum_tmpval,
            &raw mut tmpval_used,
        );
        i += 1;
    }
    if ffi_prep_cif(
        &raw mut cif,
        FFI_DEFAULT_ABI,
        n_args as ::core::ffi::c_uint,
        rtype,
        atypes,
    ) as ::core::ffi::c_uint
        != FFI_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    ffi_call(
        &raw mut cif,
        ::core::mem::transmute::<*mut ::core::ffi::c_void, Option<unsafe extern "C" fn() -> ()>>(
            if !marshal_data.is_null() {
                marshal_data as *mut ::core::ffi::c_void
            } else {
                (*cc).callback as *mut ::core::ffi::c_void
            },
        ),
        rvalue,
        args,
    );
    if !return_gvalue.is_null() && (*return_gvalue).g_type != 0 {
        value_from_ffi_type(return_gvalue, rvalue as *mut gpointer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn g_cclosure_marshal_generic_va(
    mut closure: *mut GClosure,
    mut return_value: *mut GValue,
    mut instance: gpointer,
    mut args_list: ::core::ffi::VaList,
    mut marshal_data: gpointer,
    mut n_params: ::core::ffi::c_int,
    mut param_types: *mut GType,
) {
    let mut alloca_allocations: Vec<Vec<u8>> = Vec::new();
    let mut rtype: *mut ffi_type = ::core::ptr::null_mut::<ffi_type>();
    let mut rvalue: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut n_args: ::core::ffi::c_int = 0;
    let mut atypes: *mut *mut ffi_type = ::core::ptr::null_mut::<*mut ffi_type>();
    let mut args: *mut *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut storage: *mut va_arg_storage = ::core::ptr::null_mut::<va_arg_storage>();
    let mut i: ::core::ffi::c_int = 0;
    let mut cif: ffi_cif = ffi_cif {
        abi: 0 as ffi_abi,
        nargs: 0,
        arg_types: ::core::ptr::null_mut::<*mut ffi_type>(),
        rtype: ::core::ptr::null_mut::<ffi_type>(),
        bytes: 0,
        flags: 0,
    };
    let mut cc: *mut GCClosure = closure as *mut GCClosure;
    let mut enum_tmpval: *mut gint = ::core::ptr::null_mut::<gint>();
    let mut tmpval_used: gboolean = 0 as gboolean;
    let mut args_copy: ::core::ffi::VaList<'_>;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        ::core::mem::size_of::<gint>() as usize,
    ));
    enum_tmpval = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut gint;
    if !return_value.is_null() && (*return_value).g_type != 0 {
        rtype = value_to_ffi_type(
            return_value,
            &raw mut rvalue,
            enum_tmpval,
            &raw mut tmpval_used,
        );
    } else {
        rtype = &raw mut ffi_type_void;
    }
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (if (*rtype).size > ::core::mem::size_of::<ffi_arg>() as usize {
            (*rtype).size
        } else {
            ::core::mem::size_of::<ffi_arg>() as size_t
        }) as usize,
    ));
    rvalue = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut ::core::ffi::c_void;
    n_args = n_params + 2 as ::core::ffi::c_int;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (::core::mem::size_of::<*mut ffi_type>() as usize).wrapping_mul(n_args as usize) as usize,
    ));
    atypes = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ffi_type;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (::core::mem::size_of::<gpointer>() as usize).wrapping_mul(n_args as usize) as usize,
    ));
    args = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut *mut ::core::ffi::c_void;
    alloca_allocations.push(::std::vec::from_elem(
        0,
        (::core::mem::size_of::<va_arg_storage>() as usize).wrapping_mul(n_params as usize)
            as usize,
    ));
    storage = alloca_allocations.last_mut().unwrap().as_mut_ptr() as *mut va_arg_storage;
    if (*closure).derivative_flag() != 0 {
        let ref mut fresh56 = *atypes.offset((n_args - 1 as ::core::ffi::c_int) as isize);
        *fresh56 = &raw mut ffi_type_pointer;
        let ref mut fresh57 = *args.offset((n_args - 1 as ::core::ffi::c_int) as isize);
        *fresh57 = &raw mut instance as *mut ::core::ffi::c_void;
        let ref mut fresh58 = *atypes.offset(0 as ::core::ffi::c_int as isize);
        *fresh58 = &raw mut ffi_type_pointer;
        let ref mut fresh59 = *args.offset(0 as ::core::ffi::c_int as isize);
        *fresh59 = &raw mut (*closure).data as *mut ::core::ffi::c_void;
    } else {
        let ref mut fresh60 = *atypes.offset(0 as ::core::ffi::c_int as isize);
        *fresh60 = &raw mut ffi_type_pointer;
        let ref mut fresh61 = *args.offset(0 as ::core::ffi::c_int as isize);
        *fresh61 = &raw mut instance as *mut ::core::ffi::c_void;
        let ref mut fresh62 = *atypes.offset((n_args - 1 as ::core::ffi::c_int) as isize);
        *fresh62 = &raw mut ffi_type_pointer;
        let ref mut fresh63 = *args.offset((n_args - 1 as ::core::ffi::c_int) as isize);
        *fresh63 = &raw mut (*closure).data as *mut ::core::ffi::c_void;
    }
    args_copy = args_list.clone();
    i = 0 as ::core::ffi::c_int;
    while i < n_params {
        let mut type_0: GType = *param_types.offset(i as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType);
        let mut fundamental: GType = g_type_fundamental(type_0);
        let ref mut fresh64 = *atypes.offset((i + 1 as ::core::ffi::c_int) as isize);
        *fresh64 = va_to_ffi_type(
            type_0,
            &raw mut args_copy,
            storage.offset(i as isize) as *mut va_arg_storage,
        );
        let ref mut fresh65 = *args.offset((i + 1 as ::core::ffi::c_int) as isize);
        *fresh65 = storage.offset(i as isize) as *mut va_arg_storage as *mut ::core::ffi::c_void;
        if *param_types.offset(i as isize)
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType
            == 0 as GType
        {
            if fundamental == ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
                && !(*storage.offset(i as isize))._gpointer.is_null()
            {
                let ref mut fresh66 = (*storage.offset(i as isize))._gpointer;
                *fresh66 = g_strdup_inline(
                    (*storage.offset(i as isize))._gpointer as *const ::core::ffi::c_char,
                ) as gpointer;
            } else if fundamental
                == ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
                && !(*storage.offset(i as isize))._gpointer.is_null()
            {
                let ref mut fresh67 = (*storage.offset(i as isize))._gpointer;
                *fresh67 =
                    g_param_spec_ref((*storage.offset(i as isize))._gpointer as *mut GParamSpec)
                        as gpointer;
            } else if fundamental
                == ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
                && !(*storage.offset(i as isize))._gpointer.is_null()
            {
                let ref mut fresh68 = (*storage.offset(i as isize))._gpointer;
                *fresh68 = g_boxed_copy(
                    type_0,
                    (*storage.offset(i as isize))._gpointer as gconstpointer,
                );
            } else if fundamental
                == ((21 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
                && !(*storage.offset(i as isize))._gpointer.is_null()
            {
                let ref mut fresh69 = (*storage.offset(i as isize))._gpointer;
                *fresh69 =
                    g_variant_ref_sink((*storage.offset(i as isize))._gpointer as *mut GVariant)
                        as gpointer;
            }
        }
        if fundamental == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            && !(*storage.offset(i as isize))._gpointer.is_null()
        {
            let ref mut fresh70 = (*storage.offset(i as isize))._gpointer;
            *fresh70 = g_object_ref((*storage.offset(i as isize))._gpointer) as gpointer;
        }
        i += 1;
    }
    if ffi_prep_cif(
        &raw mut cif,
        FFI_DEFAULT_ABI,
        n_args as ::core::ffi::c_uint,
        rtype,
        atypes,
    ) as ::core::ffi::c_uint
        != FFI_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    ffi_call(
        &raw mut cif,
        ::core::mem::transmute::<*mut ::core::ffi::c_void, Option<unsafe extern "C" fn() -> ()>>(
            if !marshal_data.is_null() {
                marshal_data as *mut ::core::ffi::c_void
            } else {
                (*cc).callback as *mut ::core::ffi::c_void
            },
        ),
        rvalue,
        args,
    );
    i = 0 as ::core::ffi::c_int;
    while i < n_params {
        let mut type_1: GType = *param_types.offset(i as isize)
            & !(((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType);
        let mut fundamental_0: GType = g_type_fundamental(type_1);
        if *param_types.offset(i as isize)
            & ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as GType
            == 0 as GType
        {
            if fundamental_0 == ((16 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
                && !(*storage.offset(i as isize))._gpointer.is_null()
            {
                g_free((*storage.offset(i as isize))._gpointer);
            } else if fundamental_0
                == ((19 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
                && !(*storage.offset(i as isize))._gpointer.is_null()
            {
                g_param_spec_unref((*storage.offset(i as isize))._gpointer as *mut GParamSpec);
            } else if fundamental_0
                == ((18 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
                && !(*storage.offset(i as isize))._gpointer.is_null()
            {
                g_boxed_free(type_1, (*storage.offset(i as isize))._gpointer);
            } else if fundamental_0
                == ((21 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
                && !(*storage.offset(i as isize))._gpointer.is_null()
            {
                g_variant_unref((*storage.offset(i as isize))._gpointer as *mut GVariant);
            }
        }
        if fundamental_0 == ((20 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as GType
            && !(*storage.offset(i as isize))._gpointer.is_null()
        {
            g_object_unref((*storage.offset(i as isize))._gpointer);
        }
        i += 1;
    }
    if !return_value.is_null() && (*return_value).g_type != 0 {
        value_from_ffi_type(return_value, rvalue as *mut gpointer);
    }
}
