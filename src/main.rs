use std::path::Path;

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
    let step_file_path = Path::new("./data/test.STEP").canonicalize().unwrap();
    let stl_file_path = step_file_path.parent().unwrap().join("test.stl");
    if !convert_step_to_stl(
        step_file_path.to_string_lossy().into_owned(),
        stl_file_path.to_string_lossy().into_owned(),
    ) {
        bail!("Failed to convert STEP to STL");
    }
    Ok(())
}
