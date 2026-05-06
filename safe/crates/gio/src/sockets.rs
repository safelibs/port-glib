use crate::ffi::gpointer;

pub const CLUSTER: &str = "sockets";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn g_socket_new(
    _family: i32,
    _socket_type: i32,
    _protocol: i32,
    _error: *mut gpointer,
) -> gpointer {
    crate::runtime::opaque_handle(0x736f_636b)
}
