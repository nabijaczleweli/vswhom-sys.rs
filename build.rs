#[cfg(target_os="windows")]
extern crate cc;


#[cfg(not(target_os="windows"))]
fn main() {}

#[cfg(target_os="windows")]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=ext/vswhom.cpp");

    cc::Build::new()
        .cpp(true)
        .file("ext/vswhom.cpp")
        .compile("vswhom");

    println!("cargo:rustc-link-lib=dylib=OleAut32");
    println!("cargo:rustc-link-lib=dylib=Ole32");
}
