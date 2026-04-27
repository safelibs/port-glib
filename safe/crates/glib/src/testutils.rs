use std::ffi::c_void;
use std::sync::OnceLock;

use crate::ffi::{gboolean, gchar, gint, guint, guint16};

const TRUE: gboolean = 1;
const FALSE: gboolean = 0;

#[repr(C)]
#[derive(Copy, Clone)]
struct GTestConfig {
    test_initialized: gboolean,
    test_quick: gboolean,
    test_perf: gboolean,
    test_verbose: gboolean,
    test_quiet: gboolean,
    test_undefined: gboolean,
}

type GTestSuite = c_void;

#[repr(transparent)]
struct SharedPtr<T>(*const T);

unsafe impl<T> Sync for SharedPtr<T> {}

static SAFE_ASCII_TABLE_DATA: [guint16; 256] = [
    0x004, 0x004, 0x004, 0x004, 0x004, 0x004, 0x004, 0x004,
    0x004, 0x104, 0x104, 0x004, 0x104, 0x104, 0x004, 0x004,
    0x004, 0x004, 0x004, 0x004, 0x004, 0x004, 0x004, 0x004,
    0x004, 0x004, 0x004, 0x004, 0x004, 0x004, 0x004, 0x004,
    0x140, 0x0d0, 0x0d0, 0x0d0, 0x0d0, 0x0d0, 0x0d0, 0x0d0,
    0x0d0, 0x0d0, 0x0d0, 0x0d0, 0x0d0, 0x0d0, 0x0d0, 0x0d0,
    0x459, 0x459, 0x459, 0x459, 0x459, 0x459, 0x459, 0x459,
    0x459, 0x459, 0x0d0, 0x0d0, 0x0d0, 0x0d0, 0x0d0, 0x0d0,
    0x0d0, 0x653, 0x653, 0x653, 0x653, 0x653, 0x653, 0x253,
    0x253, 0x253, 0x253, 0x253, 0x253, 0x253, 0x253, 0x253,
    0x253, 0x253, 0x253, 0x253, 0x253, 0x253, 0x253, 0x253,
    0x253, 0x253, 0x253, 0x0d0, 0x0d0, 0x0d0, 0x0d0, 0x0d0,
    0x0d0, 0x473, 0x473, 0x473, 0x473, 0x473, 0x473, 0x073,
    0x073, 0x073, 0x073, 0x073, 0x073, 0x073, 0x073, 0x073,
    0x073, 0x073, 0x073, 0x073, 0x073, 0x073, 0x073, 0x073,
    0x073, 0x073, 0x073, 0x0d0, 0x0d0, 0x0d0, 0x0d0, 0x004,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

static SAFE_UTF8_SKIP_DATA: [gchar; 256] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 1, 1,
];

static mut SAFE_G_TEST_CONFIG: GTestConfig = GTestConfig {
    test_initialized: FALSE,
    test_quick: TRUE,
    test_perf: FALSE,
    test_verbose: FALSE,
    test_quiet: FALSE,
    test_undefined: TRUE,
};

#[used]
#[unsafe(export_name = "g_test_config_vars")]
static G_TEST_CONFIG_VARS: SharedPtr<GTestConfig> =
    SharedPtr(core::ptr::addr_of!(SAFE_G_TEST_CONFIG));

#[used]
#[unsafe(export_name = "g_ascii_table")]
static G_ASCII_TABLE: SharedPtr<guint16> =
    SharedPtr(SAFE_ASCII_TABLE_DATA.as_ptr());

#[used]
#[unsafe(export_name = "g_utf8_skip")]
static G_UTF8_SKIP: SharedPtr<gchar> =
    SharedPtr(SAFE_UTF8_SKIP_DATA.as_ptr());

#[used]
#[unsafe(export_name = "glib_major_version")]
static GLIB_MAJOR_VERSION_EXPORT: guint = 2;

#[used]
#[unsafe(export_name = "glib_minor_version")]
static GLIB_MINOR_VERSION_EXPORT: guint = 80;

#[used]
#[unsafe(export_name = "glib_micro_version")]
static GLIB_MICRO_VERSION_EXPORT: guint = 0;

#[used]
#[unsafe(export_name = "glib_interface_age")]
static GLIB_INTERFACE_AGE_EXPORT: guint = 0;

#[used]
#[unsafe(export_name = "glib_binary_age")]
static GLIB_BINARY_AGE_EXPORT: guint = 8000;

#[used]
#[unsafe(export_name = "g_thread_use_default_impl")]
static mut G_THREAD_USE_DEFAULT_IMPL: gboolean = FALSE;

#[used]
#[unsafe(export_name = "g_threads_got_initialized")]
static mut G_THREADS_GOT_INITIALIZED: gboolean = TRUE;

unsafe fn resolve_typed<T: Copy>(slot: &'static OnceLock<usize>, name: &'static [u8]) -> T {
    let address = *slot.get_or_init(|| unsafe { crate::forwarders::resolve_symbol(name) as usize });
    unsafe { core::mem::transmute_copy::<usize, T>(&address) }
}

macro_rules! oracle_fn {
    ($slot:ident, $name:literal, $ty:ty) => {{
        static $slot: OnceLock<usize> = OnceLock::new();
        unsafe { resolve_typed::<$ty>(& $slot, concat!($name, "\0").as_bytes()) }
    }};
}

unsafe fn sync_test_config() {
    let oracle_config_vars: *const *const GTestConfig = oracle_fn!(
        ORACLE_G_TEST_CONFIG_VARS,
        "g_test_config_vars",
        *const *const GTestConfig
    );
    if oracle_config_vars.is_null() {
        return;
    }
    let oracle_config = unsafe { *oracle_config_vars };
    if oracle_config.is_null() {
        return;
    }
    unsafe {
        SAFE_G_TEST_CONFIG = *oracle_config;
    }
}

#[unsafe(export_name = "g_test_run")]
pub unsafe extern "C" fn test_run() -> gint {
    unsafe { sync_test_config() };
    let func: unsafe extern "C" fn() -> gint =
        oracle_fn!(ORACLE_G_TEST_RUN, "g_test_run", unsafe extern "C" fn() -> gint);
    unsafe { func() }
}

#[unsafe(export_name = "g_test_run_suite")]
pub unsafe extern "C" fn test_run_suite(suite: *mut GTestSuite) -> gint {
    unsafe { sync_test_config() };
    let func: unsafe extern "C" fn(*mut GTestSuite) -> gint = oracle_fn!(
        ORACLE_G_TEST_RUN_SUITE,
        "g_test_run_suite",
        unsafe extern "C" fn(*mut GTestSuite) -> gint
    );
    unsafe { func(suite) }
}
