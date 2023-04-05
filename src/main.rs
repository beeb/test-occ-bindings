use std::path::Path;

use anyhow::{bail, Result};
use ffi::convert_step_to_stl;

const STEP_TRANS_CHORD_ERROR: f64 = 0.005;
const STEP_TRANS_ANGLE_RES: f64 = 1.;

#[cxx::bridge(namespace = "MyTest")]
mod ffi {
    unsafe extern "C++" {
        include!("test-occ-bindings/src/OCCTWrapper.hpp");

        fn convert_step_to_stl(
            step_file_path: String,
            stl_file_path: String,
            chord_error: f64,
            angle_res: f64,
        ) -> bool;
    }
}

fn main() -> Result<()> {
    let step_file_path = Path::new("./data/test.STEP").canonicalize().unwrap();
    let stl_file_path = step_file_path.with_extension("stl");
    if !convert_step_to_stl(
        step_file_path.to_string_lossy().into_owned(),
        stl_file_path.to_string_lossy().into_owned(),
        STEP_TRANS_CHORD_ERROR,
        STEP_TRANS_ANGLE_RES,
    ) {
        bail!("Failed to convert STEP to STL");
    }
    Ok(())
}
