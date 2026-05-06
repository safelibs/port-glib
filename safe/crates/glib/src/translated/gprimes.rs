pub type gsize = ::core::ffi::c_ulong;
pub type guint = ::core::ffi::c_uint;
static mut safe_c2rust_g_primes: [guint; 34] = [
    11 as ::core::ffi::c_int as guint,
    19 as ::core::ffi::c_int as guint,
    37 as ::core::ffi::c_int as guint,
    73 as ::core::ffi::c_int as guint,
    109 as ::core::ffi::c_int as guint,
    163 as ::core::ffi::c_int as guint,
    251 as ::core::ffi::c_int as guint,
    367 as ::core::ffi::c_int as guint,
    557 as ::core::ffi::c_int as guint,
    823 as ::core::ffi::c_int as guint,
    1237 as ::core::ffi::c_int as guint,
    1861 as ::core::ffi::c_int as guint,
    2777 as ::core::ffi::c_int as guint,
    4177 as ::core::ffi::c_int as guint,
    6247 as ::core::ffi::c_int as guint,
    9371 as ::core::ffi::c_int as guint,
    14057 as ::core::ffi::c_int as guint,
    21089 as ::core::ffi::c_int as guint,
    31627 as ::core::ffi::c_int as guint,
    47431 as ::core::ffi::c_int as guint,
    71143 as ::core::ffi::c_int as guint,
    106721 as ::core::ffi::c_int as guint,
    160073 as ::core::ffi::c_int as guint,
    240101 as ::core::ffi::c_int as guint,
    360163 as ::core::ffi::c_int as guint,
    540217 as ::core::ffi::c_int as guint,
    810343 as ::core::ffi::c_int as guint,
    1215497 as ::core::ffi::c_int as guint,
    1823231 as ::core::ffi::c_int as guint,
    2734867 as ::core::ffi::c_int as guint,
    4102283 as ::core::ffi::c_int as guint,
    6153409 as ::core::ffi::c_int as guint,
    9230113 as ::core::ffi::c_int as guint,
    13845163 as ::core::ffi::c_int as guint,
];
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_spaced_primes_closest(mut num: guint) -> guint {
    let mut i: gsize = 0;
    i = 0 as gsize;
    while (i as usize)
        < (::core::mem::size_of::<[guint; 34]>() as usize)
            .wrapping_div(::core::mem::size_of::<guint>() as usize)
    {
        if safe_c2rust_g_primes[i as usize] > num {
            return safe_c2rust_g_primes[i as usize];
        }
        i = i.wrapping_add(1);
    }
    return safe_c2rust_g_primes[(::core::mem::size_of::<[guint; 34]>() as usize)
        .wrapping_div(::core::mem::size_of::<guint>() as usize)
        .wrapping_sub(1 as usize) as usize];
}
