use ::c2rust_asm_casts;
use ::c2rust_asm_casts::AsmCastTrait;
use ::core::arch::asm;
extern "C" {
    pub type _GMappedFile;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn g_mapped_file_new(
        filename: *const gchar,
        writable: gboolean,
        error: *mut *mut GError,
    ) -> *mut GMappedFile;
    fn g_mapped_file_get_length(file: *mut GMappedFile) -> gsize;
    fn g_mapped_file_get_contents(file: *mut GMappedFile) -> *mut gchar;
    fn g_mapped_file_unref(file: *mut GMappedFile);
}
pub type size_t = usize;
pub type guint32 = ::core::ffi::c_uint;
pub type gint64 = ::core::ffi::c_long;
pub type guint64 = ::core::ffi::c_ulong;
pub type gsize = ::core::ffi::c_ulong;
pub type gchar = ::core::ffi::c_char;
pub type gint = ::core::ffi::c_int;
pub type gboolean = gint;
pub type guint = ::core::ffi::c_uint;
pub type GQuark = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GError {
    pub domain: GQuark,
    pub code: gint,
    pub message: *mut gchar,
}
pub type GError = _GError;
pub type GMappedFile = _GMappedFile;
pub type __u16 = ::core::ffi::c_ushort;
pub type __s32 = ::core::ffi::c_int;
pub type __u32 = ::core::ffi::c_uint;
pub type __s64 = ::core::ffi::c_longlong;
pub type __u64 = ::core::ffi::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statx_timestamp {
    pub tv_sec: __s64,
    pub tv_nsec: __u32,
    pub __reserved: __s32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statx {
    pub stx_mask: __u32,
    pub stx_blksize: __u32,
    pub stx_attributes: __u64,
    pub stx_nlink: __u32,
    pub stx_uid: __u32,
    pub stx_gid: __u32,
    pub stx_mode: __u16,
    pub __spare0: [__u16; 1],
    pub stx_ino: __u64,
    pub stx_size: __u64,
    pub stx_blocks: __u64,
    pub stx_attributes_mask: __u64,
    pub stx_atime: statx_timestamp,
    pub stx_btime: statx_timestamp,
    pub stx_ctime: statx_timestamp,
    pub stx_mtime: statx_timestamp,
    pub stx_rdev_major: __u32,
    pub stx_rdev_minor: __u32,
    pub stx_dev_major: __u32,
    pub stx_dev_minor: __u32,
    pub stx_mnt_id: __u64,
    pub stx_dio_mem_align: __u32,
    pub stx_dio_offset_align: __u32,
    pub __spare3: [__u64; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExpectedInfo {
    pub uri: *const gchar,
    pub mtime: guint64,
    pub size: guint64,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const G_MAXUINT32: guint32 = 0xffffffff as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_size(mut buf: *const statx) -> guint64 {
    return (*buf).stx_size as guint64;
}
#[inline]
unsafe extern "C" fn safe_c2rust__g_stat_mtime(mut buf: *const statx) -> gint64 {
    return (*buf).stx_mtime.tv_sec as gint64;
}
pub const MATCHED_URI: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << 0 as ::core::ffi::c_int;
pub const MATCHED_MTIME: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int;
pub const MATCHED_ALL: ::core::ffi::c_uint = MATCHED_URI | MATCHED_MTIME;
unsafe extern "C" fn safe_c2rust_check_integer_match(
    mut expected: guint64,
    mut value: *const gchar,
    mut value_size: guint32,
) -> gboolean {
    if expected == 0 as guint64 {
        return (value_size == 1 as guint32
            && *value.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '0' as i32)
            as ::core::ffi::c_int;
    }
    while expected != 0 && value_size != 0 {
        if *value.offset(value_size.wrapping_sub(1 as guint32) as isize) as ::core::ffi::c_int
            != expected
                .wrapping_rem(10 as guint64)
                .wrapping_add('0' as i32 as guint64) as gchar as ::core::ffi::c_int
        {
            return FALSE;
        }
        expected = expected.wrapping_div(10 as guint64);
        value_size = value_size.wrapping_sub(1);
    }
    return (expected == 0 && value_size == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn safe_c2rust_check_png_info_chunk(
    mut expected_info: *mut ExpectedInfo,
    mut key: *const gchar,
    mut key_size: guint32,
    mut value: *const gchar,
    mut value_size: guint32,
    mut required_matches: *mut guint,
) -> gboolean {
    if key_size == 10 as guint32
        && memcmp(
            key as *const ::core::ffi::c_void,
            b"Thumb::URI\0" as *const u8 as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            10 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        let mut expected_size: gsize = 0;
        expected_size = strlen((*expected_info).uri as *const ::core::ffi::c_char) as gsize;
        if expected_size != value_size as gsize {
            return FALSE;
        }
        if memcmp(
            (*expected_info).uri as *const ::core::ffi::c_void,
            value as *const ::core::ffi::c_void,
            value_size as size_t,
        ) != 0 as ::core::ffi::c_int
        {
            return FALSE;
        }
        *required_matches |= MATCHED_URI;
    } else if key_size == 12 as guint32
        && memcmp(
            key as *const ::core::ffi::c_void,
            b"Thumb::MTime\0" as *const u8 as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            12 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        if safe_c2rust_check_integer_match((*expected_info).mtime, value, value_size) == 0 {
            return FALSE;
        }
        *required_matches |= MATCHED_MTIME;
    } else if key_size == 11 as guint32
        && memcmp(
            key as *const ::core::ffi::c_void,
            b"Thumb::Size\0" as *const u8 as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            11 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        if safe_c2rust_check_integer_match((*expected_info).size, value, value_size) == 0 {
            return FALSE;
        }
    }
    return TRUE;
}
unsafe extern "C" fn safe_c2rust_check_thumbnail_validity(
    mut expected_info: *mut ExpectedInfo,
    mut contents: *const gchar,
    mut size: gsize,
) -> gboolean {
    let mut required_matches: guint = 0 as guint;
    if size < 8 as gsize {
        return FALSE;
    }
    if memcmp(
        contents as *const ::core::ffi::c_void,
        b"\x89PNG\r\n\x1A\n\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        8 as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        return FALSE;
    }
    contents = contents.offset(8 as ::core::ffi::c_int as isize);
    size = size.wrapping_sub(8 as gsize);
    while size >= 12 as gsize {
        let mut chunk_size_be: guint32 = 0;
        let mut chunk_size: guint32 = 0;
        memcpy(
            &raw mut chunk_size_be as *mut ::core::ffi::c_void,
            contents as *const ::core::ffi::c_void,
            4 as size_t,
        );
        chunk_size = ({
            let mut __v: guint32 = 0;
            let mut __x: guint32 = chunk_size_be;
            if 0 != 0 {
                __v = (__x & 0xff as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int
                    | (__x & 0xff00 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int
                    | (__x & 0xff0000 as ::core::ffi::c_uint) >> 8 as ::core::ffi::c_int
                    | (__x & 0xff000000 as ::core::ffi::c_uint) >> 24 as ::core::ffi::c_int;
            } else {
                let fresh0 = &mut __v;
                let fresh1;
                let fresh2 = __x;
                asm!(
                    "bswapl {0:e}\n", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
            }
            __v
        });
        contents = contents.offset(4 as ::core::ffi::c_int as isize);
        size = size.wrapping_sub(4 as gsize);
        if G_MAXUINT32.wrapping_sub(chunk_size) < 8 as guint32
            || size < chunk_size.wrapping_add(8 as guint32) as gsize
        {
            break;
        }
        if memcmp(
            contents as *const ::core::ffi::c_void,
            b"tEXt\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            let mut key: *const gchar = contents.offset(4 as ::core::ffi::c_int as isize);
            let mut key_size: guint32 = 0;
            key_size = 0 as guint32;
            while key_size < chunk_size {
                if *key.offset(key_size as isize) as ::core::ffi::c_int == '\0' as i32 {
                    let mut value: *const gchar = ::core::ptr::null::<gchar>();
                    let mut value_size: guint32 = 0;
                    value_size = chunk_size.wrapping_sub(key_size).wrapping_sub(1 as guint32);
                    value = key
                        .offset(key_size as isize)
                        .offset(1 as ::core::ffi::c_int as isize);
                    if safe_c2rust_check_png_info_chunk(
                        expected_info,
                        key,
                        key_size,
                        value,
                        value_size,
                        &raw mut required_matches,
                    ) == 0
                    {
                        return FALSE;
                    }
                }
                key_size = key_size.wrapping_add(1);
            }
        } else if required_matches == MATCHED_ALL {
            break;
        }
        contents = contents.offset(4 as ::core::ffi::c_int as isize);
        size = size.wrapping_sub(4 as gsize);
        contents = contents.offset(chunk_size as isize);
        size = size.wrapping_sub(chunk_size as gsize);
        contents = contents.offset(4 as ::core::ffi::c_int as isize);
        size = size.wrapping_sub(4 as gsize);
    }
    return (required_matches == MATCHED_ALL) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn safe_c2rust_thumbnail_verify(
    mut thumbnail_path: *const ::core::ffi::c_char,
    mut file_uri: *const gchar,
    mut file_stat_buf: *const statx,
) -> gboolean {
    let mut thumbnail_is_valid: gboolean = FALSE;
    let mut expected_info: ExpectedInfo = ExpectedInfo {
        uri: ::core::ptr::null::<gchar>(),
        mtime: 0,
        size: 0,
    };
    let mut file: *mut GMappedFile = ::core::ptr::null_mut::<GMappedFile>();
    if file_stat_buf.is_null() {
        return FALSE;
    }
    expected_info.uri = file_uri;
    expected_info.mtime = safe_c2rust__g_stat_mtime(file_stat_buf) as guint64;
    expected_info.size = safe_c2rust__g_stat_size(file_stat_buf);
    file = g_mapped_file_new(
        thumbnail_path as *const gchar,
        FALSE,
        ::core::ptr::null_mut::<*mut GError>(),
    );
    if !file.is_null() {
        thumbnail_is_valid = safe_c2rust_check_thumbnail_validity(
            &raw mut expected_info,
            g_mapped_file_get_contents(file),
            g_mapped_file_get_length(file),
        );
        g_mapped_file_unref(file);
    }
    return thumbnail_is_valid;
}
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = (FALSE == 0) as ::core::ffi::c_int;
