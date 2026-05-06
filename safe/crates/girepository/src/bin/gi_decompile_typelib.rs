fn main() {
    if let Err(error) = safe_girepository::tools::run_decompile_typelib() {
        eprintln!("gi-decompile-typelib: {error}");
        std::process::exit(1);
    }
}
