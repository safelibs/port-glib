fn abort_with(message: &str) -> ! {
    eprintln!("{message}");
    std::process::abort();
}

#[unsafe(no_mangle)]
pub extern "C" fn safe_gio_unimplemented() -> ! {
    abort_with("safe GIO Rust export stub called before this API cluster was implemented")
}
