#![allow(missing_docs)]

use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Emit the instructions
    EmitBuilder::builder().git_sha(true).emit()?;
    Ok(())
}
