use crate::ffi::gpointer;

#[unsafe(export_name = "g_thread_init")]
pub unsafe extern "C" fn thread_init(init: gpointer) {
    if !init.is_null() {
        crate::runtime::emit_warning(
            b"g_thread_init\0",
            b"GThread system no longer supports custom thread implementations.\0",
        );
    }
}

#[unsafe(export_name = "g_thread_init_with_errorcheck_mutexes")]
pub unsafe extern "C" fn thread_init_with_errorcheck_mutexes(_vtable: gpointer) {
    crate::runtime::emit_warning(
        b"g_thread_init_with_errorcheck_mutexes\0",
        b"GThread system no longer supports errorcheck mutexes.\0",
    );
}
