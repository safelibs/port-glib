type GThreadFunctionsStorage = [usize; 21];

#[repr(C)]
pub(crate) struct ConstPtr<T>(*const T);

unsafe impl<T> Sync for ConstPtr<T> {}

#[repr(C)]
pub(crate) struct GTestConfig {
    test_initialized: i32,
    test_quick: i32,
    test_perf: i32,
    test_verbose: i32,
    test_quiet: i32,
    test_undefined: i32,
}

const G_ASCII_ALNUM: u16 = 1 << 0;
const G_ASCII_ALPHA: u16 = 1 << 1;
const G_ASCII_CNTRL: u16 = 1 << 2;
const G_ASCII_DIGIT: u16 = 1 << 3;
const G_ASCII_GRAPH: u16 = 1 << 4;
const G_ASCII_LOWER: u16 = 1 << 5;
const G_ASCII_PRINT: u16 = 1 << 6;
const G_ASCII_PUNCT: u16 = 1 << 7;
const G_ASCII_SPACE: u16 = 1 << 8;
const G_ASCII_UPPER: u16 = 1 << 9;
const G_ASCII_XDIGIT: u16 = 1 << 10;

const fn ascii_table_data() -> [u16; 256] {
    let mut table = [0u16; 256];
    let mut i = 0usize;
    while i < 128 {
        let mut flags = 0u16;
        if i < 32 || i == 127 {
            flags |= G_ASCII_CNTRL;
        }
        if i == b' ' as usize
            || i == b'\t' as usize
            || i == b'\n' as usize
            || i == 0x0c
            || i == b'\r' as usize
        {
            flags |= G_ASCII_SPACE;
        }
        if i >= 32 && i <= 126 {
            flags |= G_ASCII_PRINT;
        }
        if i >= 33 && i <= 126 {
            flags |= G_ASCII_GRAPH;
        }
        if i >= b'0' as usize && i <= b'9' as usize {
            flags |= G_ASCII_ALNUM | G_ASCII_DIGIT | G_ASCII_XDIGIT;
        } else if i >= b'A' as usize && i <= b'Z' as usize {
            flags |= G_ASCII_ALNUM | G_ASCII_ALPHA | G_ASCII_UPPER;
            if i <= b'F' as usize {
                flags |= G_ASCII_XDIGIT;
            }
        } else if i >= b'a' as usize && i <= b'z' as usize {
            flags |= G_ASCII_ALNUM | G_ASCII_ALPHA | G_ASCII_LOWER;
            if i <= b'f' as usize {
                flags |= G_ASCII_XDIGIT;
            }
        } else if i >= 33 && i <= 126 {
            flags |= G_ASCII_PUNCT;
        }
        table[i] = flags;
        i += 1;
    }
    table
}

const fn utf8_skip_data() -> [i8; 256] {
    let mut table = [1i8; 256];
    let mut i = 0xc0usize;
    while i < 0xe0 {
        table[i] = 2;
        i += 1;
    }
    while i < 0xf0 {
        table[i] = 3;
        i += 1;
    }
    while i < 0xf8 {
        table[i] = 4;
        i += 1;
    }
    while i < 0xfc {
        table[i] = 5;
        i += 1;
    }
    while i < 0xfe {
        table[i] = 6;
        i += 1;
    }
    table
}

static ASCII_TABLE_DATA: [u16; 256] = ascii_table_data();
static UTF8_SKIP_DATA: [i8; 256] = utf8_skip_data();

static TEST_CONFIG_VARS: GTestConfig = GTestConfig {
    test_initialized: 1,
    test_quick: 1,
    test_perf: 0,
    test_verbose: 0,
    test_quiet: 0,
    test_undefined: 0,
};

#[unsafe(export_name = "__glib_assert_msg")]
pub static mut GLIB_ASSERT_MSG: usize = 0;
#[unsafe(export_name = "g_mem_gc_friendly")]
pub static mut G_MEM_GC_FRIENDLY: u32 = 0;
#[unsafe(export_name = "g_thread_use_default_impl")]
pub static mut G_THREAD_USE_DEFAULT_IMPL: u32 = 0;
#[unsafe(export_name = "g_ascii_table")]
pub static G_ASCII_TABLE: ConstPtr<u16> = ConstPtr(ASCII_TABLE_DATA.as_ptr());
#[unsafe(export_name = "g_test_config_vars")]
pub static G_TEST_CONFIG_VARS: ConstPtr<GTestConfig> = ConstPtr(&TEST_CONFIG_VARS);
#[unsafe(export_name = "g_thread_functions_for_glib_use")]
pub static mut G_THREAD_FUNCTIONS_FOR_GLIB_USE: GThreadFunctionsStorage = [0; 21];
#[unsafe(export_name = "g_thread_gettime")]
pub static mut G_THREAD_GETTIME: usize = 0;
#[unsafe(export_name = "g_threads_got_initialized")]
pub static mut G_THREADS_GOT_INITIALIZED: u32 = 1;
#[unsafe(export_name = "g_utf8_skip")]
pub static G_UTF8_SKIP: ConstPtr<i8> = ConstPtr(UTF8_SKIP_DATA.as_ptr());
#[unsafe(export_name = "glib_mem_profiler_table")]
pub static mut GLIB_MEM_PROFILER_TABLE: usize = 0;
#[unsafe(export_name = "glib_on_error_halt")]
pub static mut GLIB_ON_ERROR_HALT: u32 = 0;
#[unsafe(export_name = "glib_binary_age")]
pub static mut GLIB_BINARY_AGE: u32 = 8000;
#[unsafe(export_name = "glib_interface_age")]
pub static mut GLIB_INTERFACE_AGE: u32 = 0;
#[unsafe(export_name = "glib_major_version")]
pub static mut GLIB_MAJOR_VERSION: u32 = 2;
#[unsafe(export_name = "glib_micro_version")]
pub static mut GLIB_MICRO_VERSION: u32 = 0;
#[unsafe(export_name = "glib_minor_version")]
pub static mut GLIB_MINOR_VERSION: u32 = 80;
