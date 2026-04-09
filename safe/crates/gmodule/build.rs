fn main() {
    println!("cargo:rerun-if-env-changed=SAFE_LINK_SONAME");
    println!("cargo:rerun-if-env-changed=SAFE_LINK_VERSION_SCRIPT");
    if let Ok(soname) = std::env::var("SAFE_LINK_SONAME") {
        println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,{soname}");
    }
    if let Ok(version_script) = std::env::var("SAFE_LINK_VERSION_SCRIPT") {
        println!("cargo:rustc-cdylib-link-arg=-Wl,--version-script={version_script}");
    }
}
