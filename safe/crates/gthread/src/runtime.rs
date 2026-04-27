use crate::ffi::{gchar, guint};

const G_LOG_LEVEL_WARNING: guint = 1 << 4;

unsafe extern "C" {
    fn g_log(
        log_domain: *const gchar,
        log_level: guint,
        format: *const gchar,
        ...
    );
}

pub(crate) unsafe fn emit_warning(func: &'static [u8], message: &'static [u8]) {
    let _ = func;
    g_log(
        c"GThread".as_ptr().cast(),
        G_LOG_LEVEL_WARNING,
        c"%s".as_ptr().cast(),
        message.as_ptr().cast::<gchar>(),
    );
}
