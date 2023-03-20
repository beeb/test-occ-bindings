fn main() {
    println!("cargo:rustc-link-search=/home/valentin/.nix-profile/lib");
    println!("cargo:rustc-link-lib=TKernel");
    println!("cargo:rustc-link-lib=TKMath");
    println!("cargo:rustc-link-lib=TKBRep");
    println!("cargo:rustc-link-lib=TKXSBase");
    println!("cargo:rustc-link-lib=TKService");
    println!("cargo:rustc-link-lib=TKV3d");
    println!("cargo:rustc-link-lib=TKOpenGl");
    println!("cargo:rustc-link-lib=TKIGES");
    println!("cargo:rustc-link-lib=TKSTEP");
    println!("cargo:rustc-link-lib=TKSTL");
    println!("cargo:rustc-link-lib=TKVRML");
    println!("cargo:rustc-link-lib=TKLCAF");

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
