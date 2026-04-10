use std::env;
use std::path::PathBuf;
use std::process::Command;

fn emit_cdylib_arg(arg: impl AsRef<str>) {
    println!("cargo:rustc-cdylib-link-arg={}", arg.as_ref());
}

fn archive_backend(ar: &str, archive: &PathBuf, object: &str) {
    let output = Command::new(ar)
        .arg("crus")
        .arg(archive)
        .arg(object)
        .output()
        .unwrap_or_else(|error| panic!("failed to spawn {ar}: {error}"));
    if !output.status.success() {
        panic!(
            "{ar} failed with status {}:\n{}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing manifest dir"));
    let build_check_dir = manifest_dir
        .join("../../vendor/build-check")
        .canonicalize()
        .expect("missing vendored build-check directory");
    let backend_builder = manifest_dir
        .join("../../tools/build-glib-backend.py")
        .canonicalize()
        .expect("missing backend builder");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("missing out dir"));

    println!("cargo:rerun-if-changed={}", backend_builder.display());
    println!("cargo:rerun-if-changed={}", build_check_dir.join("glib").display());
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("../../vendor/original/glib").display()
    );
    println!("cargo:rerun-if-env-changed=SAFE_LINK_SONAME");
    println!("cargo:rerun-if-env-changed=SAFE_LINK_VERSION_SCRIPT");
    println!("cargo:rerun-if-env-changed=AR");

    if let Ok(soname) = env::var("SAFE_LINK_SONAME") {
        emit_cdylib_arg(format!("-Wl,-soname,{soname}"));
    }
    if let Ok(version_script) = env::var("SAFE_LINK_VERSION_SCRIPT") {
        emit_cdylib_arg(format!("-Wl,--version-script={version_script}"));
    }

    emit_cdylib_arg("-Wl,--as-needed");
    emit_cdylib_arg("-Wl,--no-undefined");
    emit_cdylib_arg("-Wl,-z,nodelete");
    emit_cdylib_arg("-Wl,-Bsymbolic-functions");
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=pcre2-8");

    let output = Command::new("python3")
        .arg(&backend_builder)
        .arg("--build-check-dir")
        .arg(&build_check_dir)
        .arg("--out-dir")
        .arg(&out_dir)
        .output()
        .unwrap_or_else(|error| panic!("failed to spawn {}: {error}", backend_builder.display()));
    if !output.status.success() {
        panic!(
            "{} failed with status {}:\n{}\n{}",
            backend_builder.display(),
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }

    let backend_object = String::from_utf8(output.stdout)
        .expect("backend builder must print utf-8")
        .trim()
        .to_owned();
    if backend_object.is_empty() {
        panic!("backend builder did not emit a backend object path");
    }
    let backend_archive = out_dir.join("libsafe_glib_backend.a");
    let ar = env::var("AR").unwrap_or_else(|_| "ar".to_owned());
    archive_backend(&ar, &backend_archive, &backend_object);
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=safe_glib_backend");
}
