#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/api.rs"));

// FIXME: How to bundle the dll in the generated executable for Windows?
// I.e. I want the DLL to be part of the executable instead of being loaded at runtime, because the
// path won't match between where the executable was build and where it's used
/// `Sepia2` dynamic library absolute path.
use winapi::um::libloaderapi::{FindResourceW, LoadResource, SizeofResource, LockResource, FreeResource};
use winapi::um::winuser::RT_RCDATA;
use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

unsafe fn extract_and_load_dll() -> Result<Sepia2_Lib, Box<dyn std::error::Error>> {
    // Convert resource name to wide string
    let resource_name = OsStr::new("IDR_MYDLL").encode_wide().chain(Some(0)).collect::<Vec<_>>();

    // Locate the resource
    let h_resource = FindResourceW(
        ptr::null_mut(),
        resource_name.as_ptr(),
        RT_RCDATA
    );

    // Load and lock the resource
    let h_global = LoadResource(ptr::null_mut(), h_resource);
    let dll_data = LockResource(h_global) as *const u8;
    let dll_size = SizeofResource(ptr::null_mut(), h_resource) as usize;

    // Write to temp file
    let temp_path = std::env::temp_dir().join("vendor_library.dll");
    std::fs::write(&temp_path, std::slice::from_raw_parts(dll_data, dll_size))?;

    let sepia2_lib = unsafe { Sepia2_Lib::new(&temp_path) }.expect("Unable to load Sepia2 dynamic library!");


    // Cleanup (optional)
    FreeResource(h_global);
    // TODO: Cleanup the temp when the program exits
    //std::fs::remove_file(temp_path)?;

    Ok(sepia2_lib)
}

// TODO: If needed or multithreading, wrap the loaded lib in a mutex
#[cfg(target_os = "windows")]
lazy_static::lazy_static! {
    pub static ref SEPIA2: Sepia2_Lib = unsafe { extract_and_load_dll().unwrap() };
}

// ----------------------------------------------------------------
// Test the raw API with global static libloading
// ----------------------------------------------------------------

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
