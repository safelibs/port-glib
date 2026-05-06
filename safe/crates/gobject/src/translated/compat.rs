// SAFETY: These helpers centralize the translated runtime's atomic operations so the
// mechanically translated modules do not rely on removed compiler intrinsics.
use core::mem::{size_of, transmute_copy};
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

unsafe fn from_u32<T: Copy>(value: u32) -> T {
    debug_assert_eq!(size_of::<T>(), 4);
    transmute_copy(&value)
}

unsafe fn from_u64<T: Copy>(value: u64) -> T {
    debug_assert_eq!(size_of::<T>(), 8);
    transmute_copy(&value)
}

unsafe fn to_u32<T: Copy>(value: T) -> u32 {
    debug_assert_eq!(size_of::<T>(), 4);
    transmute_copy(&value)
}

unsafe fn to_u64<T: Copy>(value: T) -> u64 {
    debug_assert_eq!(size_of::<T>(), 8);
    transmute_copy(&value)
}

// SAFETY: Callers must pass a valid, properly aligned address for a 32-bit or 64-bit value.
pub unsafe fn atomic_load_seqcst<T: Copy>(ptr: *mut T) -> T {
    match size_of::<T>() {
        4 => from_u32((*(ptr as *const AtomicU32)).load(Ordering::SeqCst)),
        8 => from_u64((*(ptr as *const AtomicU64)).load(Ordering::SeqCst)),
        size => panic!("unsupported atomic load size {size}"),
    }
}

// SAFETY: Callers must pass a valid, properly aligned address for a 32-bit or 64-bit value.
pub unsafe fn atomic_store_seqcst<T: Copy>(ptr: *mut T, value: T) {
    match size_of::<T>() {
        4 => (*(ptr as *const AtomicU32).cast_mut()).store(to_u32(value), Ordering::SeqCst),
        8 => (*(ptr as *const AtomicU64).cast_mut()).store(to_u64(value), Ordering::SeqCst),
        size => panic!("unsupported atomic store size {size}"),
    }
}

// SAFETY: Callers must pass a valid, properly aligned address for a 32-bit or 64-bit value.
pub unsafe fn atomic_xchg_seqcst<T: Copy>(ptr: *mut T, value: T) -> T {
    match size_of::<T>() {
        4 => from_u32((*(ptr as *const AtomicU32).cast_mut()).swap(to_u32(value), Ordering::SeqCst)),
        8 => from_u64((*(ptr as *const AtomicU64).cast_mut()).swap(to_u64(value), Ordering::SeqCst)),
        size => panic!("unsupported atomic exchange size {size}"),
    }
}

// SAFETY: Callers must pass a valid, properly aligned address for a 32-bit or 64-bit integer.
pub unsafe fn atomic_xadd_seqcst<T: Copy>(ptr: *mut T, value: T) -> T {
    match size_of::<T>() {
        4 => from_u32((*(ptr as *const AtomicU32).cast_mut()).fetch_add(to_u32(value), Ordering::SeqCst)),
        8 => from_u64((*(ptr as *const AtomicU64).cast_mut()).fetch_add(to_u64(value), Ordering::SeqCst)),
        size => panic!("unsupported atomic add size {size}"),
    }
}

// SAFETY: Callers must pass a valid, properly aligned address for a 32-bit or 64-bit integer.
pub unsafe fn atomic_xsub_seqcst<T: Copy>(ptr: *mut T, value: T) -> T {
    match size_of::<T>() {
        4 => from_u32((*(ptr as *const AtomicU32).cast_mut()).fetch_sub(to_u32(value), Ordering::SeqCst)),
        8 => from_u64((*(ptr as *const AtomicU64).cast_mut()).fetch_sub(to_u64(value), Ordering::SeqCst)),
        size => panic!("unsupported atomic sub size {size}"),
    }
}

// SAFETY: Callers must pass a valid, properly aligned address for a 32-bit or 64-bit integer.
pub unsafe fn atomic_or_seqcst<T: Copy>(ptr: *mut T, value: T) -> T {
    match size_of::<T>() {
        4 => from_u32((*(ptr as *const AtomicU32).cast_mut()).fetch_or(to_u32(value), Ordering::SeqCst)),
        8 => from_u64((*(ptr as *const AtomicU64).cast_mut()).fetch_or(to_u64(value), Ordering::SeqCst)),
        size => panic!("unsupported atomic or size {size}"),
    }
}

// SAFETY: Callers must pass a valid, properly aligned address for a 32-bit or 64-bit integer.
pub unsafe fn atomic_and_seqcst<T: Copy>(ptr: *mut T, value: T) -> T {
    match size_of::<T>() {
        4 => from_u32((*(ptr as *const AtomicU32).cast_mut()).fetch_and(to_u32(value), Ordering::SeqCst)),
        8 => from_u64((*(ptr as *const AtomicU64).cast_mut()).fetch_and(to_u64(value), Ordering::SeqCst)),
        size => panic!("unsupported atomic and size {size}"),
    }
}

// SAFETY: Callers must pass a valid, properly aligned address for a 32-bit or 64-bit value.
pub unsafe fn atomic_cxchg_seqcst_seqcst<T: Copy>(ptr: *mut T, old: T, new: T) -> (T, bool) {
    match size_of::<T>() {
        4 => match (*(ptr as *const AtomicU32).cast_mut()).compare_exchange(
            to_u32(old),
            to_u32(new),
            Ordering::SeqCst,
            Ordering::SeqCst,
        ) {
            Ok(previous) => (from_u32(previous), true),
            Err(previous) => (from_u32(previous), false),
        },
        8 => match (*(ptr as *const AtomicU64).cast_mut()).compare_exchange(
            to_u64(old),
            to_u64(new),
            Ordering::SeqCst,
            Ordering::SeqCst,
        ) {
            Ok(previous) => (from_u64(previous), true),
            Err(previous) => (from_u64(previous), false),
        },
        size => panic!("unsupported atomic compare-exchange size {size}"),
    }
}
