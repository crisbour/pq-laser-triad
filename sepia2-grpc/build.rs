extern crate tonic_build;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=proto/api.proto");

    tonic_build::configure()
        .server_mod_attribute("attrs", "#[cfg(feature = \"server\")]")
        .client_mod_attribute("attrs", "#[cfg(feature = \"client\")]")
        .build_server(true)
        .compile_protos(&["proto/api.proto"], &["proto/"])
        .unwrap();
}
