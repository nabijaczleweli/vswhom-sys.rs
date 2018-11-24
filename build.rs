extern crate cc;


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
