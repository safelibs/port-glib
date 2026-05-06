fn main() {
    if let Err(error) = safe_girepository::tools::run_compile_repository() {
        eprintln!("gi-compile-repository: {error}");
        std::process::exit(1);
    }
}
