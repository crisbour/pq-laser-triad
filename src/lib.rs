#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[test]
fn test_get_version() {
    // import all of our stuff
    use super::super::*;
    use super::*;
    println!("OUT_DIR={}", env!("OUT_DIR"));
    unsafe {
        let mut cLibVersion: [i8; 256] = [0; 256];
        // TODO: Change return to Error type
        let iRetVal = SEPIA2_LIB_GetVersion(cLibVersion.as_mut_ptr());
        assert_eq!(iRetVal, 0);
        println!("Version: {}", std::ffi::CStr::from_ptr(cLibVersion.as_ptr()).to_str().unwrap());
    }
}

// TODO: PQ-RPC development plan
// - [ ] Generate C bindings for the PQ library
//   - [ ] stdbool.h missing for Sepia2_Def.h; Need stdenv.cc.cc.lib added to LD_LIBRARY_PATH
// - [ ] Write shims that transform pointer mutation output parameters in req->resp function calls
//   - macro to generate shims for the RPC that transform outputs in immutable response objects
// - [ ] Generate protobuf definition from the later API
// - [ ] Tonic server implementation that handles gRPC interface to this API


