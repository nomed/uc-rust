//! Build-time protobuf generation for the delivery adapter crate.
//!
//! This script compiles the versioned gRPC contract with a vendored `protoc` so
//! builds are reproducible and do not depend on host tooling. Generated types are
//! confined to `uc-adapters`; canonical application and domain crates must never
//! depend on them.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    unsafe { std::env::set_var("PROTOC", protoc) };
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&["../../proto/uc/runtime/v1/runtime.proto"], &["../../proto"])?;
    println!("cargo:rerun-if-changed=../../proto/uc/runtime/v1/runtime.proto");
    Ok(())
}
