use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=./libsepia2/Sepia2_Lib64.dll");

    // TODO: Inspect libraries that libsepia depends on when installing and wrap them in here
    // Tell cargo to make rustc link the system libraries that libsepia is depending on
    // println!("cargo:rustc-link-lib:<library>")

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let c_dir = env::current_dir().unwrap().join("libsepia2");
    println!("cargo:rustc-link-search={}", c_dir.display());
    println!("cargo:rustc-link-lib=dylib=Sepia2_Lib64");

    // NOTE: Only generate bindings for Prima PQ Laser for now
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Lib generic functions
        .allowlist_function("SEPIA2_LIB_DecodeError")
        .allowlist_function("SEPIA2_LIB_GetLibUSBVersion")
        .allowlist_function("SEPIA2_LIB_GetVersion")
        .allowlist_function("SEPIA2_LIB_IsRunningOnWine")
        // USB comm functions
        .allowlist_function("SEPIA2_USB_CloseDevice")
        .allowlist_function("SEPIA2_USB_GetStrDescrByIdx")
        .allowlist_function("SEPIA2_USB_GetStrDescriptor")
        .allowlist_function("SEPIA2_USB_IsOpenDevice")
        .allowlist_function("SEPIA2_USB_OpenDevice")
        .allowlist_function("SEPIA2_USB_OpenGetSerNumAndClose")
        // FIXME: Do I need the FW API?
        // FWR API
        .allowlist_function("SEPIA2_FWR_CreateSupportRequestText")
        .allowlist_function("SEPIA2_FWR_DecodeErrPhaseName")
        .allowlist_function("SEPIA2_FWR_FreeModuleMap")
        .allowlist_function("SEPIA2_FWR_GetLastError")
        .allowlist_function("SEPIA2_FWR_GetModuleInfoByMapIdx")
        .allowlist_function("SEPIA2_FWR_GetModuleMap")
        .allowlist_function("SEPIA2_FWR_GetUptimeInfoByMapIdx")
        .allowlist_function("SEPIA2_FWR_GetVersion")
        .allowlist_function("SEPIA2_FWR_GetWorkingMode")
        .allowlist_function("SEPIA2_FWR_RollBackToPermanentValues")
        .allowlist_function("SEPIA2_FWR_SetWorkingMode")
        .allowlist_function("SEPIA2_FWR_StoreAsPermanentValues")
        // Prima PQ control
        .allowlist_function("SEPIA2_PRI_DecodeOperationMode")
        .allowlist_function("SEPIA2_PRI_DecodeTriggerSource")
        .allowlist_function("SEPIA2_PRI_DecodeWavelength")
        .allowlist_function("SEPIA2_PRI_GetDeviceInfo")
        .allowlist_function("SEPIA2_PRI_GetFrequency")
        .allowlist_function("SEPIA2_PRI_GetFrequencyLimits")
        .allowlist_function("SEPIA2_PRI_GetGateHighImpedance")
        .allowlist_function("SEPIA2_PRI_GetGatingData")
        .allowlist_function("SEPIA2_PRI_GetGatingEnabled")
        .allowlist_function("SEPIA2_PRI_GetGatingLimits")
        .allowlist_function("SEPIA2_PRI_GetIntensity")
        .allowlist_function("SEPIA2_PRI_GetOperationMode")
        .allowlist_function("SEPIA2_PRI_GetTriggerLevel")
        .allowlist_function("SEPIA2_PRI_GetTriggerLevelLimits")
        .allowlist_function("SEPIA2_PRI_GetTriggerSource")
        .allowlist_function("SEPIA2_PRI_GetWavelengthIdx")
        .allowlist_function("SEPIA2_PRI_SetFrequency")
        .allowlist_function("SEPIA2_PRI_SetGateHighImpedance")
        .allowlist_function("SEPIA2_PRI_SetGatingData")
        .allowlist_function("SEPIA2_PRI_SetGatingEnabled")
        .allowlist_function("SEPIA2_PRI_SetIntensity")
        .allowlist_function("SEPIA2_PRI_SetOperationMode")
        .allowlist_function("SEPIA2_PRI_SetTriggerLevel")
        .allowlist_function("SEPIA2_PRI_SetTriggerSource")
        .allowlist_function("SEPIA2_PRI_SetWavelengthIdx")
        //.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let err_bindings = bindgen::Builder::default()
        .header("./libsepia2/include/Sepia2_ErrorCodes.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    //let type_bindings = bindgen::Builder::default()
    //    .header("libsepia2/include/Sepia2_Def.h")
    //    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //    //.allowlist_type("T_PRI_Constants")
    //    //.allowlist_type("T_ptrPRI_Constants")
    //    .generate()
    //    .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    err_bindings
        .write_to_file(out_path.join("error_bindings.rs"))
        .expect("Couldn't write bindings!");

    //type_bindings
    //    .write_to_file(out_path.join("type_bindings.rs"))
    //    .expect("Couldn't write bindings!");
}
