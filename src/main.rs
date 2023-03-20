use anyhow::{bail, Result};
use ffi::convert_step_to_stl;

#[cxx::bridge(namespace = "MyTest")]
mod ffi {
    unsafe extern "C++" {
        include!("test-occ-bindings/src/OCCTWrapper.hpp");

        fn convert_step_to_stl(step_file_path: String, stl_file_path: String) -> bool;
    }
}

fn main() -> Result<()> {
    let step_file_path = "./data/test.STEP".to_string();
    let stl_file_path = "./data/test.stl".to_string();
    if !convert_step_to_stl(step_file_path, stl_file_path) {
        bail!("Failed to convert STEP to STL");
    }
    Ok(())
}
