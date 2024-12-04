#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[test]
fn test_get_version() {
    println!(env!("OUT_DIR"));
    unsafe {
        let mut cLibVersion: [i8; 256] = [0; 256];
        let iRetVal = SEPIA2_LIB_GetVersion(cLibVersion.as_mut_ptr());
        assert_eq!(iRetVal, 0);
        println!("Version: {}", std::ffi::CStr::from_ptr(cLibVersion.as_ptr()).to_str().unwrap());
    }

}
