use prost::Message;
use prost_types::FileDescriptorSet;
use std::fs;

const DESCRIPTOR_PATH: &str = "descriptor.binpb";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let b = fs::read(DESCRIPTOR_PATH)?;
    let fds = FileDescriptorSet::decode(b.as_slice())?;

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_fds(fds)?;

    Ok(())
}
