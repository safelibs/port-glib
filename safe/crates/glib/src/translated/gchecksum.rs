use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::core::arch::asm;
extern "C" {
    pub type _GBytes;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_bytes_get_data(bytes: *mut GBytes, size: *mut gsize) -> gconstpointer;
    fn g_slice_alloc(block_size: gsize) -> gpointer;
    fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    fn g_free(mem: gpointer);
    fn g_malloc(n_bytes: gsize) -> gpointer;
    fn g_malloc_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    fn g_strdup(str: *const gchar) -> *mut gchar;
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
}
pub type size_t = usize;
pub type guint8 = ::core::ffi::c_uchar;
pub type guint32 = ::core::ffi::c_uint;
pub type guint64 = ::core::ffi::c_ulong;
pub type gssize = ::core::ffi::c_long;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guchar = ::core::ffi::c_uchar;
pub type guint = ::core::ffi::c_uint;
pub type gpointer = *mut ::core::ffi::c_void;
pub type gconstpointer = *const ::core::ffi::c_void;
pub type GBytes = _GBytes;
pub type GChecksumType = ::core::ffi::c_uint;
pub const G_CHECKSUM_SHA384: GChecksumType = 4;
pub const G_CHECKSUM_SHA512: GChecksumType = 3;
pub const G_CHECKSUM_SHA256: GChecksumType = 2;
pub const G_CHECKSUM_SHA1: GChecksumType = 1;
pub const G_CHECKSUM_MD5: GChecksumType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GChecksum {
    pub type_0: GChecksumType,
    pub digest_str: *mut gchar,
    pub sum: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub md5: Md5sum,
    pub sha1: Sha1sum,
    pub sha256: Sha256sum,
    pub sha512: Sha512sum,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sha512sum {
    pub H: [guint64; 8],
    pub block: [guint8; 128],
    pub block_len: guint8,
    pub data_len: [guint64; 2],
    pub digest: [guchar; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sha256sum {
    pub buf: [guint32; 8],
    pub bits: [guint32; 2],
    pub data: [guint8; 64],
    pub digest: [guchar; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sha1sum {
    pub buf: [guint32; 5],
    pub bits: [guint32; 2],
    pub data: [guint32; 16],
    pub digest: [guchar; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Md5sum {
    pub buf: [guint32; 4],
    pub bits: [guint32; 2],
    pub u: C2RustUnnamed_0,
    pub digest: [guchar; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub data: [guchar; 64],
    pub data32: [guint32; 16],
}
pub type GChecksum = _GChecksum;
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
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const G_MAXULONG: ::core::ffi::c_ulong = ULONG_MAX;
pub const G_MAXSIZE: ::core::ffi::c_ulong = G_MAXULONG;
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
static mut safe_c2rust_hex_digits: [gchar; 17] =
    unsafe { ::core::mem::transmute::<[u8; 17], [gchar; 17]>(*b"0123456789abcdef\0") };
pub const MD5_DATASIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MD5_DIGEST_LEN: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const SHA1_DATASIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const SHA1_DIGEST_LEN: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const SHA256_DATASIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const SHA256_DIGEST_LEN: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const SHA2_BLOCK_LEN: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const SHA384_DIGEST_LEN: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const SHA512_DIGEST_LEN: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn safe_c2rust_sha_byte_reverse(mut buffer: *mut guint32, mut length: gint) {
    length = (length as ::core::ffi::c_ulong)
        .wrapping_div(::core::mem::size_of::<guint32>() as usize as ::core::ffi::c_ulong)
        as gint as gint;
    loop {
        let fresh85 = length;
        length = length - 1;
        if !(fresh85 != 0) {
            break;
        }
        *buffer = ({
            let mut __v: guint32 = 0;
            let mut __x: guint32 = *buffer;
            if 0 != 0 {
                __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                    | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                    | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                    | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
            } else {
                let fresh86 = &mut __v;
                let fresh87;
                let fresh88 = __x;
                asm!(
                    "bswapl {0:e}\n", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh86, fresh88) => fresh87,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh86, fresh88, fresh87);
            }
            __v
        });
        buffer = buffer.offset(1);
    }
}
unsafe extern "C" fn safe_c2rust_digest_to_string(
    mut digest: *mut guint8,
    mut digest_len: gsize,
) -> *mut gchar {
    let mut i: gsize = 0;
    let mut len: gsize = digest_len.wrapping_mul(2 as gsize);
    let mut retval: *mut gchar = ::core::ptr::null_mut::<gchar>();
    retval = ({
        let mut __n: gsize = len.wrapping_add(1 as gsize);
        let mut __s: gsize = ::core::mem::size_of::<gchar>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if __s == 1 as gsize {
            __p = g_malloc(__n);
        } else if 0 != 0 && (__s == 0 as gsize || __n <= G_MAXSIZE.wrapping_div(__s)) {
            __p = g_malloc(__n.wrapping_mul(__s));
        } else {
            __p = g_malloc_n(__n, __s);
        }
        __p
    }) as *mut gchar;
    i = 0 as gsize;
    while i < digest_len {
        let mut byte: guint8 = *digest.offset(i as isize);
        *retval.offset((2 as gsize).wrapping_mul(i) as isize) = safe_c2rust_hex_digits
            [(byte as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as usize];
        *retval.offset((2 as gsize).wrapping_mul(i).wrapping_add(1 as gsize) as isize) =
            safe_c2rust_hex_digits
                [(byte as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize];
        i = i.wrapping_add(1);
    }
    *retval.offset(len as isize) = 0 as gchar;
    return retval;
}
unsafe extern "C" fn safe_c2rust_md5_sum_init(mut md5: *mut Md5sum) {
    (*md5).buf[0 as ::core::ffi::c_int as usize] = 0x67452301 as ::core::ffi::c_int as guint32;
    (*md5).buf[1 as ::core::ffi::c_int as usize] = 0xefcdab89 as ::core::ffi::c_uint as guint32;
    (*md5).buf[2 as ::core::ffi::c_int as usize] = 0x98badcfe as ::core::ffi::c_uint as guint32;
    (*md5).buf[3 as ::core::ffi::c_int as usize] = 0x10325476 as ::core::ffi::c_int as guint32;
    (*md5).bits[1 as ::core::ffi::c_int as usize] = 0 as guint32;
    (*md5).bits[0 as ::core::ffi::c_int as usize] = (*md5).bits[1 as ::core::ffi::c_int as usize];
}
unsafe extern "C" fn safe_c2rust_md5_transform(mut buf: *mut guint32, mut in_0: *const guint32) {
    let mut a: guint32 = 0;
    let mut b: guint32 = 0;
    let mut c: guint32 = 0;
    let mut d: guint32 = 0;
    a = *buf.offset(0 as ::core::ffi::c_int as isize);
    b = *buf.offset(1 as ::core::ffi::c_int as isize);
    c = *buf.offset(2 as ::core::ffi::c_int as isize);
    d = *buf.offset(3 as ::core::ffi::c_int as isize);
    a = (a as ::core::ffi::c_uint).wrapping_add(
        (d as ::core::ffi::c_uint
            ^ b as ::core::ffi::c_uint & (c as ::core::ffi::c_uint ^ d as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xd76aa478 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    a = a << 7 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = (d as ::core::ffi::c_uint).wrapping_add(
        (c as ::core::ffi::c_uint
            ^ a as ::core::ffi::c_uint & (b as ::core::ffi::c_uint ^ c as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xe8c7b756 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    d = d << 12 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (b ^ d & (a ^ b))
            .wrapping_add(*in_0.offset(2 as ::core::ffi::c_int as isize))
            .wrapping_add(0x242070db as ::core::ffi::c_int as guint32),
    );
    c = c << 17 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = (b as ::core::ffi::c_uint).wrapping_add(
        (a as ::core::ffi::c_uint
            ^ c as ::core::ffi::c_uint & (d as ::core::ffi::c_uint ^ a as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xc1bdceee as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    b = b << 22 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = (a as ::core::ffi::c_uint).wrapping_add(
        (d as ::core::ffi::c_uint
            ^ b as ::core::ffi::c_uint & (c as ::core::ffi::c_uint ^ d as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xf57c0faf as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    a = a << 7 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = d.wrapping_add(
        (c ^ a & (b ^ c))
            .wrapping_add(*in_0.offset(5 as ::core::ffi::c_int as isize))
            .wrapping_add(0x4787c62a as ::core::ffi::c_int as guint32),
    );
    d = d << 12 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = (c as ::core::ffi::c_uint).wrapping_add(
        (b as ::core::ffi::c_uint
            ^ d as ::core::ffi::c_uint & (a as ::core::ffi::c_uint ^ b as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xa8304613 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    c = c << 17 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = (b as ::core::ffi::c_uint).wrapping_add(
        (a as ::core::ffi::c_uint
            ^ c as ::core::ffi::c_uint & (d as ::core::ffi::c_uint ^ a as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xfd469501 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    b = b << 22 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = a.wrapping_add(
        (d ^ b & (c ^ d))
            .wrapping_add(*in_0.offset(8 as ::core::ffi::c_int as isize))
            .wrapping_add(0x698098d8 as ::core::ffi::c_int as guint32),
    );
    a = a << 7 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = (d as ::core::ffi::c_uint).wrapping_add(
        (c as ::core::ffi::c_uint
            ^ a as ::core::ffi::c_uint & (b as ::core::ffi::c_uint ^ c as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0x8b44f7af as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    d = d << 12 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = (c as ::core::ffi::c_uint).wrapping_add(
        (b as ::core::ffi::c_uint
            ^ d as ::core::ffi::c_uint & (a as ::core::ffi::c_uint ^ b as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xffff5bb1 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    c = c << 17 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = (b as ::core::ffi::c_uint).wrapping_add(
        (a as ::core::ffi::c_uint
            ^ c as ::core::ffi::c_uint & (d as ::core::ffi::c_uint ^ a as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0x895cd7be as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    b = b << 22 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = a.wrapping_add(
        (d ^ b & (c ^ d))
            .wrapping_add(*in_0.offset(12 as ::core::ffi::c_int as isize))
            .wrapping_add(0x6b901122 as ::core::ffi::c_int as guint32),
    );
    a = a << 7 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = (d as ::core::ffi::c_uint).wrapping_add(
        (c as ::core::ffi::c_uint
            ^ a as ::core::ffi::c_uint & (b as ::core::ffi::c_uint ^ c as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xfd987193 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    d = d << 12 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = (c as ::core::ffi::c_uint).wrapping_add(
        (b as ::core::ffi::c_uint
            ^ d as ::core::ffi::c_uint & (a as ::core::ffi::c_uint ^ b as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xa679438e as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    c = c << 17 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (a ^ c & (d ^ a))
            .wrapping_add(*in_0.offset(15 as ::core::ffi::c_int as isize))
            .wrapping_add(0x49b40821 as ::core::ffi::c_int as guint32),
    );
    b = b << 22 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = (a as ::core::ffi::c_uint).wrapping_add(
        (c as ::core::ffi::c_uint
            ^ d as ::core::ffi::c_uint & (b as ::core::ffi::c_uint ^ c as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xf61e2562 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    a = a << 5 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = (d as ::core::ffi::c_uint).wrapping_add(
        (b as ::core::ffi::c_uint
            ^ c as ::core::ffi::c_uint & (a as ::core::ffi::c_uint ^ b as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xc040b340 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    d = d << 9 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (a ^ b & (d ^ a))
            .wrapping_add(*in_0.offset(11 as ::core::ffi::c_int as isize))
            .wrapping_add(0x265e5a51 as ::core::ffi::c_int as guint32),
    );
    c = c << 14 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = (b as ::core::ffi::c_uint).wrapping_add(
        (d as ::core::ffi::c_uint
            ^ a as ::core::ffi::c_uint & (c as ::core::ffi::c_uint ^ d as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xe9b6c7aa as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    b = b << 20 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = (a as ::core::ffi::c_uint).wrapping_add(
        (c as ::core::ffi::c_uint
            ^ d as ::core::ffi::c_uint & (b as ::core::ffi::c_uint ^ c as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xd62f105d as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    a = a << 5 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = d.wrapping_add(
        (b ^ c & (a ^ b))
            .wrapping_add(*in_0.offset(10 as ::core::ffi::c_int as isize))
            .wrapping_add(0x2441453 as ::core::ffi::c_int as guint32),
    );
    d = d << 9 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = (c as ::core::ffi::c_uint).wrapping_add(
        (a as ::core::ffi::c_uint
            ^ b as ::core::ffi::c_uint & (d as ::core::ffi::c_uint ^ a as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xd8a1e681 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    c = c << 14 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = (b as ::core::ffi::c_uint).wrapping_add(
        (d as ::core::ffi::c_uint
            ^ a as ::core::ffi::c_uint & (c as ::core::ffi::c_uint ^ d as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xe7d3fbc8 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    b = b << 20 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = a.wrapping_add(
        (c ^ d & (b ^ c))
            .wrapping_add(*in_0.offset(9 as ::core::ffi::c_int as isize))
            .wrapping_add(0x21e1cde6 as ::core::ffi::c_int as guint32),
    );
    a = a << 5 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = (d as ::core::ffi::c_uint).wrapping_add(
        (b as ::core::ffi::c_uint
            ^ c as ::core::ffi::c_uint & (a as ::core::ffi::c_uint ^ b as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xc33707d6 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    d = d << 9 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = (c as ::core::ffi::c_uint).wrapping_add(
        (a as ::core::ffi::c_uint
            ^ b as ::core::ffi::c_uint & (d as ::core::ffi::c_uint ^ a as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xf4d50d87 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    c = c << 14 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (d ^ a & (c ^ d))
            .wrapping_add(*in_0.offset(8 as ::core::ffi::c_int as isize))
            .wrapping_add(0x455a14ed as ::core::ffi::c_int as guint32),
    );
    b = b << 20 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = (a as ::core::ffi::c_uint).wrapping_add(
        (c as ::core::ffi::c_uint
            ^ d as ::core::ffi::c_uint & (b as ::core::ffi::c_uint ^ c as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xa9e3e905 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    a = a << 5 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = (d as ::core::ffi::c_uint).wrapping_add(
        (b as ::core::ffi::c_uint
            ^ c as ::core::ffi::c_uint & (a as ::core::ffi::c_uint ^ b as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xfcefa3f8 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    d = d << 9 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (a ^ b & (d ^ a))
            .wrapping_add(*in_0.offset(7 as ::core::ffi::c_int as isize))
            .wrapping_add(0x676f02d9 as ::core::ffi::c_int as guint32),
    );
    c = c << 14 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = (b as ::core::ffi::c_uint).wrapping_add(
        (d as ::core::ffi::c_uint
            ^ a as ::core::ffi::c_uint & (c as ::core::ffi::c_uint ^ d as ::core::ffi::c_uint))
            .wrapping_add(*in_0.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0x8d2a4c8a as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    b = b << 20 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = (a as ::core::ffi::c_uint).wrapping_add(
        (b as ::core::ffi::c_uint ^ c as ::core::ffi::c_uint ^ d as ::core::ffi::c_uint)
            .wrapping_add(*in_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xfffa3942 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    a = a << 4 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = (d as ::core::ffi::c_uint).wrapping_add(
        (a as ::core::ffi::c_uint ^ b as ::core::ffi::c_uint ^ c as ::core::ffi::c_uint)
            .wrapping_add(*in_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0x8771f681 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    d = d << 11 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(*in_0.offset(11 as ::core::ffi::c_int as isize))
            .wrapping_add(0x6d9d6122 as ::core::ffi::c_int as guint32),
    );
    c = c << 16 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = (b as ::core::ffi::c_uint).wrapping_add(
        (c as ::core::ffi::c_uint ^ d as ::core::ffi::c_uint ^ a as ::core::ffi::c_uint)
            .wrapping_add(*in_0.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xfde5380c as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    b = b << 23 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = (a as ::core::ffi::c_uint).wrapping_add(
        (b as ::core::ffi::c_uint ^ c as ::core::ffi::c_uint ^ d as ::core::ffi::c_uint)
            .wrapping_add(*in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xa4beea44 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    a = a << 4 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = d.wrapping_add(
        (a ^ b ^ c)
            .wrapping_add(*in_0.offset(4 as ::core::ffi::c_int as isize))
            .wrapping_add(0x4bdecfa9 as ::core::ffi::c_int as guint32),
    );
    d = d << 11 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = (c as ::core::ffi::c_uint).wrapping_add(
        (d as ::core::ffi::c_uint ^ a as ::core::ffi::c_uint ^ b as ::core::ffi::c_uint)
            .wrapping_add(*in_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xf6bb4b60 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    c = c << 16 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = (b as ::core::ffi::c_uint).wrapping_add(
        (c as ::core::ffi::c_uint ^ d as ::core::ffi::c_uint ^ a as ::core::ffi::c_uint)
            .wrapping_add(*in_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xbebfbc70 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    b = b << 23 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = a.wrapping_add(
        (b ^ c ^ d)
            .wrapping_add(*in_0.offset(13 as ::core::ffi::c_int as isize))
            .wrapping_add(0x289b7ec6 as ::core::ffi::c_int as guint32),
    );
    a = a << 4 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = (d as ::core::ffi::c_uint).wrapping_add(
        (a as ::core::ffi::c_uint ^ b as ::core::ffi::c_uint ^ c as ::core::ffi::c_uint)
            .wrapping_add(*in_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xeaa127fa as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    d = d << 11 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = (c as ::core::ffi::c_uint).wrapping_add(
        (d as ::core::ffi::c_uint ^ a as ::core::ffi::c_uint ^ b as ::core::ffi::c_uint)
            .wrapping_add(*in_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xd4ef3085 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    c = c << 16 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(*in_0.offset(6 as ::core::ffi::c_int as isize))
            .wrapping_add(0x4881d05 as ::core::ffi::c_int as guint32),
    );
    b = b << 23 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = (a as ::core::ffi::c_uint).wrapping_add(
        (b as ::core::ffi::c_uint ^ c as ::core::ffi::c_uint ^ d as ::core::ffi::c_uint)
            .wrapping_add(*in_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xd9d4d039 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    a = a << 4 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = (d as ::core::ffi::c_uint).wrapping_add(
        (a as ::core::ffi::c_uint ^ b as ::core::ffi::c_uint ^ c as ::core::ffi::c_uint)
            .wrapping_add(*in_0.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xe6db99e5 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    d = d << 11 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(*in_0.offset(15 as ::core::ffi::c_int as isize))
            .wrapping_add(0x1fa27cf8 as ::core::ffi::c_int as guint32),
    );
    c = c << 16 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = (b as ::core::ffi::c_uint).wrapping_add(
        (c as ::core::ffi::c_uint ^ d as ::core::ffi::c_uint ^ a as ::core::ffi::c_uint)
            .wrapping_add(*in_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xc4ac5665 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    b = b << 23 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = (a as ::core::ffi::c_uint).wrapping_add(
        (c as ::core::ffi::c_uint ^ (b as ::core::ffi::c_uint | !(d as ::core::ffi::c_uint)))
            .wrapping_add(*in_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xf4292244 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    a = a << 6 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = d.wrapping_add(
        (b ^ (a | !c))
            .wrapping_add(*in_0.offset(7 as ::core::ffi::c_int as isize))
            .wrapping_add(0x432aff97 as ::core::ffi::c_int as guint32),
    );
    d = d << 10 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = (c as ::core::ffi::c_uint).wrapping_add(
        (a as ::core::ffi::c_uint ^ (d as ::core::ffi::c_uint | !(b as ::core::ffi::c_uint)))
            .wrapping_add(*in_0.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xab9423a7 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    c = c << 15 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = (b as ::core::ffi::c_uint).wrapping_add(
        (d as ::core::ffi::c_uint ^ (c as ::core::ffi::c_uint | !(a as ::core::ffi::c_uint)))
            .wrapping_add(*in_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xfc93a039 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    b = b << 21 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = a.wrapping_add(
        (c ^ (b | !d))
            .wrapping_add(*in_0.offset(12 as ::core::ffi::c_int as isize))
            .wrapping_add(0x655b59c3 as ::core::ffi::c_int as guint32),
    );
    a = a << 6 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = (d as ::core::ffi::c_uint).wrapping_add(
        (b as ::core::ffi::c_uint ^ (a as ::core::ffi::c_uint | !(c as ::core::ffi::c_uint)))
            .wrapping_add(*in_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0x8f0ccc92 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    d = d << 10 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = (c as ::core::ffi::c_uint).wrapping_add(
        (a as ::core::ffi::c_uint ^ (d as ::core::ffi::c_uint | !(b as ::core::ffi::c_uint)))
            .wrapping_add(*in_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xffeff47d as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    c = c << 15 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = (b as ::core::ffi::c_uint).wrapping_add(
        (d as ::core::ffi::c_uint ^ (c as ::core::ffi::c_uint | !(a as ::core::ffi::c_uint)))
            .wrapping_add(*in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0x85845dd1 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    b = b << 21 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = a.wrapping_add(
        (c ^ (b | !d))
            .wrapping_add(*in_0.offset(8 as ::core::ffi::c_int as isize))
            .wrapping_add(0x6fa87e4f as ::core::ffi::c_int as guint32),
    );
    a = a << 6 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = (d as ::core::ffi::c_uint).wrapping_add(
        (b as ::core::ffi::c_uint ^ (a as ::core::ffi::c_uint | !(c as ::core::ffi::c_uint)))
            .wrapping_add(*in_0.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xfe2ce6e0 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    d = d << 10 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = (c as ::core::ffi::c_uint).wrapping_add(
        (a as ::core::ffi::c_uint ^ (d as ::core::ffi::c_uint | !(b as ::core::ffi::c_uint)))
            .wrapping_add(*in_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xa3014314 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    c = c << 15 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = b.wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(*in_0.offset(13 as ::core::ffi::c_int as isize))
            .wrapping_add(0x4e0811a1 as ::core::ffi::c_int as guint32),
    );
    b = b << 21 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    a = (a as ::core::ffi::c_uint).wrapping_add(
        (c as ::core::ffi::c_uint ^ (b as ::core::ffi::c_uint | !(d as ::core::ffi::c_uint)))
            .wrapping_add(*in_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xf7537e82 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    a = a << 6 as ::core::ffi::c_int | a >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
    a = a.wrapping_add(b);
    d = (d as ::core::ffi::c_uint).wrapping_add(
        (b as ::core::ffi::c_uint ^ (a as ::core::ffi::c_uint | !(c as ::core::ffi::c_uint)))
            .wrapping_add(*in_0.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xbd3af235 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    d = d << 10 as ::core::ffi::c_int | d >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
    d = d.wrapping_add(a);
    c = c.wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(*in_0.offset(2 as ::core::ffi::c_int as isize))
            .wrapping_add(0x2ad7d2bb as ::core::ffi::c_int as guint32),
    );
    c = c << 15 as ::core::ffi::c_int | c >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    c = c.wrapping_add(d);
    b = (b as ::core::ffi::c_uint).wrapping_add(
        (d as ::core::ffi::c_uint ^ (c as ::core::ffi::c_uint | !(a as ::core::ffi::c_uint)))
            .wrapping_add(*in_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint)
            .wrapping_add(0xeb86d391 as ::core::ffi::c_uint),
    ) as guint32 as guint32;
    b = b << 21 as ::core::ffi::c_int | b >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
    b = b.wrapping_add(c);
    let ref mut fresh89 = *buf.offset(0 as ::core::ffi::c_int as isize);
    *fresh89 = (*fresh89).wrapping_add(a);
    let ref mut fresh90 = *buf.offset(1 as ::core::ffi::c_int as isize);
    *fresh90 = (*fresh90).wrapping_add(b);
    let ref mut fresh91 = *buf.offset(2 as ::core::ffi::c_int as isize);
    *fresh91 = (*fresh91).wrapping_add(c);
    let ref mut fresh92 = *buf.offset(3 as ::core::ffi::c_int as isize);
    *fresh92 = (*fresh92).wrapping_add(d);
}
unsafe extern "C" fn safe_c2rust_md5_sum_update(
    mut md5: *mut Md5sum,
    mut data: *const guchar,
    mut length: gsize,
) {
    let mut bit: guint32 = 0;
    bit = (*md5).bits[0 as ::core::ffi::c_int as usize];
    (*md5).bits[0 as ::core::ffi::c_int as usize] =
        bit.wrapping_add((length as guint32) << 3 as ::core::ffi::c_int);
    if (*md5).bits[0 as ::core::ffi::c_int as usize] < bit {
        (*md5).bits[1 as ::core::ffi::c_int as usize] =
            (*md5).bits[1 as ::core::ffi::c_int as usize].wrapping_add(1 as guint32);
    }
    (*md5).bits[1 as ::core::ffi::c_int as usize] =
        ((*md5).bits[1 as ::core::ffi::c_int as usize] as gsize)
            .wrapping_add(length >> 29 as ::core::ffi::c_int) as guint32 as guint32;
    bit = bit >> 3 as ::core::ffi::c_int & 0x3f as guint32;
    if bit != 0 {
        let mut p: *mut guchar = (&raw mut (*md5).u.data as *mut guchar).offset(bit as isize);
        bit = (MD5_DATASIZE as guint32).wrapping_sub(bit);
        if length < bit as gsize {
            memcpy(
                p as *mut ::core::ffi::c_void,
                data as *const ::core::ffi::c_void,
                length as size_t,
            );
            return;
        }
        memcpy(
            p as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            bit as size_t,
        );
        safe_c2rust_md5_transform(
            &raw mut (*md5).buf as *mut guint32,
            &raw mut (*md5).u.data32 as *mut guint32 as *const guint32,
        );
        data = data.offset(bit as isize);
        length = length.wrapping_sub(bit as gsize);
    }
    while length >= MD5_DATASIZE as gsize {
        memcpy(
            &raw mut (*md5).u.data as *mut guchar as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            MD5_DATASIZE as size_t,
        );
        safe_c2rust_md5_transform(
            &raw mut (*md5).buf as *mut guint32,
            &raw mut (*md5).u.data32 as *mut guint32 as *const guint32,
        );
        data = data.offset(MD5_DATASIZE as isize);
        length = length.wrapping_sub(MD5_DATASIZE as gsize);
    }
    memcpy(
        &raw mut (*md5).u.data as *mut guchar as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        length as size_t,
    );
}
unsafe extern "C" fn safe_c2rust_md5_sum_close(mut md5: *mut Md5sum) {
    let mut count: guint = 0;
    let mut p: *mut guchar = ::core::ptr::null_mut::<guchar>();
    count = ((*md5).bits[0 as ::core::ffi::c_int as usize] >> 3 as ::core::ffi::c_int
        & 0x3f as guint32) as guint;
    p = (&raw mut (*md5).u.data as *mut guchar).offset(count as isize);
    let fresh94 = p;
    p = p.offset(1);
    *fresh94 = 0x80 as guchar;
    count = ((MD5_DATASIZE - 1 as ::core::ffi::c_int) as guint).wrapping_sub(count);
    if count < 8 as guint {
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            count as size_t,
        );
        safe_c2rust_md5_transform(
            &raw mut (*md5).buf as *mut guint32,
            &raw mut (*md5).u.data32 as *mut guint32 as *const guint32,
        );
        memset(
            &raw mut (*md5).u.data as *mut guchar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (MD5_DATASIZE - 8 as ::core::ffi::c_int) as size_t,
        );
    } else {
        memset(
            p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            count.wrapping_sub(8 as guint) as size_t,
        );
    }
    (*md5).u.data32[14 as ::core::ffi::c_int as usize] =
        (*md5).bits[0 as ::core::ffi::c_int as usize];
    (*md5).u.data32[15 as ::core::ffi::c_int as usize] =
        (*md5).bits[1 as ::core::ffi::c_int as usize];
    safe_c2rust_md5_transform(
        &raw mut (*md5).buf as *mut guint32,
        &raw mut (*md5).u.data32 as *mut guint32 as *const guint32,
    );
    memcpy(
        &raw mut (*md5).digest as *mut guchar as *mut ::core::ffi::c_void,
        &raw mut (*md5).buf as *mut guint32 as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memset(
        &raw mut (*md5).buf as *mut guint32 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[guint32; 4]>() as size_t,
    );
    memset(
        &raw mut (*md5).u.data as *mut guchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[guchar; 64]>() as size_t,
    );
}
unsafe extern "C" fn safe_c2rust_md5_sum_to_string(mut md5: *mut Md5sum) -> *mut gchar {
    return safe_c2rust_digest_to_string(
        &raw mut (*md5).digest as *mut guint8,
        MD5_DIGEST_LEN as gsize,
    );
}
unsafe extern "C" fn safe_c2rust_md5_sum_digest(mut md5: *mut Md5sum, mut digest: *mut guint8) {
    let mut i: gint = 0;
    i = 0 as ::core::ffi::c_int as gint;
    while i < MD5_DIGEST_LEN {
        *digest.offset(i as isize) = (*md5).digest[i as usize] as guint8;
        i += 1;
    }
}
unsafe extern "C" fn safe_c2rust_sha1_sum_init(mut sha1: *mut Sha1sum) {
    (*sha1).buf[0 as ::core::ffi::c_int as usize] = 0x67452301 as ::core::ffi::c_long as guint32;
    (*sha1).buf[1 as ::core::ffi::c_int as usize] = 0xefcdab89 as ::core::ffi::c_long as guint32;
    (*sha1).buf[2 as ::core::ffi::c_int as usize] = 0x98badcfe as ::core::ffi::c_long as guint32;
    (*sha1).buf[3 as ::core::ffi::c_int as usize] = 0x10325476 as ::core::ffi::c_long as guint32;
    (*sha1).buf[4 as ::core::ffi::c_int as usize] = 0xc3d2e1f0 as ::core::ffi::c_long as guint32;
    (*sha1).bits[1 as ::core::ffi::c_int as usize] = 0 as guint32;
    (*sha1).bits[0 as ::core::ffi::c_int as usize] = (*sha1).bits[1 as ::core::ffi::c_int as usize];
}
unsafe extern "C" fn safe_c2rust_sha1_transform(mut buf: *mut guint32, mut in_0: *mut guint32) {
    let mut A: guint32 = 0;
    let mut B: guint32 = 0;
    let mut C: guint32 = 0;
    let mut D: guint32 = 0;
    let mut E: guint32 = 0;
    A = *buf.offset(0 as ::core::ffi::c_int as isize);
    B = *buf.offset(1 as ::core::ffi::c_int as isize);
    C = *buf.offset(2 as ::core::ffi::c_int as isize);
    D = *buf.offset(3 as ::core::ffi::c_int as isize);
    E = *buf.offset(4 as ::core::ffi::c_int as isize);
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D ^ B & (C ^ D)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C ^ A & (B ^ C)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B ^ E & (A ^ B)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A ^ D & (E ^ A)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E ^ C & (D ^ E)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D ^ B & (C ^ D)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C ^ A & (B ^ C)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B ^ E & (A ^ B)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A ^ D & (E ^ A)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E ^ C & (D ^ E)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D ^ B & (C ^ D)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C ^ A & (B ^ C)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B ^ E & (A ^ B)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A ^ D & (E ^ A)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E ^ C & (D ^ E)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D ^ B & (C ^ D)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *in_0.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_long))
        as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh16 =
        *in_0.offset((16 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh16 = (*in_0.offset((16 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (16 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (16 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (16 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((16 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (16 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (16 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (16 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C ^ A & (B ^ C)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *fresh16 as ::core::ffi::c_long)) as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh17 =
        *in_0.offset((17 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh17 = (*in_0.offset((17 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (17 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (17 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (17 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((17 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (17 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (17 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (17 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B ^ E & (A ^ B)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *fresh17 as ::core::ffi::c_long)) as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh18 =
        *in_0.offset((18 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh18 = (*in_0.offset((18 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (18 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (18 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (18 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((18 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (18 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (18 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (18 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A ^ D & (E ^ A)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *fresh18 as ::core::ffi::c_long)) as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh19 =
        *in_0.offset((19 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh19 = (*in_0.offset((19 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (19 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (19 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (19 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((19 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (19 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (19 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (19 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E ^ C & (D ^ E)) as ::core::ffi::c_long
            + 0x5a827999 as ::core::ffi::c_long
            + *fresh19 as ::core::ffi::c_long)) as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh20 =
        *in_0.offset((20 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh20 = (*in_0.offset((20 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (20 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (20 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (20 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((20 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (20 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (20 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (20 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B ^ C ^ D) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh20 as ::core::ffi::c_long)) as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh21 =
        *in_0.offset((21 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh21 = (*in_0.offset((21 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (21 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (21 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (21 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((21 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (21 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (21 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (21 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A ^ B ^ C) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh21 as ::core::ffi::c_long)) as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh22 =
        *in_0.offset((22 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh22 = (*in_0.offset((22 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (22 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (22 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (22 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((22 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (22 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (22 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (22 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E ^ A ^ B) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh22 as ::core::ffi::c_long)) as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh23 =
        *in_0.offset((23 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh23 = (*in_0.offset((23 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (23 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (23 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (23 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((23 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (23 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (23 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (23 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D ^ E ^ A) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh23 as ::core::ffi::c_long)) as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh24 =
        *in_0.offset((24 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh24 = (*in_0.offset((24 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (24 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (24 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (24 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((24 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (24 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (24 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (24 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C ^ D ^ E) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh24 as ::core::ffi::c_long)) as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh25 =
        *in_0.offset((25 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh25 = (*in_0.offset((25 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (25 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (25 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (25 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((25 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (25 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (25 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (25 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B ^ C ^ D) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh25 as ::core::ffi::c_long)) as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh26 =
        *in_0.offset((26 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh26 = (*in_0.offset((26 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (26 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (26 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (26 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((26 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (26 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (26 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (26 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A ^ B ^ C) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh26 as ::core::ffi::c_long)) as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh27 =
        *in_0.offset((27 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh27 = (*in_0.offset((27 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (27 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (27 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (27 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((27 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (27 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (27 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (27 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E ^ A ^ B) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh27 as ::core::ffi::c_long)) as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh28 =
        *in_0.offset((28 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh28 = (*in_0.offset((28 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (28 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (28 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (28 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((28 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (28 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (28 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (28 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D ^ E ^ A) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh28 as ::core::ffi::c_long)) as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh29 =
        *in_0.offset((29 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh29 = (*in_0.offset((29 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (29 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (29 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (29 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((29 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (29 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (29 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (29 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C ^ D ^ E) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh29 as ::core::ffi::c_long)) as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh30 =
        *in_0.offset((30 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh30 = (*in_0.offset((30 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (30 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (30 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (30 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((30 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (30 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (30 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (30 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B ^ C ^ D) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh30 as ::core::ffi::c_long)) as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh31 =
        *in_0.offset((31 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh31 = (*in_0.offset((31 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (31 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (31 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (31 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((31 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (31 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (31 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (31 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A ^ B ^ C) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh31 as ::core::ffi::c_long)) as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh32 =
        *in_0.offset((32 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh32 = (*in_0.offset((32 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (32 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((32 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (32 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E ^ A ^ B) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh32 as ::core::ffi::c_long)) as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh33 =
        *in_0.offset((33 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh33 = (*in_0.offset((33 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (33 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (33 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (33 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((33 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (33 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (33 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (33 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D ^ E ^ A) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh33 as ::core::ffi::c_long)) as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh34 =
        *in_0.offset((34 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh34 = (*in_0.offset((34 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (34 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (34 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (34 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((34 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (34 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (34 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (34 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C ^ D ^ E) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh34 as ::core::ffi::c_long)) as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh35 =
        *in_0.offset((35 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh35 = (*in_0.offset((35 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (35 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (35 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (35 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((35 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (35 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (35 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (35 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B ^ C ^ D) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh35 as ::core::ffi::c_long)) as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh36 =
        *in_0.offset((36 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh36 = (*in_0.offset((36 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (36 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (36 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (36 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((36 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (36 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (36 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (36 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A ^ B ^ C) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh36 as ::core::ffi::c_long)) as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh37 =
        *in_0.offset((37 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh37 = (*in_0.offset((37 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (37 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (37 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (37 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((37 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (37 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (37 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (37 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E ^ A ^ B) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh37 as ::core::ffi::c_long)) as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh38 =
        *in_0.offset((38 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh38 = (*in_0.offset((38 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (38 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (38 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (38 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((38 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (38 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (38 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (38 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D ^ E ^ A) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh38 as ::core::ffi::c_long)) as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh39 =
        *in_0.offset((39 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh39 = (*in_0.offset((39 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (39 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (39 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (39 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((39 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (39 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (39 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (39 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C ^ D ^ E) as ::core::ffi::c_long
            + 0x6ed9eba1 as ::core::ffi::c_long
            + *fresh39 as ::core::ffi::c_long)) as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh40 =
        *in_0.offset((40 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh40 = (*in_0.offset((40 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (40 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (40 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (40 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((40 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (40 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (40 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (40 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B & C | D & (B | C)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh40 as ::core::ffi::c_long)) as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh41 =
        *in_0.offset((41 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh41 = (*in_0.offset((41 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (41 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (41 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (41 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((41 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (41 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (41 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (41 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A & B | C & (A | B)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh41 as ::core::ffi::c_long)) as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh42 =
        *in_0.offset((42 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh42 = (*in_0.offset((42 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (42 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (42 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (42 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((42 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (42 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (42 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (42 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E & A | B & (E | A)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh42 as ::core::ffi::c_long)) as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh43 =
        *in_0.offset((43 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh43 = (*in_0.offset((43 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (43 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (43 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (43 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((43 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (43 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (43 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (43 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D & E | A & (D | E)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh43 as ::core::ffi::c_long)) as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh44 =
        *in_0.offset((44 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh44 = (*in_0.offset((44 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (44 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (44 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (44 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((44 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (44 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (44 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (44 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C & D | E & (C | D)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh44 as ::core::ffi::c_long)) as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh45 =
        *in_0.offset((45 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh45 = (*in_0.offset((45 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (45 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (45 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (45 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((45 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (45 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (45 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (45 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B & C | D & (B | C)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh45 as ::core::ffi::c_long)) as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh46 =
        *in_0.offset((46 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh46 = (*in_0.offset((46 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (46 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (46 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (46 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((46 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (46 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (46 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (46 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A & B | C & (A | B)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh46 as ::core::ffi::c_long)) as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh47 =
        *in_0.offset((47 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh47 = (*in_0.offset((47 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (47 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (47 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (47 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((47 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (47 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (47 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (47 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E & A | B & (E | A)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh47 as ::core::ffi::c_long)) as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh48 =
        *in_0.offset((48 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh48 = (*in_0.offset((48 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (48 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (48 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (48 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((48 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (48 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (48 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (48 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D & E | A & (D | E)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh48 as ::core::ffi::c_long)) as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh49 =
        *in_0.offset((49 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh49 = (*in_0.offset((49 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (49 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (49 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (49 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((49 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (49 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (49 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (49 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C & D | E & (C | D)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh49 as ::core::ffi::c_long)) as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh50 =
        *in_0.offset((50 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh50 = (*in_0.offset((50 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (50 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (50 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (50 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((50 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (50 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (50 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (50 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B & C | D & (B | C)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh50 as ::core::ffi::c_long)) as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh51 =
        *in_0.offset((51 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh51 = (*in_0.offset((51 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (51 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (51 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (51 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((51 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (51 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (51 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (51 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A & B | C & (A | B)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh51 as ::core::ffi::c_long)) as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh52 =
        *in_0.offset((52 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh52 = (*in_0.offset((52 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (52 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (52 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (52 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((52 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (52 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (52 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (52 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E & A | B & (E | A)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh52 as ::core::ffi::c_long)) as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh53 =
        *in_0.offset((53 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh53 = (*in_0.offset((53 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (53 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (53 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (53 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((53 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (53 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (53 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (53 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D & E | A & (D | E)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh53 as ::core::ffi::c_long)) as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh54 =
        *in_0.offset((54 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh54 = (*in_0.offset((54 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (54 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (54 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (54 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((54 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (54 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (54 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (54 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C & D | E & (C | D)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh54 as ::core::ffi::c_long)) as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh55 =
        *in_0.offset((55 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh55 = (*in_0.offset((55 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (55 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (55 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (55 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((55 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (55 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (55 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (55 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B & C | D & (B | C)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh55 as ::core::ffi::c_long)) as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh56 =
        *in_0.offset((56 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh56 = (*in_0.offset((56 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (56 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (56 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (56 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((56 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (56 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (56 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (56 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A & B | C & (A | B)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh56 as ::core::ffi::c_long)) as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh57 =
        *in_0.offset((57 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh57 = (*in_0.offset((57 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (57 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (57 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (57 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((57 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (57 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (57 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (57 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E & A | B & (E | A)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh57 as ::core::ffi::c_long)) as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh58 =
        *in_0.offset((58 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh58 = (*in_0.offset((58 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (58 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (58 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (58 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((58 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (58 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (58 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (58 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D & E | A & (D | E)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh58 as ::core::ffi::c_long)) as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh59 =
        *in_0.offset((59 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh59 = (*in_0.offset((59 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (59 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (59 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (59 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((59 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (59 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (59 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (59 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C & D | E & (C | D)) as ::core::ffi::c_long
            + 0x8f1bbcdc as ::core::ffi::c_long
            + *fresh59 as ::core::ffi::c_long)) as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh60 =
        *in_0.offset((60 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh60 = (*in_0.offset((60 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (60 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (60 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (60 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((60 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (60 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (60 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (60 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B ^ C ^ D) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh60 as ::core::ffi::c_long)) as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh61 =
        *in_0.offset((61 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh61 = (*in_0.offset((61 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (61 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (61 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (61 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((61 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (61 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (61 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (61 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A ^ B ^ C) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh61 as ::core::ffi::c_long)) as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh62 =
        *in_0.offset((62 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh62 = (*in_0.offset((62 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (62 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (62 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (62 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((62 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (62 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (62 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (62 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E ^ A ^ B) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh62 as ::core::ffi::c_long)) as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh63 =
        *in_0.offset((63 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh63 = (*in_0.offset((63 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (63 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (63 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (63 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((63 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (63 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (63 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (63 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D ^ E ^ A) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh63 as ::core::ffi::c_long)) as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh64 =
        *in_0.offset((64 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh64 = (*in_0.offset((64 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (64 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((64 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (64 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C ^ D ^ E) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh64 as ::core::ffi::c_long)) as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh65 =
        *in_0.offset((65 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh65 = (*in_0.offset((65 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (65 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (65 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (65 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((65 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (65 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (65 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (65 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B ^ C ^ D) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh65 as ::core::ffi::c_long)) as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh66 =
        *in_0.offset((66 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh66 = (*in_0.offset((66 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (66 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (66 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (66 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((66 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (66 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (66 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (66 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A ^ B ^ C) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh66 as ::core::ffi::c_long)) as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh67 =
        *in_0.offset((67 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh67 = (*in_0.offset((67 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (67 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (67 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (67 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((67 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (67 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (67 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (67 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E ^ A ^ B) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh67 as ::core::ffi::c_long)) as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh68 =
        *in_0.offset((68 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh68 = (*in_0.offset((68 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (68 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (68 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (68 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((68 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (68 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (68 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (68 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D ^ E ^ A) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh68 as ::core::ffi::c_long)) as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh69 =
        *in_0.offset((69 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh69 = (*in_0.offset((69 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (69 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (69 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (69 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((69 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (69 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (69 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (69 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C ^ D ^ E) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh69 as ::core::ffi::c_long)) as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh70 =
        *in_0.offset((70 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh70 = (*in_0.offset((70 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (70 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (70 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (70 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((70 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (70 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (70 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (70 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B ^ C ^ D) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh70 as ::core::ffi::c_long)) as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh71 =
        *in_0.offset((71 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh71 = (*in_0.offset((71 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (71 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (71 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (71 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((71 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (71 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (71 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (71 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A ^ B ^ C) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh71 as ::core::ffi::c_long)) as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh72 =
        *in_0.offset((72 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh72 = (*in_0.offset((72 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (72 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (72 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (72 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((72 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (72 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (72 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (72 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E ^ A ^ B) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh72 as ::core::ffi::c_long)) as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh73 =
        *in_0.offset((73 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh73 = (*in_0.offset((73 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (73 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (73 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (73 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((73 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (73 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (73 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (73 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D ^ E ^ A) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh73 as ::core::ffi::c_long)) as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh74 =
        *in_0.offset((74 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh74 = (*in_0.offset((74 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (74 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (74 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (74 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((74 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (74 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (74 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (74 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C ^ D ^ E) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh74 as ::core::ffi::c_long)) as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh75 =
        *in_0.offset((75 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh75 = (*in_0.offset((75 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (75 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (75 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (75 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((75 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (75 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (75 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (75 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    E = (E as ::core::ffi::c_long
        + ((A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(B ^ C ^ D) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh75 as ::core::ffi::c_long)) as guint32;
    B = B << 30 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh76 =
        *in_0.offset((76 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh76 = (*in_0.offset((76 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (76 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (76 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (76 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((76 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (76 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (76 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (76 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    D = (D as ::core::ffi::c_long
        + ((E << 5 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(A ^ B ^ C) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh76 as ::core::ffi::c_long)) as guint32;
    A = A << 30 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh77 =
        *in_0.offset((77 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh77 = (*in_0.offset((77 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (77 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (77 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (77 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((77 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (77 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (77 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (77 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    C = (C as ::core::ffi::c_long
        + ((D << 5 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(E ^ A ^ B) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh77 as ::core::ffi::c_long)) as guint32;
    E = E << 30 as ::core::ffi::c_int | E >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh78 =
        *in_0.offset((78 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh78 = (*in_0.offset((78 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (78 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (78 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (78 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((78 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (78 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (78 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (78 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    B = (B as ::core::ffi::c_long
        + ((C << 5 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(D ^ E ^ A) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh78 as ::core::ffi::c_long)) as guint32;
    D = D << 30 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh79 =
        *in_0.offset((79 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *fresh79 = (*in_0.offset((79 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
        ^ *in_0.offset(
            (79 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (79 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        )
        ^ *in_0.offset(
            (79 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                as isize,
        ))
        << 1 as ::core::ffi::c_int
        | (*in_0.offset((79 as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize)
            ^ *in_0.offset(
                (79 as ::core::ffi::c_int - 14 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (79 as ::core::ffi::c_int - 8 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            )
            ^ *in_0.offset(
                (79 as ::core::ffi::c_int - 3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int)
                    as isize,
            ))
            >> 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    A = (A as ::core::ffi::c_long
        + ((B << 5 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
            .wrapping_add(C ^ D ^ E) as ::core::ffi::c_long
            + 0xca62c1d6 as ::core::ffi::c_long
            + *fresh79 as ::core::ffi::c_long)) as guint32;
    C = C << 30 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 30 as ::core::ffi::c_int;
    let ref mut fresh80 = *buf.offset(0 as ::core::ffi::c_int as isize);
    *fresh80 = (*fresh80).wrapping_add(A);
    let ref mut fresh81 = *buf.offset(1 as ::core::ffi::c_int as isize);
    *fresh81 = (*fresh81).wrapping_add(B);
    let ref mut fresh82 = *buf.offset(2 as ::core::ffi::c_int as isize);
    *fresh82 = (*fresh82).wrapping_add(C);
    let ref mut fresh83 = *buf.offset(3 as ::core::ffi::c_int as isize);
    *fresh83 = (*fresh83).wrapping_add(D);
    let ref mut fresh84 = *buf.offset(4 as ::core::ffi::c_int as isize);
    *fresh84 = (*fresh84).wrapping_add(E);
}
unsafe extern "C" fn safe_c2rust_sha1_sum_update(
    mut sha1: *mut Sha1sum,
    mut buffer: *const guchar,
    mut count: gsize,
) {
    let mut tmp: guint32 = 0;
    let mut dataCount: guint = 0;
    tmp = (*sha1).bits[0 as ::core::ffi::c_int as usize];
    (*sha1).bits[0 as ::core::ffi::c_int as usize] =
        tmp.wrapping_add((count as guint32) << 3 as ::core::ffi::c_int);
    if (*sha1).bits[0 as ::core::ffi::c_int as usize] < tmp {
        (*sha1).bits[1 as ::core::ffi::c_int as usize] =
            (*sha1).bits[1 as ::core::ffi::c_int as usize].wrapping_add(1 as guint32);
    }
    (*sha1).bits[1 as ::core::ffi::c_int as usize] =
        ((*sha1).bits[1 as ::core::ffi::c_int as usize] as gsize)
            .wrapping_add(count >> 29 as ::core::ffi::c_int) as guint32 as guint32;
    dataCount = tmp >> 3 as ::core::ffi::c_int & 0x3f as guint;
    if dataCount != 0 {
        let mut p: *mut guchar =
            (&raw mut (*sha1).data as *mut guint32 as *mut guchar).offset(dataCount as isize);
        dataCount = (SHA1_DATASIZE as guint).wrapping_sub(dataCount);
        if count < dataCount as gsize {
            memcpy(
                p as *mut ::core::ffi::c_void,
                buffer as *const ::core::ffi::c_void,
                count as size_t,
            );
            return;
        }
        memcpy(
            p as *mut ::core::ffi::c_void,
            buffer as *const ::core::ffi::c_void,
            dataCount as size_t,
        );
        safe_c2rust_sha_byte_reverse(&raw mut (*sha1).data as *mut guint32, SHA1_DATASIZE);
        safe_c2rust_sha1_transform(
            &raw mut (*sha1).buf as *mut guint32,
            &raw mut (*sha1).data as *mut guint32,
        );
        buffer = buffer.offset(dataCount as isize);
        count = count.wrapping_sub(dataCount as gsize);
    }
    while count >= SHA1_DATASIZE as gsize {
        memcpy(
            &raw mut (*sha1).data as *mut guint32 as *mut ::core::ffi::c_void,
            buffer as *const ::core::ffi::c_void,
            SHA1_DATASIZE as size_t,
        );
        safe_c2rust_sha_byte_reverse(&raw mut (*sha1).data as *mut guint32, SHA1_DATASIZE);
        safe_c2rust_sha1_transform(
            &raw mut (*sha1).buf as *mut guint32,
            &raw mut (*sha1).data as *mut guint32,
        );
        buffer = buffer.offset(SHA1_DATASIZE as isize);
        count = count.wrapping_sub(SHA1_DATASIZE as gsize);
    }
    memcpy(
        &raw mut (*sha1).data as *mut guint32 as *mut ::core::ffi::c_void,
        buffer as *const ::core::ffi::c_void,
        count as size_t,
    );
}
unsafe extern "C" fn safe_c2rust_sha1_sum_close(mut sha1: *mut Sha1sum) {
    let mut count: gint = 0;
    let mut data_p: *mut guchar = ::core::ptr::null_mut::<guchar>();
    count = ((*sha1).bits[0 as ::core::ffi::c_int as usize] >> 3 as ::core::ffi::c_int
        & 0x3f as guint32) as gint;
    data_p = (&raw mut (*sha1).data as *mut guint32 as *mut guchar).offset(count as isize);
    let fresh93 = data_p;
    data_p = data_p.offset(1);
    *fresh93 = 0x80 as guchar;
    count = SHA1_DATASIZE - 1 as gint - count;
    if count < 8 as ::core::ffi::c_int {
        memset(
            data_p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            count as size_t,
        );
        safe_c2rust_sha_byte_reverse(&raw mut (*sha1).data as *mut guint32, SHA1_DATASIZE);
        safe_c2rust_sha1_transform(
            &raw mut (*sha1).buf as *mut guint32,
            &raw mut (*sha1).data as *mut guint32,
        );
        memset(
            &raw mut (*sha1).data as *mut guint32 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (SHA1_DATASIZE - 8 as ::core::ffi::c_int) as size_t,
        );
    } else {
        memset(
            data_p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (count as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as size_t,
        );
    }
    (*sha1).data[14 as ::core::ffi::c_int as usize] =
        (*sha1).bits[1 as ::core::ffi::c_int as usize];
    (*sha1).data[15 as ::core::ffi::c_int as usize] =
        (*sha1).bits[0 as ::core::ffi::c_int as usize];
    safe_c2rust_sha_byte_reverse(
        &raw mut (*sha1).data as *mut guint32,
        SHA1_DATASIZE - 8 as gint,
    );
    safe_c2rust_sha1_transform(
        &raw mut (*sha1).buf as *mut guint32,
        &raw mut (*sha1).data as *mut guint32,
    );
    safe_c2rust_sha_byte_reverse(&raw mut (*sha1).buf as *mut guint32, SHA1_DIGEST_LEN);
    memcpy(
        &raw mut (*sha1).digest as *mut guchar as *mut ::core::ffi::c_void,
        &raw mut (*sha1).buf as *mut guint32 as *const ::core::ffi::c_void,
        SHA1_DIGEST_LEN as size_t,
    );
    memset(
        &raw mut (*sha1).buf as *mut guint32 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[guint32; 5]>() as size_t,
    );
    memset(
        &raw mut (*sha1).data as *mut guint32 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[guint32; 16]>() as size_t,
    );
}
unsafe extern "C" fn safe_c2rust_sha1_sum_to_string(mut sha1: *mut Sha1sum) -> *mut gchar {
    return safe_c2rust_digest_to_string(
        &raw mut (*sha1).digest as *mut guint8,
        SHA1_DIGEST_LEN as gsize,
    );
}
unsafe extern "C" fn safe_c2rust_sha1_sum_digest(mut sha1: *mut Sha1sum, mut digest: *mut guint8) {
    let mut i: gint = 0;
    i = 0 as ::core::ffi::c_int as gint;
    while i < SHA1_DIGEST_LEN {
        *digest.offset(i as isize) = (*sha1).digest[i as usize] as guint8;
        i += 1;
    }
}
unsafe extern "C" fn safe_c2rust_sha256_sum_init(mut sha256: *mut Sha256sum) {
    (*sha256).buf[0 as ::core::ffi::c_int as usize] = 0x6a09e667 as ::core::ffi::c_int as guint32;
    (*sha256).buf[1 as ::core::ffi::c_int as usize] = 0xbb67ae85 as ::core::ffi::c_uint as guint32;
    (*sha256).buf[2 as ::core::ffi::c_int as usize] = 0x3c6ef372 as ::core::ffi::c_int as guint32;
    (*sha256).buf[3 as ::core::ffi::c_int as usize] = 0xa54ff53a as ::core::ffi::c_uint as guint32;
    (*sha256).buf[4 as ::core::ffi::c_int as usize] = 0x510e527f as ::core::ffi::c_int as guint32;
    (*sha256).buf[5 as ::core::ffi::c_int as usize] = 0x9b05688c as ::core::ffi::c_uint as guint32;
    (*sha256).buf[6 as ::core::ffi::c_int as usize] = 0x1f83d9ab as ::core::ffi::c_int as guint32;
    (*sha256).buf[7 as ::core::ffi::c_int as usize] = 0x5be0cd19 as ::core::ffi::c_int as guint32;
    (*sha256).bits[1 as ::core::ffi::c_int as usize] = 0 as guint32;
    (*sha256).bits[0 as ::core::ffi::c_int as usize] =
        (*sha256).bits[1 as ::core::ffi::c_int as usize];
}
unsafe extern "C" fn safe_c2rust_sha256_transform(mut buf: *mut guint32, mut data: *const guint8) {
    let mut temp1: guint32 = 0;
    let mut temp2: guint32 = 0;
    let mut W: [guint32; 64] = [0; 64];
    let mut A: guint32 = 0;
    let mut B: guint32 = 0;
    let mut C: guint32 = 0;
    let mut D: guint32 = 0;
    let mut E: guint32 = 0;
    let mut F: guint32 = 0;
    let mut G: guint32 = 0;
    let mut H: guint32 = 0;
    W[0 as ::core::ffi::c_int as usize] = (*data.offset(0 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[1 as ::core::ffi::c_int as usize] = (*data.offset(4 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[2 as ::core::ffi::c_int as usize] = (*data.offset(8 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[3 as ::core::ffi::c_int as usize] = (*data.offset(12 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[4 as ::core::ffi::c_int as usize] = (*data.offset(16 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[5 as ::core::ffi::c_int as usize] = (*data.offset(20 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((20 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((20 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((20 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[6 as ::core::ffi::c_int as usize] = (*data.offset(24 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((24 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((24 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((24 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[7 as ::core::ffi::c_int as usize] = (*data.offset(28 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((28 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((28 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((28 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[8 as ::core::ffi::c_int as usize] = (*data.offset(32 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[9 as ::core::ffi::c_int as usize] = (*data.offset(36 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[10 as ::core::ffi::c_int as usize] = (*data.offset(40 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((40 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((40 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((40 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[11 as ::core::ffi::c_int as usize] = (*data.offset(44 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((44 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((44 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((44 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[12 as ::core::ffi::c_int as usize] = (*data.offset(48 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[13 as ::core::ffi::c_int as usize] = (*data.offset(52 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((52 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((52 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((52 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[14 as ::core::ffi::c_int as usize] = (*data.offset(56 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((56 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((56 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((56 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    W[15 as ::core::ffi::c_int as usize] = (*data.offset(60 as ::core::ffi::c_int as isize)
        as guint32)
        << 24 as ::core::ffi::c_int
        | (*data.offset((60 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as guint32)
            << 16 as ::core::ffi::c_int
        | (*data.offset((60 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as guint32)
            << 8 as ::core::ffi::c_int
        | *data.offset((60 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) as guint32;
    A = *buf.offset(0 as ::core::ffi::c_int as isize);
    B = *buf.offset(1 as ::core::ffi::c_int as isize);
    C = *buf.offset(2 as ::core::ffi::c_int as isize);
    D = *buf.offset(3 as ::core::ffi::c_int as isize);
    E = *buf.offset(4 as ::core::ffi::c_int as isize);
    F = *buf.offset(5 as ::core::ffi::c_int as isize);
    G = *buf.offset(6 as ::core::ffi::c_int as isize);
    H = *buf.offset(7 as ::core::ffi::c_int as isize);
    temp1 = H
        .wrapping_add(
            ((E & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | E << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(G ^ E & (F ^ G))
        .wrapping_add(0x428a2f98 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[0 as ::core::ffi::c_int as usize]);
    temp2 = (((A & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | A << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(A & B | C & (A | B));
    D = D.wrapping_add(temp1);
    H = temp1.wrapping_add(temp2);
    temp1 = G
        .wrapping_add(
            ((D & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | D << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(F ^ D & (E ^ F))
        .wrapping_add(0x71374491 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[1 as ::core::ffi::c_int as usize]);
    temp2 = (((H & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | H << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(H & A | B & (H | A));
    C = C.wrapping_add(temp1);
    G = temp1.wrapping_add(temp2);
    temp1 = F
        .wrapping_add(
            ((C & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | C << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(E ^ C & (D ^ E))
        .wrapping_add(0xb5c0fbcf as guint32)
        .wrapping_add(W[2 as ::core::ffi::c_int as usize]);
    temp2 = (((G & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | G << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(G & H | A & (G | H));
    B = B.wrapping_add(temp1);
    F = temp1.wrapping_add(temp2);
    temp1 = E
        .wrapping_add(
            ((B & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | B << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(D ^ B & (C ^ D))
        .wrapping_add(0xe9b5dba5 as guint32)
        .wrapping_add(W[3 as ::core::ffi::c_int as usize]);
    temp2 = (((F & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | F << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(F & G | H & (F | G));
    A = A.wrapping_add(temp1);
    E = temp1.wrapping_add(temp2);
    temp1 = D
        .wrapping_add(
            ((A & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | A << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(C ^ A & (B ^ C))
        .wrapping_add(0x3956c25b as ::core::ffi::c_int as guint32)
        .wrapping_add(W[4 as ::core::ffi::c_int as usize]);
    temp2 = (((E & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | E << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(E & F | G & (E | F));
    H = H.wrapping_add(temp1);
    D = temp1.wrapping_add(temp2);
    temp1 = C
        .wrapping_add(
            ((H & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | H << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(B ^ H & (A ^ B))
        .wrapping_add(0x59f111f1 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[5 as ::core::ffi::c_int as usize]);
    temp2 = (((D & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | D << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(D & E | F & (D | E));
    G = G.wrapping_add(temp1);
    C = temp1.wrapping_add(temp2);
    temp1 = B
        .wrapping_add(
            ((G & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | G << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(A ^ G & (H ^ A))
        .wrapping_add(0x923f82a4 as guint32)
        .wrapping_add(W[6 as ::core::ffi::c_int as usize]);
    temp2 = (((C & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | C << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(C & D | E & (C | D));
    F = F.wrapping_add(temp1);
    B = temp1.wrapping_add(temp2);
    temp1 = A
        .wrapping_add(
            ((F & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | F << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(H ^ F & (G ^ H))
        .wrapping_add(0xab1c5ed5 as guint32)
        .wrapping_add(W[7 as ::core::ffi::c_int as usize]);
    temp2 = (((B & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | B << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(B & C | D & (B | C));
    E = E.wrapping_add(temp1);
    A = temp1.wrapping_add(temp2);
    temp1 = H
        .wrapping_add(
            ((E & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | E << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(G ^ E & (F ^ G))
        .wrapping_add(0xd807aa98 as guint32)
        .wrapping_add(W[8 as ::core::ffi::c_int as usize]);
    temp2 = (((A & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | A << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(A & B | C & (A | B));
    D = D.wrapping_add(temp1);
    H = temp1.wrapping_add(temp2);
    temp1 = G
        .wrapping_add(
            ((D & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | D << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(F ^ D & (E ^ F))
        .wrapping_add(0x12835b01 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[9 as ::core::ffi::c_int as usize]);
    temp2 = (((H & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | H << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(H & A | B & (H | A));
    C = C.wrapping_add(temp1);
    G = temp1.wrapping_add(temp2);
    temp1 = F
        .wrapping_add(
            ((C & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | C << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(E ^ C & (D ^ E))
        .wrapping_add(0x243185be as ::core::ffi::c_int as guint32)
        .wrapping_add(W[10 as ::core::ffi::c_int as usize]);
    temp2 = (((G & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | G << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(G & H | A & (G | H));
    B = B.wrapping_add(temp1);
    F = temp1.wrapping_add(temp2);
    temp1 = E
        .wrapping_add(
            ((B & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | B << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(D ^ B & (C ^ D))
        .wrapping_add(0x550c7dc3 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[11 as ::core::ffi::c_int as usize]);
    temp2 = (((F & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | F << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(F & G | H & (F | G));
    A = A.wrapping_add(temp1);
    E = temp1.wrapping_add(temp2);
    temp1 = D
        .wrapping_add(
            ((A & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | A << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(C ^ A & (B ^ C))
        .wrapping_add(0x72be5d74 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[12 as ::core::ffi::c_int as usize]);
    temp2 = (((E & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | E << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(E & F | G & (E | F));
    H = H.wrapping_add(temp1);
    D = temp1.wrapping_add(temp2);
    temp1 = C
        .wrapping_add(
            ((H & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | H << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(B ^ H & (A ^ B))
        .wrapping_add(0x80deb1fe as guint32)
        .wrapping_add(W[13 as ::core::ffi::c_int as usize]);
    temp2 = (((D & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | D << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(D & E | F & (D | E));
    G = G.wrapping_add(temp1);
    C = temp1.wrapping_add(temp2);
    temp1 = B
        .wrapping_add(
            ((G & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | G << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(A ^ G & (H ^ A))
        .wrapping_add(0x9bdc06a7 as guint32)
        .wrapping_add(W[14 as ::core::ffi::c_int as usize]);
    temp2 = (((C & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | C << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(C & D | E & (C | D));
    F = F.wrapping_add(temp1);
    B = temp1.wrapping_add(temp2);
    temp1 = A
        .wrapping_add(
            ((F & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | F << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(H ^ F & (G ^ H))
        .wrapping_add(0xc19bf174 as guint32)
        .wrapping_add(W[15 as ::core::ffi::c_int as usize]);
    temp2 = (((B & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | B << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(B & C | D & (B | C));
    E = E.wrapping_add(temp1);
    A = temp1.wrapping_add(temp2);
    W[16 as ::core::ffi::c_int as usize] = (((W
        [(16 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(16 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(16 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(16 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(16 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(16 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(16 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(16 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(16 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(16 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(16 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(16 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = H
        .wrapping_add(
            ((E & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | E << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(G ^ E & (F ^ G))
        .wrapping_add(0xe49b69c1 as guint32)
        .wrapping_add(W[16 as ::core::ffi::c_int as usize]);
    temp2 = (((A & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | A << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(A & B | C & (A | B));
    D = D.wrapping_add(temp1);
    H = temp1.wrapping_add(temp2);
    W[17 as ::core::ffi::c_int as usize] = (((W
        [(17 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(17 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(17 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(17 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(17 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(17 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(17 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(17 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(17 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(17 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(17 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(17 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = G
        .wrapping_add(
            ((D & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | D << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(F ^ D & (E ^ F))
        .wrapping_add(0xefbe4786 as guint32)
        .wrapping_add(W[17 as ::core::ffi::c_int as usize]);
    temp2 = (((H & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | H << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(H & A | B & (H | A));
    C = C.wrapping_add(temp1);
    G = temp1.wrapping_add(temp2);
    W[18 as ::core::ffi::c_int as usize] = (((W
        [(18 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(18 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(18 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(18 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(18 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(18 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(18 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(18 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(18 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(18 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(18 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(18 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = F
        .wrapping_add(
            ((C & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | C << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(E ^ C & (D ^ E))
        .wrapping_add(0xfc19dc6 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[18 as ::core::ffi::c_int as usize]);
    temp2 = (((G & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | G << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(G & H | A & (G | H));
    B = B.wrapping_add(temp1);
    F = temp1.wrapping_add(temp2);
    W[19 as ::core::ffi::c_int as usize] = (((W
        [(19 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(19 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(19 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(19 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(19 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(19 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(19 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(19 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(19 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(19 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(19 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(19 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = E
        .wrapping_add(
            ((B & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | B << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(D ^ B & (C ^ D))
        .wrapping_add(0x240ca1cc as ::core::ffi::c_int as guint32)
        .wrapping_add(W[19 as ::core::ffi::c_int as usize]);
    temp2 = (((F & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | F << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(F & G | H & (F | G));
    A = A.wrapping_add(temp1);
    E = temp1.wrapping_add(temp2);
    W[20 as ::core::ffi::c_int as usize] = (((W
        [(20 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(20 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(20 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(20 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(20 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(20 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(20 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(20 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(20 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(20 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(20 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(20 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = D
        .wrapping_add(
            ((A & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | A << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(C ^ A & (B ^ C))
        .wrapping_add(0x2de92c6f as ::core::ffi::c_int as guint32)
        .wrapping_add(W[20 as ::core::ffi::c_int as usize]);
    temp2 = (((E & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | E << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(E & F | G & (E | F));
    H = H.wrapping_add(temp1);
    D = temp1.wrapping_add(temp2);
    W[21 as ::core::ffi::c_int as usize] = (((W
        [(21 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(21 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(21 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(21 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(21 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(21 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(21 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(21 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(21 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(21 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(21 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(21 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = C
        .wrapping_add(
            ((H & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | H << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(B ^ H & (A ^ B))
        .wrapping_add(0x4a7484aa as ::core::ffi::c_int as guint32)
        .wrapping_add(W[21 as ::core::ffi::c_int as usize]);
    temp2 = (((D & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | D << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(D & E | F & (D | E));
    G = G.wrapping_add(temp1);
    C = temp1.wrapping_add(temp2);
    W[22 as ::core::ffi::c_int as usize] = (((W
        [(22 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(22 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(22 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(22 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(22 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(22 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(22 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(22 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(22 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(22 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(22 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(22 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = B
        .wrapping_add(
            ((G & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | G << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(A ^ G & (H ^ A))
        .wrapping_add(0x5cb0a9dc as ::core::ffi::c_int as guint32)
        .wrapping_add(W[22 as ::core::ffi::c_int as usize]);
    temp2 = (((C & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | C << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(C & D | E & (C | D));
    F = F.wrapping_add(temp1);
    B = temp1.wrapping_add(temp2);
    W[23 as ::core::ffi::c_int as usize] = (((W
        [(23 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(23 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(23 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(23 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(23 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(23 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(23 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(23 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(23 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(23 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(23 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(23 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = A
        .wrapping_add(
            ((F & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | F << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(H ^ F & (G ^ H))
        .wrapping_add(0x76f988da as ::core::ffi::c_int as guint32)
        .wrapping_add(W[23 as ::core::ffi::c_int as usize]);
    temp2 = (((B & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | B << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(B & C | D & (B | C));
    E = E.wrapping_add(temp1);
    A = temp1.wrapping_add(temp2);
    W[24 as ::core::ffi::c_int as usize] = (((W
        [(24 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(24 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(24 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(24 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(24 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(24 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(24 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(24 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(24 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(24 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(24 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(24 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = H
        .wrapping_add(
            ((E & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | E << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(G ^ E & (F ^ G))
        .wrapping_add(0x983e5152 as guint32)
        .wrapping_add(W[24 as ::core::ffi::c_int as usize]);
    temp2 = (((A & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | A << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(A & B | C & (A | B));
    D = D.wrapping_add(temp1);
    H = temp1.wrapping_add(temp2);
    W[25 as ::core::ffi::c_int as usize] = (((W
        [(25 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(25 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(25 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(25 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(25 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(25 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(25 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(25 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(25 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(25 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(25 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(25 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = G
        .wrapping_add(
            ((D & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | D << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(F ^ D & (E ^ F))
        .wrapping_add(0xa831c66d as guint32)
        .wrapping_add(W[25 as ::core::ffi::c_int as usize]);
    temp2 = (((H & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | H << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(H & A | B & (H | A));
    C = C.wrapping_add(temp1);
    G = temp1.wrapping_add(temp2);
    W[26 as ::core::ffi::c_int as usize] = (((W
        [(26 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(26 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(26 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(26 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(26 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(26 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(26 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(26 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(26 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(26 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(26 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(26 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = F
        .wrapping_add(
            ((C & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | C << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(E ^ C & (D ^ E))
        .wrapping_add(0xb00327c8 as guint32)
        .wrapping_add(W[26 as ::core::ffi::c_int as usize]);
    temp2 = (((G & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | G << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(G & H | A & (G | H));
    B = B.wrapping_add(temp1);
    F = temp1.wrapping_add(temp2);
    W[27 as ::core::ffi::c_int as usize] = (((W
        [(27 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(27 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(27 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(27 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(27 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(27 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(27 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(27 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(27 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(27 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(27 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(27 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = E
        .wrapping_add(
            ((B & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | B << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(D ^ B & (C ^ D))
        .wrapping_add(0xbf597fc7 as guint32)
        .wrapping_add(W[27 as ::core::ffi::c_int as usize]);
    temp2 = (((F & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | F << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(F & G | H & (F | G));
    A = A.wrapping_add(temp1);
    E = temp1.wrapping_add(temp2);
    W[28 as ::core::ffi::c_int as usize] = (((W
        [(28 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(28 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(28 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(28 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(28 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(28 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(28 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(28 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(28 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(28 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(28 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(28 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = D
        .wrapping_add(
            ((A & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | A << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(C ^ A & (B ^ C))
        .wrapping_add(0xc6e00bf3 as guint32)
        .wrapping_add(W[28 as ::core::ffi::c_int as usize]);
    temp2 = (((E & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | E << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(E & F | G & (E | F));
    H = H.wrapping_add(temp1);
    D = temp1.wrapping_add(temp2);
    W[29 as ::core::ffi::c_int as usize] = (((W
        [(29 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(29 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(29 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(29 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(29 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(29 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(29 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(29 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(29 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(29 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(29 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(29 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = C
        .wrapping_add(
            ((H & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | H << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(B ^ H & (A ^ B))
        .wrapping_add(0xd5a79147 as guint32)
        .wrapping_add(W[29 as ::core::ffi::c_int as usize]);
    temp2 = (((D & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | D << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(D & E | F & (D | E));
    G = G.wrapping_add(temp1);
    C = temp1.wrapping_add(temp2);
    W[30 as ::core::ffi::c_int as usize] = (((W
        [(30 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(30 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(30 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(30 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(30 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(30 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(30 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(30 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(30 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(30 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(30 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(30 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = B
        .wrapping_add(
            ((G & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | G << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(A ^ G & (H ^ A))
        .wrapping_add(0x6ca6351 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[30 as ::core::ffi::c_int as usize]);
    temp2 = (((C & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | C << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(C & D | E & (C | D));
    F = F.wrapping_add(temp1);
    B = temp1.wrapping_add(temp2);
    W[31 as ::core::ffi::c_int as usize] = (((W
        [(31 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(31 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(31 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(31 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(31 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(31 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(31 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(31 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(31 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(31 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(31 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(31 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = A
        .wrapping_add(
            ((F & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | F << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(H ^ F & (G ^ H))
        .wrapping_add(0x14292967 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[31 as ::core::ffi::c_int as usize]);
    temp2 = (((B & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | B << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(B & C | D & (B | C));
    E = E.wrapping_add(temp1);
    A = temp1.wrapping_add(temp2);
    W[32 as ::core::ffi::c_int as usize] = (((W
        [(32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = H
        .wrapping_add(
            ((E & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | E << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(G ^ E & (F ^ G))
        .wrapping_add(0x27b70a85 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[32 as ::core::ffi::c_int as usize]);
    temp2 = (((A & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | A << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(A & B | C & (A | B));
    D = D.wrapping_add(temp1);
    H = temp1.wrapping_add(temp2);
    W[33 as ::core::ffi::c_int as usize] = (((W
        [(33 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(33 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(33 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(33 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(33 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(33 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(33 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(33 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(33 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(33 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(33 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(33 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = G
        .wrapping_add(
            ((D & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | D << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(F ^ D & (E ^ F))
        .wrapping_add(0x2e1b2138 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[33 as ::core::ffi::c_int as usize]);
    temp2 = (((H & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | H << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(H & A | B & (H | A));
    C = C.wrapping_add(temp1);
    G = temp1.wrapping_add(temp2);
    W[34 as ::core::ffi::c_int as usize] = (((W
        [(34 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(34 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(34 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(34 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(34 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(34 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(34 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(34 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(34 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(34 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(34 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(34 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = F
        .wrapping_add(
            ((C & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | C << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(E ^ C & (D ^ E))
        .wrapping_add(0x4d2c6dfc as ::core::ffi::c_int as guint32)
        .wrapping_add(W[34 as ::core::ffi::c_int as usize]);
    temp2 = (((G & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | G << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(G & H | A & (G | H));
    B = B.wrapping_add(temp1);
    F = temp1.wrapping_add(temp2);
    W[35 as ::core::ffi::c_int as usize] = (((W
        [(35 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(35 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(35 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(35 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(35 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(35 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(35 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(35 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(35 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(35 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(35 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(35 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = E
        .wrapping_add(
            ((B & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | B << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(D ^ B & (C ^ D))
        .wrapping_add(0x53380d13 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[35 as ::core::ffi::c_int as usize]);
    temp2 = (((F & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | F << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(F & G | H & (F | G));
    A = A.wrapping_add(temp1);
    E = temp1.wrapping_add(temp2);
    W[36 as ::core::ffi::c_int as usize] = (((W
        [(36 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(36 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(36 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(36 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(36 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(36 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(36 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(36 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(36 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(36 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(36 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(36 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = D
        .wrapping_add(
            ((A & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | A << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(C ^ A & (B ^ C))
        .wrapping_add(0x650a7354 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[36 as ::core::ffi::c_int as usize]);
    temp2 = (((E & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | E << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(E & F | G & (E | F));
    H = H.wrapping_add(temp1);
    D = temp1.wrapping_add(temp2);
    W[37 as ::core::ffi::c_int as usize] = (((W
        [(37 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(37 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(37 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(37 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(37 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(37 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(37 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(37 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(37 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(37 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(37 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(37 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = C
        .wrapping_add(
            ((H & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | H << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(B ^ H & (A ^ B))
        .wrapping_add(0x766a0abb as ::core::ffi::c_int as guint32)
        .wrapping_add(W[37 as ::core::ffi::c_int as usize]);
    temp2 = (((D & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | D << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(D & E | F & (D | E));
    G = G.wrapping_add(temp1);
    C = temp1.wrapping_add(temp2);
    W[38 as ::core::ffi::c_int as usize] = (((W
        [(38 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(38 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(38 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(38 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(38 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(38 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(38 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(38 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(38 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(38 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(38 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(38 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = B
        .wrapping_add(
            ((G & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | G << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(A ^ G & (H ^ A))
        .wrapping_add(0x81c2c92e as guint32)
        .wrapping_add(W[38 as ::core::ffi::c_int as usize]);
    temp2 = (((C & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | C << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(C & D | E & (C | D));
    F = F.wrapping_add(temp1);
    B = temp1.wrapping_add(temp2);
    W[39 as ::core::ffi::c_int as usize] = (((W
        [(39 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(39 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(39 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(39 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(39 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(39 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(39 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(39 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(39 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(39 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(39 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(39 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = A
        .wrapping_add(
            ((F & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | F << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(H ^ F & (G ^ H))
        .wrapping_add(0x92722c85 as guint32)
        .wrapping_add(W[39 as ::core::ffi::c_int as usize]);
    temp2 = (((B & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | B << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(B & C | D & (B | C));
    E = E.wrapping_add(temp1);
    A = temp1.wrapping_add(temp2);
    W[40 as ::core::ffi::c_int as usize] = (((W
        [(40 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(40 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(40 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(40 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(40 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(40 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(40 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(40 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(40 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(40 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(40 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(40 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = H
        .wrapping_add(
            ((E & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | E << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(G ^ E & (F ^ G))
        .wrapping_add(0xa2bfe8a1 as guint32)
        .wrapping_add(W[40 as ::core::ffi::c_int as usize]);
    temp2 = (((A & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | A << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(A & B | C & (A | B));
    D = D.wrapping_add(temp1);
    H = temp1.wrapping_add(temp2);
    W[41 as ::core::ffi::c_int as usize] = (((W
        [(41 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(41 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(41 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(41 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(41 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(41 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(41 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(41 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(41 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(41 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(41 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(41 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = G
        .wrapping_add(
            ((D & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | D << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(F ^ D & (E ^ F))
        .wrapping_add(0xa81a664b as guint32)
        .wrapping_add(W[41 as ::core::ffi::c_int as usize]);
    temp2 = (((H & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | H << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(H & A | B & (H | A));
    C = C.wrapping_add(temp1);
    G = temp1.wrapping_add(temp2);
    W[42 as ::core::ffi::c_int as usize] = (((W
        [(42 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(42 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(42 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(42 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(42 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(42 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(42 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(42 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(42 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(42 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(42 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(42 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = F
        .wrapping_add(
            ((C & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | C << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(E ^ C & (D ^ E))
        .wrapping_add(0xc24b8b70 as guint32)
        .wrapping_add(W[42 as ::core::ffi::c_int as usize]);
    temp2 = (((G & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | G << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(G & H | A & (G | H));
    B = B.wrapping_add(temp1);
    F = temp1.wrapping_add(temp2);
    W[43 as ::core::ffi::c_int as usize] = (((W
        [(43 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(43 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(43 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(43 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(43 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(43 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(43 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(43 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(43 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(43 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(43 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(43 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = E
        .wrapping_add(
            ((B & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | B << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(D ^ B & (C ^ D))
        .wrapping_add(0xc76c51a3 as guint32)
        .wrapping_add(W[43 as ::core::ffi::c_int as usize]);
    temp2 = (((F & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | F << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(F & G | H & (F | G));
    A = A.wrapping_add(temp1);
    E = temp1.wrapping_add(temp2);
    W[44 as ::core::ffi::c_int as usize] = (((W
        [(44 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(44 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(44 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(44 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(44 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(44 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(44 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(44 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(44 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(44 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(44 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(44 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = D
        .wrapping_add(
            ((A & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | A << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(C ^ A & (B ^ C))
        .wrapping_add(0xd192e819 as guint32)
        .wrapping_add(W[44 as ::core::ffi::c_int as usize]);
    temp2 = (((E & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | E << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(E & F | G & (E | F));
    H = H.wrapping_add(temp1);
    D = temp1.wrapping_add(temp2);
    W[45 as ::core::ffi::c_int as usize] = (((W
        [(45 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(45 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(45 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(45 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(45 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(45 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(45 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(45 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(45 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(45 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(45 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(45 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = C
        .wrapping_add(
            ((H & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | H << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(B ^ H & (A ^ B))
        .wrapping_add(0xd6990624 as guint32)
        .wrapping_add(W[45 as ::core::ffi::c_int as usize]);
    temp2 = (((D & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | D << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(D & E | F & (D | E));
    G = G.wrapping_add(temp1);
    C = temp1.wrapping_add(temp2);
    W[46 as ::core::ffi::c_int as usize] = (((W
        [(46 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(46 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(46 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(46 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(46 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(46 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(46 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(46 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(46 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(46 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(46 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(46 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = B
        .wrapping_add(
            ((G & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | G << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(A ^ G & (H ^ A))
        .wrapping_add(0xf40e3585 as guint32)
        .wrapping_add(W[46 as ::core::ffi::c_int as usize]);
    temp2 = (((C & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | C << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(C & D | E & (C | D));
    F = F.wrapping_add(temp1);
    B = temp1.wrapping_add(temp2);
    W[47 as ::core::ffi::c_int as usize] = (((W
        [(47 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(47 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(47 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(47 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(47 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(47 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(47 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(47 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(47 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(47 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(47 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(47 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = A
        .wrapping_add(
            ((F & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | F << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(H ^ F & (G ^ H))
        .wrapping_add(0x106aa070 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[47 as ::core::ffi::c_int as usize]);
    temp2 = (((B & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | B << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(B & C | D & (B | C));
    E = E.wrapping_add(temp1);
    A = temp1.wrapping_add(temp2);
    W[48 as ::core::ffi::c_int as usize] = (((W
        [(48 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(48 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(48 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(48 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(48 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(48 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(48 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(48 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(48 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(48 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(48 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(48 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = H
        .wrapping_add(
            ((E & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | E << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(G ^ E & (F ^ G))
        .wrapping_add(0x19a4c116 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[48 as ::core::ffi::c_int as usize]);
    temp2 = (((A & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | A << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(A & B | C & (A | B));
    D = D.wrapping_add(temp1);
    H = temp1.wrapping_add(temp2);
    W[49 as ::core::ffi::c_int as usize] = (((W
        [(49 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(49 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(49 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(49 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(49 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(49 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(49 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(49 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(49 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(49 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(49 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(49 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = G
        .wrapping_add(
            ((D & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | D << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(F ^ D & (E ^ F))
        .wrapping_add(0x1e376c08 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[49 as ::core::ffi::c_int as usize]);
    temp2 = (((H & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | H << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(H & A | B & (H | A));
    C = C.wrapping_add(temp1);
    G = temp1.wrapping_add(temp2);
    W[50 as ::core::ffi::c_int as usize] = (((W
        [(50 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(50 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(50 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(50 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(50 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(50 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(50 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(50 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(50 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(50 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(50 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(50 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = F
        .wrapping_add(
            ((C & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | C << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(E ^ C & (D ^ E))
        .wrapping_add(0x2748774c as ::core::ffi::c_int as guint32)
        .wrapping_add(W[50 as ::core::ffi::c_int as usize]);
    temp2 = (((G & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | G << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(G & H | A & (G | H));
    B = B.wrapping_add(temp1);
    F = temp1.wrapping_add(temp2);
    W[51 as ::core::ffi::c_int as usize] = (((W
        [(51 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(51 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(51 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(51 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(51 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(51 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(51 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(51 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(51 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(51 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(51 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(51 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = E
        .wrapping_add(
            ((B & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | B << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(D ^ B & (C ^ D))
        .wrapping_add(0x34b0bcb5 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[51 as ::core::ffi::c_int as usize]);
    temp2 = (((F & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | F << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(F & G | H & (F | G));
    A = A.wrapping_add(temp1);
    E = temp1.wrapping_add(temp2);
    W[52 as ::core::ffi::c_int as usize] = (((W
        [(52 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(52 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(52 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(52 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(52 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(52 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(52 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(52 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(52 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(52 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(52 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(52 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = D
        .wrapping_add(
            ((A & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | A << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(C ^ A & (B ^ C))
        .wrapping_add(0x391c0cb3 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[52 as ::core::ffi::c_int as usize]);
    temp2 = (((E & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | E << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(E & F | G & (E | F));
    H = H.wrapping_add(temp1);
    D = temp1.wrapping_add(temp2);
    W[53 as ::core::ffi::c_int as usize] = (((W
        [(53 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(53 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(53 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(53 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(53 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(53 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(53 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(53 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(53 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(53 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(53 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(53 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = C
        .wrapping_add(
            ((H & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | H << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(B ^ H & (A ^ B))
        .wrapping_add(0x4ed8aa4a as ::core::ffi::c_int as guint32)
        .wrapping_add(W[53 as ::core::ffi::c_int as usize]);
    temp2 = (((D & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | D << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(D & E | F & (D | E));
    G = G.wrapping_add(temp1);
    C = temp1.wrapping_add(temp2);
    W[54 as ::core::ffi::c_int as usize] = (((W
        [(54 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(54 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(54 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(54 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(54 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(54 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(54 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(54 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(54 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(54 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(54 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(54 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = B
        .wrapping_add(
            ((G & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | G << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(A ^ G & (H ^ A))
        .wrapping_add(0x5b9cca4f as ::core::ffi::c_int as guint32)
        .wrapping_add(W[54 as ::core::ffi::c_int as usize]);
    temp2 = (((C & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | C << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(C & D | E & (C | D));
    F = F.wrapping_add(temp1);
    B = temp1.wrapping_add(temp2);
    W[55 as ::core::ffi::c_int as usize] = (((W
        [(55 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(55 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(55 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(55 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(55 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(55 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(55 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(55 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(55 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(55 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(55 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(55 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = A
        .wrapping_add(
            ((F & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | F << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(H ^ F & (G ^ H))
        .wrapping_add(0x682e6ff3 as ::core::ffi::c_int as guint32)
        .wrapping_add(W[55 as ::core::ffi::c_int as usize]);
    temp2 = (((B & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | B << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(B & C | D & (B | C));
    E = E.wrapping_add(temp1);
    A = temp1.wrapping_add(temp2);
    W[56 as ::core::ffi::c_int as usize] = (((W
        [(56 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(56 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(56 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(56 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(56 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(56 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(56 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(56 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(56 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(56 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(56 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(56 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = H
        .wrapping_add(
            ((E & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | E << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((E & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | E << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(G ^ E & (F ^ G))
        .wrapping_add(0x748f82ee as ::core::ffi::c_int as guint32)
        .wrapping_add(W[56 as ::core::ffi::c_int as usize]);
    temp2 = (((A & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | A << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((A & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | A << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(A & B | C & (A | B));
    D = D.wrapping_add(temp1);
    H = temp1.wrapping_add(temp2);
    W[57 as ::core::ffi::c_int as usize] = (((W
        [(57 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(57 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(57 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(57 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(57 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(57 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(57 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(57 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(57 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(57 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(57 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(57 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = G
        .wrapping_add(
            ((D & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | D << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((D & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | D << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(F ^ D & (E ^ F))
        .wrapping_add(0x78a5636f as ::core::ffi::c_int as guint32)
        .wrapping_add(W[57 as ::core::ffi::c_int as usize]);
    temp2 = (((H & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | H << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((H & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | H << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(H & A | B & (H | A));
    C = C.wrapping_add(temp1);
    G = temp1.wrapping_add(temp2);
    W[58 as ::core::ffi::c_int as usize] = (((W
        [(58 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(58 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(58 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(58 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(58 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(58 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(58 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(58 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(58 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(58 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(58 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(58 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = F
        .wrapping_add(
            ((C & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | C << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((C & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | C << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(E ^ C & (D ^ E))
        .wrapping_add(0x84c87814 as guint32)
        .wrapping_add(W[58 as ::core::ffi::c_int as usize]);
    temp2 = (((G & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | G << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((G & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | G << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(G & H | A & (G | H));
    B = B.wrapping_add(temp1);
    F = temp1.wrapping_add(temp2);
    W[59 as ::core::ffi::c_int as usize] = (((W
        [(59 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(59 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(59 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(59 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(59 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(59 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(59 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(59 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(59 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(59 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(59 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(59 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = E
        .wrapping_add(
            ((B & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | B << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((B & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | B << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(D ^ B & (C ^ D))
        .wrapping_add(0x8cc70208 as guint32)
        .wrapping_add(W[59 as ::core::ffi::c_int as usize]);
    temp2 = (((F & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | F << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((F & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | F << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(F & G | H & (F | G));
    A = A.wrapping_add(temp1);
    E = temp1.wrapping_add(temp2);
    W[60 as ::core::ffi::c_int as usize] = (((W
        [(60 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(60 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(60 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(60 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(60 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(60 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(60 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(60 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(60 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(60 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(60 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(60 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = D
        .wrapping_add(
            ((A & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | A << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((A & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | A << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(C ^ A & (B ^ C))
        .wrapping_add(0x90befffa as guint32)
        .wrapping_add(W[60 as ::core::ffi::c_int as usize]);
    temp2 = (((E & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | E << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((E & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | E << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(E & F | G & (E | F));
    H = H.wrapping_add(temp1);
    D = temp1.wrapping_add(temp2);
    W[61 as ::core::ffi::c_int as usize] = (((W
        [(61 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(61 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(61 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(61 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(61 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(61 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(61 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(61 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(61 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(61 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(61 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(61 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = C
        .wrapping_add(
            ((H & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | H << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((H & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | H << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(B ^ H & (A ^ B))
        .wrapping_add(0xa4506ceb as guint32)
        .wrapping_add(W[61 as ::core::ffi::c_int as usize]);
    temp2 = (((D & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | D << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((D & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | D << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(D & E | F & (D | E));
    G = G.wrapping_add(temp1);
    C = temp1.wrapping_add(temp2);
    W[62 as ::core::ffi::c_int as usize] = (((W
        [(62 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(62 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(62 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(62 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(62 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(62 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(62 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(62 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(62 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(62 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(62 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(62 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = B
        .wrapping_add(
            ((G & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | G << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((G & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | G << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(A ^ G & (H ^ A))
        .wrapping_add(0xbef9a3f7 as guint32)
        .wrapping_add(W[62 as ::core::ffi::c_int as usize]);
    temp2 = (((C & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | C << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((C & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | C << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(C & D | E & (C | D));
    F = F.wrapping_add(temp1);
    B = temp1.wrapping_add(temp2);
    W[63 as ::core::ffi::c_int as usize] = (((W
        [(63 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
        & 0xffffffff as guint32)
        >> 17 as ::core::ffi::c_int
        | W[(63 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            << 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int)
        ^ ((W[(63 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 19 as ::core::ffi::c_int
            | W[(63 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                << 32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
        ^ (W[(63 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
            & 0xffffffff as guint32)
            >> 10 as ::core::ffi::c_int)
        .wrapping_add(W[(63 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
        .wrapping_add(
            ((W[(63 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                & 0xffffffff as guint32)
                >> 7 as ::core::ffi::c_int
                | W[(63 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    << 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                ^ ((W[(63 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 18 as ::core::ffi::c_int
                    | W[(63 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        << 32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                ^ (W[(63 as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                    & 0xffffffff as guint32)
                    >> 3 as ::core::ffi::c_int,
        )
        .wrapping_add(W[(63 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
    temp1 = A
        .wrapping_add(
            ((F & 0xffffffff as guint32) >> 6 as ::core::ffi::c_int
                | F << 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 11 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int)
                ^ ((F & 0xffffffff as guint32) >> 25 as ::core::ffi::c_int
                    | F << 32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int),
        )
        .wrapping_add(H ^ F & (G ^ H))
        .wrapping_add(0xc67178f2 as guint32)
        .wrapping_add(W[63 as ::core::ffi::c_int as usize]);
    temp2 = (((B & 0xffffffff as guint32) >> 2 as ::core::ffi::c_int
        | B << 32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 13 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int)
        ^ ((B & 0xffffffff as guint32) >> 22 as ::core::ffi::c_int
            | B << 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int))
        .wrapping_add(B & C | D & (B | C));
    E = E.wrapping_add(temp1);
    A = temp1.wrapping_add(temp2);
    let ref mut fresh8 = *buf.offset(0 as ::core::ffi::c_int as isize);
    *fresh8 = (*fresh8).wrapping_add(A);
    let ref mut fresh9 = *buf.offset(1 as ::core::ffi::c_int as isize);
    *fresh9 = (*fresh9).wrapping_add(B);
    let ref mut fresh10 = *buf.offset(2 as ::core::ffi::c_int as isize);
    *fresh10 = (*fresh10).wrapping_add(C);
    let ref mut fresh11 = *buf.offset(3 as ::core::ffi::c_int as isize);
    *fresh11 = (*fresh11).wrapping_add(D);
    let ref mut fresh12 = *buf.offset(4 as ::core::ffi::c_int as isize);
    *fresh12 = (*fresh12).wrapping_add(E);
    let ref mut fresh13 = *buf.offset(5 as ::core::ffi::c_int as isize);
    *fresh13 = (*fresh13).wrapping_add(F);
    let ref mut fresh14 = *buf.offset(6 as ::core::ffi::c_int as isize);
    *fresh14 = (*fresh14).wrapping_add(G);
    let ref mut fresh15 = *buf.offset(7 as ::core::ffi::c_int as isize);
    *fresh15 = (*fresh15).wrapping_add(H);
}
unsafe extern "C" fn safe_c2rust_sha256_sum_update(
    mut sha256: *mut Sha256sum,
    mut buffer: *const guchar,
    mut length: gsize,
) {
    let mut left: guint32 = 0;
    let mut fill: guint32 = 0;
    let mut input: *const guint8 = buffer as *const guint8;
    if length == 0 as gsize {
        return;
    }
    left = (*sha256).bits[0 as ::core::ffi::c_int as usize] & 0x3f as guint32;
    fill = (64 as guint32).wrapping_sub(left);
    (*sha256).bits[0 as ::core::ffi::c_int as usize] =
        ((*sha256).bits[0 as ::core::ffi::c_int as usize] as gsize).wrapping_add(length) as guint32
            as guint32;
    (*sha256).bits[0 as ::core::ffi::c_int as usize] &= 0xffffffff as ::core::ffi::c_uint;
    if ((*sha256).bits[0 as ::core::ffi::c_int as usize] as gsize) < length {
        (*sha256).bits[1 as ::core::ffi::c_int as usize] =
            (*sha256).bits[1 as ::core::ffi::c_int as usize].wrapping_add(1);
    }
    if left > 0 as guint32 && length >= fill as gsize {
        memcpy(
            (&raw mut (*sha256).data as *mut guint8).offset(left as isize)
                as *mut ::core::ffi::c_void,
            input as *const ::core::ffi::c_void,
            fill as size_t,
        );
        safe_c2rust_sha256_transform(
            &raw mut (*sha256).buf as *mut guint32,
            &raw mut (*sha256).data as *mut guint8 as *const guint8,
        );
        length = length.wrapping_sub(fill as gsize);
        input = input.offset(fill as isize);
        left = 0 as guint32;
    }
    while length >= SHA256_DATASIZE as gsize {
        safe_c2rust_sha256_transform(
            &raw mut (*sha256).buf as *mut guint32,
            input as *const guint8,
        );
        length = length.wrapping_sub(64 as gsize);
        input = input.offset(64 as ::core::ffi::c_int as isize);
    }
    if length != 0 {
        memcpy(
            (&raw mut (*sha256).data as *mut guint8).offset(left as isize)
                as *mut ::core::ffi::c_void,
            input as *const ::core::ffi::c_void,
            length as size_t,
        );
    }
}
static mut safe_c2rust_sha256_padding: [guint8; 64] = [
    0x80 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
    0 as ::core::ffi::c_int as guint8,
];
unsafe extern "C" fn safe_c2rust_sha256_sum_close(mut sha256: *mut Sha256sum) {
    let mut last: guint32 = 0;
    let mut padn: guint32 = 0;
    let mut high: guint32 = 0;
    let mut low: guint32 = 0;
    let mut msglen: [guint8; 8] = [0; 8];
    high = (*sha256).bits[0 as ::core::ffi::c_int as usize] >> 29 as ::core::ffi::c_int
        | (*sha256).bits[1 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int;
    low = (*sha256).bits[0 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int;
    msglen[0 as ::core::ffi::c_int as usize] = (high >> 24 as ::core::ffi::c_int) as guint8;
    msglen[(0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        (high >> 16 as ::core::ffi::c_int) as guint8;
    msglen[(0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        (high >> 8 as ::core::ffi::c_int) as guint8;
    msglen[(0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] = high as guint8;
    msglen[4 as ::core::ffi::c_int as usize] = (low >> 24 as ::core::ffi::c_int) as guint8;
    msglen[(4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        (low >> 16 as ::core::ffi::c_int) as guint8;
    msglen[(4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        (low >> 8 as ::core::ffi::c_int) as guint8;
    msglen[(4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] = low as guint8;
    last = (*sha256).bits[0 as ::core::ffi::c_int as usize] & 0x3f as guint32;
    padn = if last < 56 as guint32 {
        (56 as guint32).wrapping_sub(last)
    } else {
        (120 as guint32).wrapping_sub(last)
    };
    safe_c2rust_sha256_sum_update(
        sha256,
        &raw mut safe_c2rust_sha256_padding as *mut guint8,
        padn as gsize,
    );
    safe_c2rust_sha256_sum_update(sha256, &raw mut msglen as *mut guint8, 8 as gsize);
    (*sha256).digest[0 as ::core::ffi::c_int as usize] =
        ((*sha256).buf[0 as ::core::ffi::c_int as usize] >> 24 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[0 as ::core::ffi::c_int as usize] >> 16 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[0 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
        (*sha256).buf[0 as ::core::ffi::c_int as usize] as guint8 as guchar;
    (*sha256).digest[4 as ::core::ffi::c_int as usize] =
        ((*sha256).buf[1 as ::core::ffi::c_int as usize] >> 24 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[1 as ::core::ffi::c_int as usize] >> 16 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[1 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
        (*sha256).buf[1 as ::core::ffi::c_int as usize] as guint8 as guchar;
    (*sha256).digest[8 as ::core::ffi::c_int as usize] =
        ((*sha256).buf[2 as ::core::ffi::c_int as usize] >> 24 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[2 as ::core::ffi::c_int as usize] >> 16 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[2 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
        (*sha256).buf[2 as ::core::ffi::c_int as usize] as guint8 as guchar;
    (*sha256).digest[12 as ::core::ffi::c_int as usize] =
        ((*sha256).buf[3 as ::core::ffi::c_int as usize] >> 24 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[3 as ::core::ffi::c_int as usize] >> 16 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[3 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
        (*sha256).buf[3 as ::core::ffi::c_int as usize] as guint8 as guchar;
    (*sha256).digest[16 as ::core::ffi::c_int as usize] =
        ((*sha256).buf[4 as ::core::ffi::c_int as usize] >> 24 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[4 as ::core::ffi::c_int as usize] >> 16 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[4 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
        (*sha256).buf[4 as ::core::ffi::c_int as usize] as guint8 as guchar;
    (*sha256).digest[20 as ::core::ffi::c_int as usize] =
        ((*sha256).buf[5 as ::core::ffi::c_int as usize] >> 24 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(20 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[5 as ::core::ffi::c_int as usize] >> 16 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(20 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[5 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(20 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
        (*sha256).buf[5 as ::core::ffi::c_int as usize] as guint8 as guchar;
    (*sha256).digest[24 as ::core::ffi::c_int as usize] =
        ((*sha256).buf[6 as ::core::ffi::c_int as usize] >> 24 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(24 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[6 as ::core::ffi::c_int as usize] >> 16 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(24 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[6 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(24 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
        (*sha256).buf[6 as ::core::ffi::c_int as usize] as guint8 as guchar;
    (*sha256).digest[28 as ::core::ffi::c_int as usize] =
        ((*sha256).buf[7 as ::core::ffi::c_int as usize] >> 24 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(28 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[7 as ::core::ffi::c_int as usize] >> 16 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(28 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
        ((*sha256).buf[7 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int) as guint8
            as guchar;
    (*sha256).digest[(28 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
        (*sha256).buf[7 as ::core::ffi::c_int as usize] as guint8 as guchar;
}
unsafe extern "C" fn safe_c2rust_sha256_sum_to_string(mut sha256: *mut Sha256sum) -> *mut gchar {
    return safe_c2rust_digest_to_string(
        &raw mut (*sha256).digest as *mut guint8,
        SHA256_DIGEST_LEN as gsize,
    );
}
unsafe extern "C" fn safe_c2rust_sha256_sum_digest(
    mut sha256: *mut Sha256sum,
    mut digest: *mut guint8,
) {
    let mut i: gint = 0;
    i = 0 as ::core::ffi::c_int as gint;
    while i < SHA256_DIGEST_LEN {
        *digest.offset(i as isize) = (*sha256).digest[i as usize] as guint8;
        i += 1;
    }
}
static mut safe_c2rust_SHA2_K: [guint64; 80] = [
    0x428a2f98d728ae22 as ::core::ffi::c_ulong,
    0x7137449123ef65cd as ::core::ffi::c_ulong,
    0xb5c0fbcfec4d3b2f as ::core::ffi::c_ulong,
    0xe9b5dba58189dbbc as ::core::ffi::c_ulong,
    0x3956c25bf348b538 as ::core::ffi::c_ulong,
    0x59f111f1b605d019 as ::core::ffi::c_ulong,
    0x923f82a4af194f9b as ::core::ffi::c_ulong,
    0xab1c5ed5da6d8118 as ::core::ffi::c_ulong,
    0xd807aa98a3030242 as ::core::ffi::c_ulong,
    0x12835b0145706fbe as ::core::ffi::c_ulong,
    0x243185be4ee4b28c as ::core::ffi::c_ulong,
    0x550c7dc3d5ffb4e2 as ::core::ffi::c_ulong,
    0x72be5d74f27b896f as ::core::ffi::c_ulong,
    0x80deb1fe3b1696b1 as ::core::ffi::c_ulong,
    0x9bdc06a725c71235 as ::core::ffi::c_ulong,
    0xc19bf174cf692694 as ::core::ffi::c_ulong,
    0xe49b69c19ef14ad2 as ::core::ffi::c_ulong,
    0xefbe4786384f25e3 as ::core::ffi::c_ulong,
    0xfc19dc68b8cd5b5 as ::core::ffi::c_ulong,
    0x240ca1cc77ac9c65 as ::core::ffi::c_ulong,
    0x2de92c6f592b0275 as ::core::ffi::c_ulong,
    0x4a7484aa6ea6e483 as ::core::ffi::c_ulong,
    0x5cb0a9dcbd41fbd4 as ::core::ffi::c_ulong,
    0x76f988da831153b5 as ::core::ffi::c_ulong,
    0x983e5152ee66dfab as ::core::ffi::c_ulong,
    0xa831c66d2db43210 as ::core::ffi::c_ulong,
    0xb00327c898fb213f as ::core::ffi::c_ulong,
    0xbf597fc7beef0ee4 as ::core::ffi::c_ulong,
    0xc6e00bf33da88fc2 as ::core::ffi::c_ulong,
    0xd5a79147930aa725 as ::core::ffi::c_ulong,
    0x6ca6351e003826f as ::core::ffi::c_ulong,
    0x142929670a0e6e70 as ::core::ffi::c_ulong,
    0x27b70a8546d22ffc as ::core::ffi::c_ulong,
    0x2e1b21385c26c926 as ::core::ffi::c_ulong,
    0x4d2c6dfc5ac42aed as ::core::ffi::c_ulong,
    0x53380d139d95b3df as ::core::ffi::c_ulong,
    0x650a73548baf63de as ::core::ffi::c_ulong,
    0x766a0abb3c77b2a8 as ::core::ffi::c_ulong,
    0x81c2c92e47edaee6 as ::core::ffi::c_ulong,
    0x92722c851482353b as ::core::ffi::c_ulong,
    0xa2bfe8a14cf10364 as ::core::ffi::c_ulong,
    0xa81a664bbc423001 as ::core::ffi::c_ulong,
    0xc24b8b70d0f89791 as ::core::ffi::c_ulong,
    0xc76c51a30654be30 as ::core::ffi::c_ulong,
    0xd192e819d6ef5218 as ::core::ffi::c_ulong,
    0xd69906245565a910 as ::core::ffi::c_ulong,
    0xf40e35855771202a as ::core::ffi::c_ulong,
    0x106aa07032bbd1b8 as ::core::ffi::c_ulong,
    0x19a4c116b8d2d0c8 as ::core::ffi::c_ulong,
    0x1e376c085141ab53 as ::core::ffi::c_ulong,
    0x2748774cdf8eeb99 as ::core::ffi::c_ulong,
    0x34b0bcb5e19b48a8 as ::core::ffi::c_ulong,
    0x391c0cb3c5c95a63 as ::core::ffi::c_ulong,
    0x4ed8aa4ae3418acb as ::core::ffi::c_ulong,
    0x5b9cca4f7763e373 as ::core::ffi::c_ulong,
    0x682e6ff3d6b2b8a3 as ::core::ffi::c_ulong,
    0x748f82ee5defb2fc as ::core::ffi::c_ulong,
    0x78a5636f43172f60 as ::core::ffi::c_ulong,
    0x84c87814a1f0ab72 as ::core::ffi::c_ulong,
    0x8cc702081a6439ec as ::core::ffi::c_ulong,
    0x90befffa23631e28 as ::core::ffi::c_ulong,
    0xa4506cebde82bde9 as ::core::ffi::c_ulong,
    0xbef9a3f7b2c67915 as ::core::ffi::c_ulong,
    0xc67178f2e372532b as ::core::ffi::c_ulong,
    0xca273eceea26619c as ::core::ffi::c_ulong,
    0xd186b8c721c0c207 as ::core::ffi::c_ulong,
    0xeada7dd6cde0eb1e as ::core::ffi::c_ulong,
    0xf57d4f7fee6ed178 as ::core::ffi::c_ulong,
    0x6f067aa72176fba as ::core::ffi::c_ulong,
    0xa637dc5a2c898a6 as ::core::ffi::c_ulong,
    0x113f9804bef90dae as ::core::ffi::c_ulong,
    0x1b710b35131c471b as ::core::ffi::c_ulong,
    0x28db77f523047d84 as ::core::ffi::c_ulong,
    0x32caab7b40c72493 as ::core::ffi::c_ulong,
    0x3c9ebe0a15c9bebc as ::core::ffi::c_ulong,
    0x431d67c49c100d4c as ::core::ffi::c_ulong,
    0x4cc5d4becb3e42b6 as ::core::ffi::c_ulong,
    0x597f299cfc657e2a as ::core::ffi::c_ulong,
    0x5fcb6fab3ad6faec as ::core::ffi::c_ulong,
    0x6c44198c4a475817 as ::core::ffi::c_ulong,
];
unsafe extern "C" fn safe_c2rust_sha384_sum_init(mut sha512: *mut Sha512sum) {
    (*sha512).H[0 as ::core::ffi::c_int as usize] =
        0xcbbb9d5dc1059ed8 as ::core::ffi::c_ulong as guint64;
    (*sha512).H[1 as ::core::ffi::c_int as usize] =
        0x629a292a367cd507 as ::core::ffi::c_ulong as guint64;
    (*sha512).H[2 as ::core::ffi::c_int as usize] =
        0x9159015a3070dd17 as ::core::ffi::c_ulong as guint64;
    (*sha512).H[3 as ::core::ffi::c_int as usize] =
        0x152fecd8f70e5939 as ::core::ffi::c_ulong as guint64;
    (*sha512).H[4 as ::core::ffi::c_int as usize] =
        0x67332667ffc00b31 as ::core::ffi::c_ulong as guint64;
    (*sha512).H[5 as ::core::ffi::c_int as usize] =
        0x8eb44a8768581511 as ::core::ffi::c_ulong as guint64;
    (*sha512).H[6 as ::core::ffi::c_int as usize] =
        0xdb0c2e0d64f98fa7 as ::core::ffi::c_ulong as guint64;
    (*sha512).H[7 as ::core::ffi::c_int as usize] =
        0x47b5481dbefa4fa4 as ::core::ffi::c_ulong as guint64;
    (*sha512).block_len = 0 as guint8;
    (*sha512).data_len[0 as ::core::ffi::c_int as usize] = 0 as guint64;
    (*sha512).data_len[1 as ::core::ffi::c_int as usize] = 0 as guint64;
}
unsafe extern "C" fn safe_c2rust_sha512_sum_init(mut sha512: *mut Sha512sum) {
    (*sha512).H[0 as ::core::ffi::c_int as usize] =
        0x6a09e667f3bcc908 as ::core::ffi::c_ulong as guint64;
    (*sha512).H[1 as ::core::ffi::c_int as usize] =
        0xbb67ae8584caa73b as ::core::ffi::c_ulong as guint64;
    (*sha512).H[2 as ::core::ffi::c_int as usize] =
        0x3c6ef372fe94f82b as ::core::ffi::c_ulong as guint64;
    (*sha512).H[3 as ::core::ffi::c_int as usize] =
        0xa54ff53a5f1d36f1 as ::core::ffi::c_ulong as guint64;
    (*sha512).H[4 as ::core::ffi::c_int as usize] =
        0x510e527fade682d1 as ::core::ffi::c_ulong as guint64;
    (*sha512).H[5 as ::core::ffi::c_int as usize] =
        0x9b05688c2b3e6c1f as ::core::ffi::c_ulong as guint64;
    (*sha512).H[6 as ::core::ffi::c_int as usize] =
        0x1f83d9abfb41bd6b as ::core::ffi::c_ulong as guint64;
    (*sha512).H[7 as ::core::ffi::c_int as usize] =
        0x5be0cd19137e2179 as ::core::ffi::c_ulong as guint64;
    (*sha512).block_len = 0 as guint8;
    (*sha512).data_len[0 as ::core::ffi::c_int as usize] = 0 as guint64;
    (*sha512).data_len[1 as ::core::ffi::c_int as usize] = 0 as guint64;
}
unsafe extern "C" fn safe_c2rust_sha512_transform(mut H: *mut guint64, mut data: *const guint8) {
    let mut i: gint = 0;
    let mut t: gint = 0;
    let mut a: guint64 = 0;
    let mut b: guint64 = 0;
    let mut c: guint64 = 0;
    let mut d: guint64 = 0;
    let mut e: guint64 = 0;
    let mut f: guint64 = 0;
    let mut g: guint64 = 0;
    let mut h: guint64 = 0;
    let mut M: [guint64; 16] = [0; 16];
    let mut W: [guint64; 80] = [0; 80];
    i = 0 as ::core::ffi::c_int as gint;
    while i < 16 as ::core::ffi::c_int {
        let mut p: gint = i * 8 as gint;
        M[i as usize] = (*data.offset((p as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
            as guint64)
            << 56 as ::core::ffi::c_int
            | (*data.offset((p as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                as guint64)
                << 48 as ::core::ffi::c_int
            | (*data.offset((p as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                as guint64)
                << 40 as ::core::ffi::c_int
            | (*data.offset((p as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                as guint64)
                << 32 as ::core::ffi::c_int
            | (*data.offset((p as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                as guint64)
                << 24 as ::core::ffi::c_int
            | (*data.offset((p as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                as guint64)
                << 16 as ::core::ffi::c_int
            | (*data.offset((p as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                as guint64)
                << 8 as ::core::ffi::c_int
            | *data.offset((p as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize) as guint64;
        i += 1;
    }
    t = 0 as ::core::ffi::c_int as gint;
    while t < 80 as ::core::ffi::c_int {
        if t < 16 as ::core::ffi::c_int {
            W[t as usize] = M[t as usize];
        } else {
            W[t as usize] = ((W[(t as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                >> 19 as ::core::ffi::c_int
                | W[(t as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (W[(t as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                    >> 61 as ::core::ffi::c_int
                    | W[(t as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ W[(t as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as usize]
                    >> 6 as ::core::ffi::c_int)
                .wrapping_add(W[(t as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as usize])
                .wrapping_add(
                    (W[(t as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                        >> 1 as ::core::ffi::c_int
                        | W[(t as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (W[(t as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                            >> 8 as ::core::ffi::c_int
                            | W[(t as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ W[(t as ::core::ffi::c_int - 15 as ::core::ffi::c_int) as usize]
                            >> 7 as ::core::ffi::c_int,
                )
                .wrapping_add(W[(t as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as usize]);
        }
        t += 1;
    }
    a = *H.offset(0 as ::core::ffi::c_int as isize);
    b = *H.offset(1 as ::core::ffi::c_int as isize);
    c = *H.offset(2 as ::core::ffi::c_int as isize);
    d = *H.offset(3 as ::core::ffi::c_int as isize);
    e = *H.offset(4 as ::core::ffi::c_int as isize);
    f = *H.offset(5 as ::core::ffi::c_int as isize);
    g = *H.offset(6 as ::core::ffi::c_int as isize);
    h = *H.offset(7 as ::core::ffi::c_int as isize);
    t = 0 as ::core::ffi::c_int as gint;
    while t < 80 as ::core::ffi::c_int {
        let mut T1: guint64 = 0;
        let mut T2: guint64 = 0;
        T1 = h
            .wrapping_add(
                (e >> 14 as ::core::ffi::c_int
                    | e << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (e >> 18 as ::core::ffi::c_int
                        | e << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (e >> 41 as ::core::ffi::c_int
                        | e << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(safe_c2rust_SHA2_K[t as usize])
            .wrapping_add(W[t as usize]);
        T2 = ((a >> 28 as ::core::ffi::c_int
            | a << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (a >> 34 as ::core::ffi::c_int
                | a << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (a >> 39 as ::core::ffi::c_int
                | a << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(a & b ^ a & c ^ b & c);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(T1);
        d = c;
        c = b;
        b = a;
        a = T1.wrapping_add(T2);
        t += 1;
    }
    let ref mut fresh0 = *H.offset(0 as ::core::ffi::c_int as isize);
    *fresh0 = (*fresh0).wrapping_add(a);
    let ref mut fresh1 = *H.offset(1 as ::core::ffi::c_int as isize);
    *fresh1 = (*fresh1).wrapping_add(b);
    let ref mut fresh2 = *H.offset(2 as ::core::ffi::c_int as isize);
    *fresh2 = (*fresh2).wrapping_add(c);
    let ref mut fresh3 = *H.offset(3 as ::core::ffi::c_int as isize);
    *fresh3 = (*fresh3).wrapping_add(d);
    let ref mut fresh4 = *H.offset(4 as ::core::ffi::c_int as isize);
    *fresh4 = (*fresh4).wrapping_add(e);
    let ref mut fresh5 = *H.offset(5 as ::core::ffi::c_int as isize);
    *fresh5 = (*fresh5).wrapping_add(f);
    let ref mut fresh6 = *H.offset(6 as ::core::ffi::c_int as isize);
    *fresh6 = (*fresh6).wrapping_add(g);
    let ref mut fresh7 = *H.offset(7 as ::core::ffi::c_int as isize);
    *fresh7 = (*fresh7).wrapping_add(h);
}
unsafe extern "C" fn safe_c2rust_sha512_sum_update(
    mut sha512: *mut Sha512sum,
    mut buffer: *const guchar,
    mut length: gsize,
) {
    let mut block_left: gsize = 0;
    let mut offset: gsize = 0 as gsize;
    if length == 0 as gsize {
        return;
    }
    (*sha512).data_len[0 as ::core::ffi::c_int as usize] =
        ((*sha512).data_len[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_ulong)
            .wrapping_add(length.wrapping_mul(8 as gsize) as ::core::ffi::c_ulong)
            as guint64 as guint64;
    if (*sha512).data_len[0 as ::core::ffi::c_int as usize] < length {
        (*sha512).data_len[1 as ::core::ffi::c_int as usize] =
            (*sha512).data_len[1 as ::core::ffi::c_int as usize].wrapping_add(1);
    }
    block_left = (SHA2_BLOCK_LEN - (*sha512).block_len as ::core::ffi::c_int) as gsize;
    if block_left > 0 as gsize {
        let mut fill_len: gsize = 0;
        fill_len = if block_left < length {
            block_left
        } else {
            length
        };
        memcpy(
            (&raw mut (*sha512).block as *mut guint8)
                .offset((*sha512).block_len as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
            buffer as *const ::core::ffi::c_void,
            fill_len as size_t,
        );
        (*sha512).block_len =
            ((*sha512).block_len as gsize).wrapping_add(fill_len) as guint8 as guint8;
        length = length.wrapping_sub(fill_len);
        offset = offset.wrapping_add(fill_len);
        if (*sha512).block_len as ::core::ffi::c_int == SHA2_BLOCK_LEN {
            safe_c2rust_sha512_transform(
                &raw mut (*sha512).H as *mut guint64,
                &raw mut (*sha512).block as *mut guint8 as *const guint8,
            );
            (*sha512).block_len = 0 as guint8;
        }
    }
    while length >= SHA2_BLOCK_LEN as gsize {
        memcpy(
            &raw mut (*sha512).block as *mut guint8 as *mut ::core::ffi::c_void,
            buffer.offset(offset as isize) as *const ::core::ffi::c_void,
            SHA2_BLOCK_LEN as size_t,
        );
        safe_c2rust_sha512_transform(
            &raw mut (*sha512).H as *mut guint64,
            &raw mut (*sha512).block as *mut guint8 as *const guint8,
        );
        length = length.wrapping_sub(SHA2_BLOCK_LEN as gsize);
        offset = offset.wrapping_add(SHA2_BLOCK_LEN as gsize);
    }
    if length > 0 as gsize {
        memcpy(
            &raw mut (*sha512).block as *mut guint8 as *mut ::core::ffi::c_void,
            buffer.offset(offset as isize) as *const ::core::ffi::c_void,
            length as size_t,
        );
        (*sha512).block_len = length as guint8;
    }
}
unsafe extern "C" fn safe_c2rust_sha512_sum_close(mut sha512: *mut Sha512sum) {
    let mut l: guint = 0;
    let mut zeros: gint = 0;
    let mut pad: [guint8; 256] = [
        0 as ::core::ffi::c_int as guint8,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut pad_len: guint = 0 as guint;
    let mut i: gint = 0;
    l = ((*sha512).block_len as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as guint;
    zeros = (896 as guint).wrapping_sub(l.wrapping_add(1 as guint)) as gint;
    if zeros < 0 as ::core::ffi::c_int {
        zeros += 128 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
    }
    pad[0 as ::core::ffi::c_int as usize] = 0x80 as guint8;
    zeros -= 7 as ::core::ffi::c_int;
    pad_len = pad_len.wrapping_add(1);
    memset(
        (&raw mut pad as *mut guint8).offset(pad_len as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (zeros as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as size_t,
    );
    pad_len =
        pad_len.wrapping_add((zeros as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as guint);
    zeros = (zeros as ::core::ffi::c_int % 8 as ::core::ffi::c_int) as gint;
    pad[pad_len as usize] = ((*sha512).data_len[1 as ::core::ffi::c_int as usize]
        >> 56 as ::core::ffi::c_int) as guint8;
    pad[pad_len.wrapping_add(1 as guint) as usize] = ((*sha512).data_len
        [1 as ::core::ffi::c_int as usize]
        >> 48 as ::core::ffi::c_int) as guint8;
    pad[pad_len.wrapping_add(2 as guint) as usize] = ((*sha512).data_len
        [1 as ::core::ffi::c_int as usize]
        >> 40 as ::core::ffi::c_int) as guint8;
    pad[pad_len.wrapping_add(3 as guint) as usize] = ((*sha512).data_len
        [1 as ::core::ffi::c_int as usize]
        >> 32 as ::core::ffi::c_int) as guint8;
    pad[pad_len.wrapping_add(4 as guint) as usize] = ((*sha512).data_len
        [1 as ::core::ffi::c_int as usize]
        >> 24 as ::core::ffi::c_int) as guint8;
    pad[pad_len.wrapping_add(5 as guint) as usize] = ((*sha512).data_len
        [1 as ::core::ffi::c_int as usize]
        >> 16 as ::core::ffi::c_int) as guint8;
    pad[pad_len.wrapping_add(6 as guint) as usize] =
        ((*sha512).data_len[1 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int) as guint8;
    pad[pad_len.wrapping_add(7 as guint) as usize] =
        (*sha512).data_len[1 as ::core::ffi::c_int as usize] as guint8;
    pad_len = pad_len.wrapping_add(8 as guint);
    pad[pad_len as usize] = ((*sha512).data_len[0 as ::core::ffi::c_int as usize]
        >> 56 as ::core::ffi::c_int) as guint8;
    pad[pad_len.wrapping_add(1 as guint) as usize] = ((*sha512).data_len
        [0 as ::core::ffi::c_int as usize]
        >> 48 as ::core::ffi::c_int) as guint8;
    pad[pad_len.wrapping_add(2 as guint) as usize] = ((*sha512).data_len
        [0 as ::core::ffi::c_int as usize]
        >> 40 as ::core::ffi::c_int) as guint8;
    pad[pad_len.wrapping_add(3 as guint) as usize] = ((*sha512).data_len
        [0 as ::core::ffi::c_int as usize]
        >> 32 as ::core::ffi::c_int) as guint8;
    pad[pad_len.wrapping_add(4 as guint) as usize] = ((*sha512).data_len
        [0 as ::core::ffi::c_int as usize]
        >> 24 as ::core::ffi::c_int) as guint8;
    pad[pad_len.wrapping_add(5 as guint) as usize] = ((*sha512).data_len
        [0 as ::core::ffi::c_int as usize]
        >> 16 as ::core::ffi::c_int) as guint8;
    pad[pad_len.wrapping_add(6 as guint) as usize] =
        ((*sha512).data_len[0 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int) as guint8;
    pad[pad_len.wrapping_add(7 as guint) as usize] =
        (*sha512).data_len[0 as ::core::ffi::c_int as usize] as guint8;
    pad_len = pad_len.wrapping_add(8 as guint);
    safe_c2rust_sha512_sum_update(sha512, &raw mut pad as *mut guint8, pad_len as gsize);
    i = 0 as ::core::ffi::c_int as gint;
    while i < 8 as ::core::ffi::c_int {
        (*sha512).digest[(i as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
            ((*sha512).H[i as usize] >> 56 as ::core::ffi::c_int) as guint8 as guchar;
        (*sha512).digest[(i as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as usize] =
            ((*sha512).H[i as usize] >> 48 as ::core::ffi::c_int) as guint8 as guchar;
        (*sha512).digest[(i as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) as usize] =
            ((*sha512).H[i as usize] >> 40 as ::core::ffi::c_int) as guint8 as guchar;
        (*sha512).digest[(i as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int) as usize] =
            ((*sha512).H[i as usize] >> 32 as ::core::ffi::c_int) as guint8 as guchar;
        (*sha512).digest[(i as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int) as usize] =
            ((*sha512).H[i as usize] >> 24 as ::core::ffi::c_int) as guint8 as guchar;
        (*sha512).digest[(i as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int) as usize] =
            ((*sha512).H[i as usize] >> 16 as ::core::ffi::c_int) as guint8 as guchar;
        (*sha512).digest[(i as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int) as usize] =
            ((*sha512).H[i as usize] >> 8 as ::core::ffi::c_int) as guint8 as guchar;
        (*sha512).digest[(i as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int) as usize] = (*sha512).H[i as usize] as guint8 as guchar;
        i += 1;
    }
}
unsafe extern "C" fn safe_c2rust_sha384_sum_to_string(mut sha512: *mut Sha512sum) -> *mut gchar {
    return safe_c2rust_digest_to_string(
        &raw mut (*sha512).digest as *mut guint8,
        SHA384_DIGEST_LEN as gsize,
    );
}
unsafe extern "C" fn safe_c2rust_sha512_sum_to_string(mut sha512: *mut Sha512sum) -> *mut gchar {
    return safe_c2rust_digest_to_string(
        &raw mut (*sha512).digest as *mut guint8,
        SHA512_DIGEST_LEN as gsize,
    );
}
unsafe extern "C" fn safe_c2rust_sha384_sum_digest(
    mut sha512: *mut Sha512sum,
    mut digest: *mut guint8,
) {
    memcpy(
        digest as *mut ::core::ffi::c_void,
        &raw mut (*sha512).digest as *mut guchar as *const ::core::ffi::c_void,
        SHA384_DIGEST_LEN as size_t,
    );
}
unsafe extern "C" fn safe_c2rust_sha512_sum_digest(
    mut sha512: *mut Sha512sum,
    mut digest: *mut guint8,
) {
    memcpy(
        digest as *mut ::core::ffi::c_void,
        &raw mut (*sha512).digest as *mut guchar as *const ::core::ffi::c_void,
        SHA512_DIGEST_LEN as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_checksum_type_get_length(
    mut checksum_type: GChecksumType,
) -> gssize {
    let mut len: gssize = -(1 as ::core::ffi::c_int) as gssize;
    match checksum_type as ::core::ffi::c_uint {
        0 => {
            len = MD5_DIGEST_LEN as gssize;
        }
        1 => {
            len = SHA1_DIGEST_LEN as gssize;
        }
        2 => {
            len = SHA256_DIGEST_LEN as gssize;
        }
        4 => {
            len = SHA384_DIGEST_LEN as gssize;
        }
        3 => {
            len = SHA512_DIGEST_LEN as gssize;
        }
        _ => {
            len = -(1 as ::core::ffi::c_int) as gssize;
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_checksum_new(
    mut checksum_type: GChecksumType,
) -> *mut GChecksum {
    let mut checksum: *mut GChecksum = ::core::ptr::null_mut::<GChecksum>();
    if !(checksum_type as ::core::ffi::c_uint
        >= G_CHECKSUM_MD5 as ::core::ffi::c_int as ::core::ffi::c_uint
        && checksum_type as ::core::ffi::c_uint
            <= G_CHECKSUM_SHA384 as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        return ::core::ptr::null_mut::<GChecksum>();
    }
    checksum = ({
        let mut __s: gsize = ::core::mem::size_of::<GChecksum>() as gsize;
        let mut __p: gpointer = ::core::ptr::null_mut::<::core::ffi::c_void>();
        __p = g_slice_alloc(__s);
        memset(
            __p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            __s as size_t,
        );
        __p
    }) as *mut GChecksum;
    (*checksum).type_0 = checksum_type;
    safe_c2rust_g_checksum_reset(checksum);
    return checksum;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_checksum_reset(mut checksum: *mut GChecksum) {
    if ({
        let mut _g_boolean_var_8: ::core::ffi::c_int = 0;
        if !checksum.is_null() {
            _g_boolean_var_8 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_8 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_8
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"checksum != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    g_free((*checksum).digest_str as gpointer);
    (*checksum).digest_str = ::core::ptr::null_mut::<gchar>();
    match (*checksum).type_0 as ::core::ffi::c_uint {
        0 => {
            safe_c2rust_md5_sum_init(&raw mut (*checksum).sum.md5);
        }
        1 => {
            safe_c2rust_sha1_sum_init(&raw mut (*checksum).sum.sha1);
        }
        2 => {
            safe_c2rust_sha256_sum_init(&raw mut (*checksum).sum.sha256);
        }
        4 => {
            safe_c2rust_sha384_sum_init(&raw mut (*checksum).sum.sha512);
        }
        3 => {
            safe_c2rust_sha512_sum_init(&raw mut (*checksum).sum.sha512);
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gchecksum.c\0" as *const u8 as *const ::core::ffi::c_char,
                1522 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_checksum_copy(
    mut checksum: *const GChecksum,
) -> *mut GChecksum {
    let mut copy: *mut GChecksum = ::core::ptr::null_mut::<GChecksum>();
    if ({
        let mut _g_boolean_var_9: ::core::ffi::c_int = 0;
        if !checksum.is_null() {
            _g_boolean_var_9 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_9 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_9
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"checksum != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<GChecksum>();
    }
    copy = g_slice_alloc(::core::mem::size_of::<GChecksum>() as gsize) as *mut GChecksum;
    *copy = *checksum;
    (*copy).digest_str = safe_c2rust_g_strdup_inline((*checksum).digest_str) as *mut gchar;
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_checksum_free(mut checksum: *mut GChecksum) {
    if ({
        let mut _g_boolean_var_10: ::core::ffi::c_int = 0;
        if !checksum.is_null() {
            _g_boolean_var_10 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_10 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_10
    }) as ::core::ffi::c_long
        != 0
    {
        g_free((*checksum).digest_str as gpointer);
        g_slice_free1(
            ::core::mem::size_of::<GChecksum>() as gsize,
            checksum as gpointer,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_checksum_update(
    mut checksum: *mut GChecksum,
    mut data: *const guchar,
    mut length: gssize,
) {
    if ({
        let mut _g_boolean_var_11: ::core::ffi::c_int = 0;
        if !checksum.is_null() {
            _g_boolean_var_11 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_11 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_11
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"checksum != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if ({
        let mut _g_boolean_var_12: ::core::ffi::c_int = 0;
        if length == 0 as gssize || !data.is_null() {
            _g_boolean_var_12 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_12 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_12
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"length == 0 || data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if length < 0 as gssize {
        length = strlen(data as *const ::core::ffi::c_char) as gssize;
    }
    if !(*checksum).digest_str.is_null() {
        g_log(
            G_LOG_DOMAIN.as_ptr() as *const gchar,
            G_LOG_LEVEL_WARNING,
            b"The checksum '%s' has been closed and cannot be updated anymore.\0" as *const u8
                as *const gchar,
            (*checksum).digest_str,
        );
        return;
    }
    match (*checksum).type_0 as ::core::ffi::c_uint {
        0 => {
            safe_c2rust_md5_sum_update(&raw mut (*checksum).sum.md5, data, length as gsize);
        }
        1 => {
            safe_c2rust_sha1_sum_update(&raw mut (*checksum).sum.sha1, data, length as gsize);
        }
        2 => {
            safe_c2rust_sha256_sum_update(&raw mut (*checksum).sum.sha256, data, length as gsize);
        }
        4 | 3 => {
            safe_c2rust_sha512_sum_update(&raw mut (*checksum).sum.sha512, data, length as gsize);
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gchecksum.c\0" as *const u8 as *const ::core::ffi::c_char,
                1621 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_checksum_get_string(
    mut checksum: *mut GChecksum,
) -> *const gchar {
    let mut str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_13: ::core::ffi::c_int = 0;
        if !checksum.is_null() {
            _g_boolean_var_13 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_13 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_13
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"checksum != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null::<gchar>();
    }
    if !(*checksum).digest_str.is_null() {
        return (*checksum).digest_str;
    }
    match (*checksum).type_0 as ::core::ffi::c_uint {
        0 => {
            safe_c2rust_md5_sum_close(&raw mut (*checksum).sum.md5);
            str = safe_c2rust_md5_sum_to_string(&raw mut (*checksum).sum.md5);
        }
        1 => {
            safe_c2rust_sha1_sum_close(&raw mut (*checksum).sum.sha1);
            str = safe_c2rust_sha1_sum_to_string(&raw mut (*checksum).sum.sha1);
        }
        2 => {
            safe_c2rust_sha256_sum_close(&raw mut (*checksum).sum.sha256);
            str = safe_c2rust_sha256_sum_to_string(&raw mut (*checksum).sum.sha256);
        }
        4 => {
            safe_c2rust_sha512_sum_close(&raw mut (*checksum).sum.sha512);
            str = safe_c2rust_sha384_sum_to_string(&raw mut (*checksum).sum.sha512);
        }
        3 => {
            safe_c2rust_sha512_sum_close(&raw mut (*checksum).sum.sha512);
            str = safe_c2rust_sha512_sum_to_string(&raw mut (*checksum).sum.sha512);
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gchecksum.c\0" as *const u8 as *const ::core::ffi::c_char,
                1676 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    (*checksum).digest_str = str;
    return (*checksum).digest_str;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_checksum_get_digest(
    mut checksum: *mut GChecksum,
    mut buffer: *mut guint8,
    mut digest_len: *mut gsize,
) {
    let mut checksum_open: gboolean = FALSE;
    let mut str: *mut gchar = ::core::ptr::null_mut::<gchar>();
    let mut len: gsize = 0;
    if ({
        let mut _g_boolean_var_14: ::core::ffi::c_int = 0;
        if !checksum.is_null() {
            _g_boolean_var_14 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_14 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_14
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"checksum != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    len = safe_c2rust_g_checksum_type_get_length((*checksum).type_0) as gsize;
    if ({
        let mut _g_boolean_var_15: ::core::ffi::c_int = 0;
        if *digest_len >= len {
            _g_boolean_var_15 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_15 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_15
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"*digest_len >= len\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    checksum_open = (*checksum).digest_str.is_null() as ::core::ffi::c_int as gboolean;
    match (*checksum).type_0 as ::core::ffi::c_uint {
        0 => {
            if checksum_open != 0 {
                safe_c2rust_md5_sum_close(&raw mut (*checksum).sum.md5);
                str = safe_c2rust_md5_sum_to_string(&raw mut (*checksum).sum.md5);
            }
            safe_c2rust_md5_sum_digest(&raw mut (*checksum).sum.md5, buffer);
        }
        1 => {
            if checksum_open != 0 {
                safe_c2rust_sha1_sum_close(&raw mut (*checksum).sum.sha1);
                str = safe_c2rust_sha1_sum_to_string(&raw mut (*checksum).sum.sha1);
            }
            safe_c2rust_sha1_sum_digest(&raw mut (*checksum).sum.sha1, buffer);
        }
        2 => {
            if checksum_open != 0 {
                safe_c2rust_sha256_sum_close(&raw mut (*checksum).sum.sha256);
                str = safe_c2rust_sha256_sum_to_string(&raw mut (*checksum).sum.sha256);
            }
            safe_c2rust_sha256_sum_digest(&raw mut (*checksum).sum.sha256, buffer);
        }
        4 => {
            if checksum_open != 0 {
                safe_c2rust_sha512_sum_close(&raw mut (*checksum).sum.sha512);
                str = safe_c2rust_sha384_sum_to_string(&raw mut (*checksum).sum.sha512);
            }
            safe_c2rust_sha384_sum_digest(&raw mut (*checksum).sum.sha512, buffer);
        }
        3 => {
            if checksum_open != 0 {
                safe_c2rust_sha512_sum_close(&raw mut (*checksum).sum.sha512);
                str = safe_c2rust_sha512_sum_to_string(&raw mut (*checksum).sum.sha512);
            }
            safe_c2rust_sha512_sum_digest(&raw mut (*checksum).sum.sha512, buffer);
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN.as_ptr(),
                b"../original/glib/gchecksum.c\0" as *const u8 as *const ::core::ffi::c_char,
                1759 as ::core::ffi::c_int,
                G_STRFUNC,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    if !str.is_null() {
        (*checksum).digest_str = str;
    }
    *digest_len = len;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_compute_checksum_for_data(
    mut checksum_type: GChecksumType,
    mut data: *const guchar,
    mut length: gsize,
) -> *mut gchar {
    let mut checksum: *mut GChecksum = ::core::ptr::null_mut::<GChecksum>();
    let mut retval: *mut gchar = ::core::ptr::null_mut::<gchar>();
    if ({
        let mut _g_boolean_var_16: ::core::ffi::c_int = 0;
        if length == 0 as gsize || !data.is_null() {
            _g_boolean_var_16 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_16 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_16
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"length == 0 || data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    checksum = safe_c2rust_g_checksum_new(checksum_type);
    if checksum.is_null() {
        return ::core::ptr::null_mut::<gchar>();
    }
    safe_c2rust_g_checksum_update(checksum, data, length as gssize);
    retval = safe_c2rust_g_strdup_inline(
        safe_c2rust_g_checksum_get_string(checksum) as *const ::core::ffi::c_char
    ) as *mut gchar;
    safe_c2rust_g_checksum_free(checksum);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_compute_checksum_for_string(
    mut checksum_type: GChecksumType,
    mut str: *const gchar,
    mut length: gssize,
) -> *mut gchar {
    if ({
        let mut _g_boolean_var_17: ::core::ffi::c_int = 0;
        if length == 0 as gssize || !str.is_null() {
            _g_boolean_var_17 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_17 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_17
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"length == 0 || str != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    if length < 0 as gssize {
        length = strlen(str as *const ::core::ffi::c_char) as gssize;
    }
    return safe_c2rust_g_compute_checksum_for_data(
        checksum_type,
        str as *const guchar,
        length as gsize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_g_compute_checksum_for_bytes(
    mut checksum_type: GChecksumType,
    mut data: *mut GBytes,
) -> *mut gchar {
    let mut byte_data: gconstpointer = ::core::ptr::null::<::core::ffi::c_void>();
    let mut length: gsize = 0;
    if ({
        let mut _g_boolean_var_18: ::core::ffi::c_int = 0;
        if !data.is_null() {
            _g_boolean_var_18 = 1 as ::core::ffi::c_int;
        } else {
            _g_boolean_var_18 = 0 as ::core::ffi::c_int;
        }
        _g_boolean_var_18
    }) as ::core::ffi::c_long
        != 0
    {
    } else {
        g_return_if_fail_warning(
            G_LOG_DOMAIN.as_ptr(),
            G_STRFUNC,
            b"data != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<gchar>();
    }
    byte_data = g_bytes_get_data(data, &raw mut length);
    return safe_c2rust_g_compute_checksum_for_data(
        checksum_type,
        byte_data as *const guchar,
        length,
    );
}
pub const G_LOG_DOMAIN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GLib\0") };
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const G_STRFUNC: *const ::core::ffi::c_char =
    b"g_checksum_reset\0" as *const u8 as *const ::core::ffi::c_char;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
