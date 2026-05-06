fn main() {
    if let Err(error) = safe_girepository::tools::run_inspect_typelib() {
        eprintln!("gi-inspect-typelib: {error}");
        std::process::exit(1);
    }
}
