#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod api {
    include!(concat!(env!("OUT_DIR"), "/api.rs"));

    // TODO: Test 32bit version with wine
    /// `Sepia2` dynamic library absolute path.
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub const SEPIA2_PATH: &str = concat!(env!("OUT_DIR"), "\\Sepia2_Lib.dll");
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    pub const SEPIA2_PATH: &str = concat!(env!("OUT_DIR"), "\\Sepia2_Lib64.dll");

    // TODO:Load library in lazy_static to be used across the library directly
    lazy_static::lazy_static! {
        pub static ref SEPIA2: Sepia2_Lib = {
            unsafe { Sepia2_Lib::new(SEPIA2_PATH) }.expect("Unable to load Sepia2 dynamic library!")
        };
    }

    #[cfg(test)]
    mod test {
        // import all of our stuff
        use super::*;
        use test_log::test;
        use log::info;
        // Test API version
        #[test]
        fn test_get_version() {
            println!("Try to load Sepia2 lib from {}", SEPIA2_PATH);
            let mut cLibVersion: [i8; 256] = [0; 256];
            let iRetVal = unsafe { SEPIA2.SEPIA2_LIB_GetVersion(cLibVersion.as_mut_ptr()) };
            assert_eq!(iRetVal, 0);
            info!("Version: {}", unsafe {
                std::ffi::CStr::from_ptr(cLibVersion.as_ptr())
                    .to_str()
                    .unwrap()
            });
        }

        // Test error codes
        #[test]
        fn test_error_msg() {
            println!("Try to load Sepia2 lib from {}", SEPIA2_PATH);
            let mut cErrorString: [i8; 256] = [0; 256];

            // Test error code -1001
            let iRetVal =
                unsafe { SEPIA2.SEPIA2_LIB_DecodeError(-1001, cErrorString.as_mut_ptr()) };
            info!(
                "Match succesful of error string '{}' for code {}",
                unsafe {
                    std::ffi::CStr::from_ptr(cErrorString.as_ptr())
                        .to_str()
                        .unwrap()
                },
                -1001
            );
            assert_eq!(iRetVal, 0);
            assert_eq!(
                unsafe {
                    std::ffi::CStr::from_ptr(cErrorString.as_ptr())
                        .to_str()
                        .unwrap()
                },
                "FW: memory allocation error"
            );

            // Test error code -9999
            let iRetVal =
                unsafe { SEPIA2.SEPIA2_LIB_DecodeError(-9999, cErrorString.as_mut_ptr()) };
            info!(
                "Match succesful of error string '{}' for code {}",
                unsafe {
                    std::ffi::CStr::from_ptr(cErrorString.as_ptr())
                        .to_str()
                        .unwrap()
                },
                -9999
            );
            assert_eq!(iRetVal, 0);
            assert_eq!(
                unsafe {
                    std::ffi::CStr::from_ptr(cErrorString.as_ptr())
                        .to_str()
                        .unwrap()
                },
                "LIB: unknown error code"
            );
        }

        // Test USB Lib version
        #[test]
        fn test_usb_version() {
            println!("Try to load Sepia2 lib from {}", SEPIA2_PATH);
            let mut cLibVersion: [i8; 256] = [0; 256];
            let iRetVal =
                unsafe { SEPIA2.SEPIA2_LIB_GetLibUSBVersion(cLibVersion.as_mut_ptr()) };
            assert_eq!(iRetVal, 0);
            info!("USB Version: {}", unsafe {
                std::ffi::CStr::from_ptr(cLibVersion.as_ptr())
                    .to_str()
                    .unwrap()
            });
        }

        // Test Wine
        #[test]
        fn test_wine() {
            println!("Try to load Sepia2 lib from {}", SEPIA2_PATH);
            let mut wine_bool: u8 = 0;
            let iRetVal = unsafe { SEPIA2.SEPIA2_LIB_IsRunningOnWine(&mut wine_bool) };
            assert_eq!(iRetVal, 0);
            info!("Is using wine? Response = {}", wine_bool);
        }
    }
}

pub mod errors {
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}

pub mod types {
    include!(concat!(env!("OUT_DIR"), "/types.rs"));
}

#[cfg(test)]
mod test {
    // import all of our stuff
    use super::api::*;
    use super::errors::*;
    use super::types::*;
    use super::*;
    use test_log::test;
    use log::{info,warn};

    #[test]
    fn test_usb_list() {
        println!("Try to load Sepia2 lib from {}", SEPIA2_PATH);

        for i in 0..SEPIA2_MAX_USB_DEVICES {
            let mut cProductModel: [i8; 256] = [0; 256];
            let mut cSerialNum: [i8; 256] = [0; 256];
            let iRetVal = unsafe {
                SEPIA2.SEPIA2_USB_OpenGetSerNumAndClose(
                    i as i32,
                    cProductModel.as_mut_ptr(),
                    cSerialNum.as_mut_ptr(),
                )
            };
            if iRetVal != 0 {
                warn!(
                    "Error calling SEPIA2_USB_OpenGetSerNumAndClose with err: {}",
                    iRetVal
                );
                continue;
            }
            info!(
                "USB device no.={} with model:{} and serial:{}",
                i,
                unsafe {
                    std::ffi::CStr::from_ptr(cProductModel.as_ptr())
                        .to_str()
                        .unwrap()
                },
                unsafe {
                    std::ffi::CStr::from_ptr(cSerialNum.as_ptr())
                        .to_str()
                        .unwrap()
                },
            );
        }
    }
}
