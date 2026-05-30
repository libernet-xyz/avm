use prost_build;
use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["proto/version.proto", "proto/v1.proto"], &["proto/"])?;
    Ok(())
}
