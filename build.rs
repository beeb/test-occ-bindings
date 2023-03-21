fn main() {
    let home_dir = home::home_dir().unwrap();

    println!(
        "cargo:rustc-link-search={}",
        home_dir.join(".nix-profile/lib").to_string_lossy()
    );
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
    println!("cargo:rustc-link-lib=TKMesh");

    cxx_build::bridge("src/main.rs")
        .file("src/OCCTWrapper.cpp")
        .include(home_dir.join(".nix-profile/include/opencascade"))
        .include("src")
        .flag_if_supported("-std=c++14")
        .compile("occtwrapper");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/OCCTWrapper.cpp");
    println!("cargo:rerun-if-changed=src/OCCTWrapper.hpp");
}
