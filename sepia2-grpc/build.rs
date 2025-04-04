extern crate prost_build;

// TODO: Write `.proto` and generate API from it using `prost`
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=proto/api.proto");

    prost_build::compile_protos(&["proto/api.proto"],
                                &["proto/"]).unwrap();
}
