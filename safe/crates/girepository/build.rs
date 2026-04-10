use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn emit_cdylib_arg(arg: impl AsRef<str>) {
    println!("cargo:rustc-cdylib-link-arg={}", arg.as_ref());
}

fn emit_objects(object_dir: &Path) {
    let mut objects: Vec<_> = fs::read_dir(object_dir)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", object_dir.display()))
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            (path.extension().and_then(|ext| ext.to_str()) == Some("o")).then_some(path)
        })
        .collect();
    objects.sort();
    if objects.is_empty() {
        panic!("no object files found in {}", object_dir.display());
    }

    for object in objects {
        println!("cargo:rerun-if-changed={}", object.display());
        emit_cdylib_arg(object.display().to_string());
    }
}

fn emit_archive(archive: &Path) {
    println!("cargo:rerun-if-changed={}", archive.display());
    emit_cdylib_arg(archive.display().to_string());
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let object_dir = manifest_dir
        .join("../../vendor/build-check/girepository/libgirepository-2.0.so.0.8000.0.p")
        .canonicalize()
        .expect("missing vendored girepository object directory");
    let archives = [
        manifest_dir
            .join("../../vendor/build-check/girepository/libgirepository-internals.a")
            .canonicalize()
            .expect("missing vendored girepository internals archive"),
        manifest_dir
            .join("../../vendor/build-check/girepository/libgirepository-gthash.a")
            .canonicalize()
            .expect("missing vendored girepository gthash archive"),
        manifest_dir
            .join("../../vendor/build-check/girepository/cmph/libcmph.a")
            .canonicalize()
            .expect("missing vendored cmph archive"),
    ];

    println!("cargo:rerun-if-changed={}", object_dir.display());
    println!("cargo:rerun-if-env-changed=SAFE_LINK_SONAME");
    println!("cargo:rerun-if-env-changed=SAFE_LINK_VERSION_SCRIPT");

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
    emit_objects(&object_dir);
    for archive in &archives {
        emit_archive(archive);
    }
    println!("cargo:rustc-link-lib=dylib=glib-2.0");
    println!("cargo:rustc-link-lib=dylib=gobject-2.0");
    println!("cargo:rustc-link-lib=dylib=gmodule-2.0");
    println!("cargo:rustc-link-lib=dylib=ffi");
    println!("cargo:rustc-link-lib=dylib=m");
}
