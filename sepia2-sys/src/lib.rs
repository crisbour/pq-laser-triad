// API only available to use from windows
#[cfg(target_os = "windows")]
pub mod api;

pub mod errors;
pub mod types;
pub mod constants;

// Convert Rust string to C-style string
#[allow(unused_macros)]
macro_rules! const_ptr_c_char {
    ($value:expr) => {
        format!("{}{}", $value, "\0").as_ptr().cast::<c_char>()
    };
}

#[allow(unused_imports)]
pub(crate) use const_ptr_c_char;

#[cfg(test)]
mod test {
    // import all of our stuff
    use super::api::*;
    use super::errors::*;
    use super::types::*;
    use super::constants::*;
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
