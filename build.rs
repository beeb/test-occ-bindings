use std::{fs, io::Result};

fn main() -> Result<()> {
    let occt_paths = fs::read_dir("./occt")?;

    let dirs = occt_paths
        .into_iter()
        .filter_map(|i| i.ok().map(|i| i.path()))
        .filter(|i| i.is_dir());

    cxx_build::bridge("src/main.rs")
        .file("src/OCCTWrapper.cpp")
        .includes(dirs)
        .flag_if_supported("-std=c++14")
        .compile("occtwrapper");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/OCCTWrapper.cpp");
    println!("cargo:rerun-if-changed=src/OCCTWrapper.hpp");
    Ok(())
}
