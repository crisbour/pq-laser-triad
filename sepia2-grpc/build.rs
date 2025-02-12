use std::{env, path::PathBuf};

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=proto/api.proto");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .server_mod_attribute("attrs", "#[cfg(feature = \"server\")]")
        .client_mod_attribute("attrs", "#[cfg(feature = \"client\")]")
        .file_descriptor_set_path(out_dir.join("sepia2_rpc_descriptor.bin"))
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(true)
        .compile_protos(&["proto/api.proto"], &["proto/"])
        .unwrap();
}
