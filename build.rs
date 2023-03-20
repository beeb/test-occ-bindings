fn main() {
    println!("cargo:rustc-link-search=/home/valentin/.nix-profile/lib");

    cxx_build::bridge("src/main.rs")
        .file("src/OCCTWrapper.cpp")
        .include("/home/valentin/.nix-profile/include/opencascade")
        .include("src")
        .flag_if_supported("-std=c++14")
        .compile("occtwrapper");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/OCCTWrapper.cpp");
    println!("cargo:rerun-if-changed=src/OCCTWrapper.hpp");
}
