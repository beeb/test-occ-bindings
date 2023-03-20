fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/OCCTWrapper.cpp")
        .include("/home/valentin/.nix-profile/include/opencascade")
        .flag_if_supported("-std=c++14")
        .compile("occtwrapper");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/OCCTWrapper.cpp");
    println!("cargo:rerun-if-changed=src/OCCTWrapper.hpp");
}
