use crate::translated::original_gio_ginetaddress::{in6_addr, C2RustUnnamed_1};

#[unsafe(export_name = "__lsan_disable")]
pub unsafe extern "C" fn lsan_disable() {}

#[unsafe(export_name = "safe_c2rust_in6addr_any")]
pub static SAFE_C2RUST_IN6ADDR_ANY: in6_addr = in6_addr {
    __in6_u: C2RustUnnamed_1 {
        __u6_addr8: [0; 16],
    },
};

#[unsafe(export_name = "safe_c2rust_in6addr_loopback")]
pub static SAFE_C2RUST_IN6ADDR_LOOPBACK: in6_addr = in6_addr {
    __in6_u: C2RustUnnamed_1 {
        __u6_addr8: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    },
};
