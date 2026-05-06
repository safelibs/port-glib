// SAFETY: Mechanically translated from the checked-in upstream GObject sources; unsafe is retained at ABI-preserving FFI and memory-layout boundaries.
use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::core::arch::asm;
extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn g_mutex_lock(mutex: *mut GMutex);
    fn g_mutex_unlock(mutex: *mut GMutex);
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_assertion_message_expr(
        domain: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        func: *const ::core::ffi::c_char,
        expr: *const ::core::ffi::c_char,
    ) -> !;
}
pub type uintptr_t = usize;
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
pub type guint8 = ::core::ffi::c_uchar;
pub type gsize = ::core::ffi::c_ulong;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2],
}
pub type GMutex = _GMutex;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _GAtomicArrayMetadata {
    pub size: gsize,
    pub _alignment_padding: gpointer,
}
pub type GAtomicArrayMetadata = _GAtomicArrayMetadata;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GAtomicArray {
    pub data: gpointer,
}
pub type GAtomicArray = _GAtomicArray;
pub type FreeListNode = _FreeListNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FreeListNode {
    pub next: *mut FreeListNode,
}
static mut g__array_lock: GMutex = _GMutex {
    p: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
};
static mut freelist: *mut FreeListNode = ::core::ptr::null::<FreeListNode>() as *mut FreeListNode;
unsafe extern "C" fn freelist_alloc(mut size: gsize, mut reuse: gboolean) -> gpointer {
    let mut mem: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut free: *mut FreeListNode = ::core::ptr::null_mut::<FreeListNode>();
    let mut prev: *mut *mut FreeListNode = ::core::ptr::null_mut::<*mut FreeListNode>();
    let mut real_size: gsize = 0;
    if reuse != 0 {
        free = freelist;
        prev = &raw mut freelist;
        while !free.is_null() {
            if (*(free as *mut GAtomicArrayMetadata).offset(-(1 as ::core::ffi::c_int as isize)))
                .size
                == size
            {
                *prev = (*free).next;
                return free as gpointer;
            }
            prev = &raw mut (*free).next;
            free = (*free).next;
        }
    }
    real_size = (::core::mem::size_of::<GAtomicArrayMetadata>() as usize).wrapping_add(
        (if size as usize > ::core::mem::size_of::<FreeListNode>() as usize {
            size as usize
        } else {
            ::core::mem::size_of::<FreeListNode>() as usize
        }),
    ) as gsize;
    mem = g_slice_alloc(real_size);
    mem = (mem as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<GAtomicArrayMetadata>() as usize as isize)
        as gpointer;
    (*(mem as *mut GAtomicArrayMetadata).offset(-(1 as ::core::ffi::c_int as isize))).size = size;
    ({
        let mut _zzq_args: [uintptr_t; 6] = [0; 6];
        let mut _zzq_result: ::core::ffi::c_ulong = 0;
        ::core::ptr::write_volatile(
            &mut _zzq_args[0 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            VG_USERREQ__MALLOCLIKE_BLOCK as ::core::ffi::c_int as uintptr_t,
        );
        ::core::ptr::write_volatile(
            &mut _zzq_args[1 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            mem as uintptr_t,
        );
        ::core::ptr::write_volatile(
            &mut _zzq_args[2 as ::core::ffi::c_int as usize] as *mut uintptr_t,
            (real_size as usize)
                .wrapping_sub(::core::mem::size_of::<GAtomicArrayMetadata>() as usize)
                as uintptr_t,
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
        let fresh0 = &mut _zzq_result;
        let fresh1;
        let fresh2 = 0 as ::core::ffi::c_int;
        asm!(
            "rolq $3,  %rdi ; rolq $13, %rdi\n", "\trolq $61, %rdi ; rolq $51, %rdi\n",
            "\txchgq %rbx,%rbx\n", inlateout("dx")
            c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1, inlateout("ax")
            (& raw mut _zzq_args as * mut uintptr_t).offset(0 as ::core::ffi::c_int as
            isize) as * mut uintptr_t => _, options(att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    });
    return mem;
}
unsafe extern "C" fn freelist_free(mut mem: gpointer) {
    let mut free: *mut FreeListNode = ::core::ptr::null_mut::<FreeListNode>();
    free = mem as *mut FreeListNode;
    (*free).next = freelist;
    freelist = free;
}
#[no_mangle]
pub unsafe extern "C" fn _g_atomic_array_init(mut array: *mut GAtomicArray) {
    (*array).data = ::core::ptr::null_mut::<::core::ffi::c_void>() as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn _g_atomic_array_copy(
    mut array: *mut GAtomicArray,
    mut header_size: gsize,
    mut additional_element_size: gsize,
) -> gpointer {
    let mut new: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut old: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    let mut old_size: gsize = 0;
    let mut new_size: gsize = 0;
    g_mutex_lock(&raw mut g__array_lock);
    old = ({
        let mut gapg_temp_newval: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut gapg_temp_atomic: *mut gpointer = &raw mut (*array).data;
        *&raw mut gapg_temp_newval = crate::translated::compat::atomic_load_seqcst(gapg_temp_atomic);
        gapg_temp_newval
    }) as *mut guint8;
    if !old.is_null() {
        old_size =
            (*(old as *mut GAtomicArrayMetadata).offset(-(1 as ::core::ffi::c_int as isize))).size;
        new_size = old_size.wrapping_add(additional_element_size);
        new = freelist_alloc(
            new_size,
            (additional_element_size != 0 as gsize) as ::core::ffi::c_int,
        ) as *mut guint8;
        memcpy(
            new as *mut ::core::ffi::c_void,
            old as *const ::core::ffi::c_void,
            old_size as size_t,
        );
    } else if additional_element_size != 0 as gsize {
        new_size = header_size.wrapping_add(additional_element_size);
        new = freelist_alloc(
            new_size,
            (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int,
        ) as *mut guint8;
    } else {
        new = ::core::ptr::null_mut::<guint8>();
    }
    g_mutex_unlock(&raw mut g__array_lock);
    return new as gpointer;
}
#[no_mangle]
pub unsafe extern "C" fn _g_atomic_array_update(
    mut array: *mut GAtomicArray,
    mut new_data: gpointer,
) {
    let mut old: *mut guint8 = ::core::ptr::null_mut::<guint8>();
    g_mutex_lock(&raw mut g__array_lock);
    old = ({
        if 0 as ::core::ffi::c_int != 0 {
            (*array).data;
        } else {
        };
        crate::translated::compat::atomic_xchg_seqcst(&raw mut (*array).data, new_data)
    }) as *mut guint8;
    if old.is_null()
        || (*(old as *mut GAtomicArrayMetadata).offset(-(1 as ::core::ffi::c_int as isize))).size
            <= (*(new_data as *mut GAtomicArrayMetadata)
                .offset(-(1 as ::core::ffi::c_int as isize)))
            .size
    {
    } else {
        g_assertion_message_expr(
            b"GLib-GObject\0" as *const u8 as *const ::core::ffi::c_char,
            b"/home/yans/safelibs/port-glib/safe/vendor/original/gobject/gatomicarray.c\0"
                as *const u8 as *const ::core::ffi::c_char,
            176 as ::core::ffi::c_int,
            b"_g_atomic_array_update\0" as *const u8 as *const ::core::ffi::c_char,
            b"old == NULL || G_ATOMIC_ARRAY_DATA_SIZE (old) <= G_ATOMIC_ARRAY_DATA_SIZE (new_data)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if !old.is_null() {
        freelist_free(old as gpointer);
    }
    g_mutex_unlock(&raw mut g__array_lock);
}
