use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    // TODO: Inspect libraries that Sepia2_Lib depends on when installing and wrap them in here
    // Tell cargo to make rustc link against them, especially useful for Wine to work
    // println!("cargo:rustc-link-lib:<library>")

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=src/bindings.rs");

    let (target_os, target_arch) = get_target_os_and_arch();
    let (lib_name, lib_extension) = get_lib_name_and_extension(&target_os, &target_arch);
    let src_dir = setup_src_dir(&target_os, &target_arch);
    let target_dir = setup_target_dir(&target_os);
    setup_lib(&lib_name, &lib_extension, &src_dir, &target_dir);

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
        .clang_arg("-v")
        // To retain documentation comments, enable:
        //.clang_arg("-fretain-comments-from-system-headers")
        .dynamic_library_name("Sepia2_Lib")
        .dynamic_link_require_all(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let err_bindings = bindgen::Builder::default()
        .header("./native/include/Sepia2_ErrorCodes.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let constants_bindings = bindgen::Builder::default()
        .header("native/include/Sepia2_Def.h")
        .header("native/include/Portabt.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_type(".*")
        .generate()
        .expect("Unable to generate bindings");

    let types_bindings = bindgen::Builder::default()
        .header("native/include/Sepia2_Def.h")
        .header("native/include/Portabt.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // FIXME: Allow only what is useful for Prima Laser
        .allowlist_type(".*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(target_dir.join("api.rs"))
        .expect("Couldn't write bindings!");

    err_bindings
        .write_to_file(target_dir.join("errors.rs"))
        .expect("Couldn't write bindings!");

    constants_bindings
        .write_to_file(target_dir.join("constants.rs"))
        .expect("Couldn't write bindings!");

    types_bindings
        .write_to_file(target_dir.join("types.rs"))
        .expect("Couldn't write bindings!");
}

#[allow(clippy::enum_variant_names)]
enum OS {
    Windows,
    Linux,
    MacOS,
}

#[derive(Debug)]
enum Arch {
    X86,
    X64,
    ARM64,
}

fn get_target_os_and_arch() -> (OS, Arch) {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match (target_os.as_str(), target_arch.as_str()) {
        ("windows", "x86_64") => (OS::Windows, Arch::X64),
        ("windows", "x86") => (OS::Windows, Arch::X86),
        ("linux", "x86_64") => (OS::Linux, Arch::X64),
        ("macos", "x86_64") => (OS::MacOS, Arch::X64),
        ("macos", "aarch64") => (OS::MacOS, Arch::ARM64),
        _ => panic!(
            "Unsupported target platform: {} ({})",
            target_os, target_arch
        ),
    }
}

fn get_lib_name_and_extension(target_os: &OS, target_arch: &Arch) -> (String, String) {
    let lib_name = if matches!(target_os, OS::Windows) {
        match target_arch {
            Arch::X86 => "Sepia2_Lib",
            Arch::X64 => "Sepia2_Lib64",
            unsupported => panic!("Architecture {:?} not supported for Windows library", unsupported),
        }
    } else {
        panic!("Only windows library exists");
    };
    let lib_extension = match target_os {
        OS::Windows => ".dll",
        OS::Linux => ".so",
        OS::MacOS => ".dylib",
    };
    (lib_name.into(), lib_extension.into())
}

fn setup_src_dir(target_os: &OS, target_arch: &Arch) -> PathBuf {
    let subfolder = match (target_os, target_arch) {
        (OS::Windows, Arch::X64) => "win-x64",
        (OS::Windows, Arch::X86) => "win-x86",
        (OS::Linux, Arch::X64) => "lin-x64",
        (OS::MacOS, Arch::X64) => "mac-x64",
        (OS::MacOS, Arch::ARM64) => "mac-arm64",
        _ => "",
    };
    let src_dir = PathBuf::from(format!("native/lib/{}", subfolder))
        .canonicalize()
        .expect("Unable to canonicalize Sepia2_Lib directory path!");
    println!(
        "cargo:rustc-link-search=native={}",
        src_dir.to_str().unwrap()
    );
    src_dir
}

fn setup_target_dir(target_os: &OS) -> PathBuf {
    let target_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!(
        "cargo:rustc-link-search=native={}",
        target_dir.to_str().unwrap()
    );
    if matches!(target_os, OS::Linux | OS::MacOS) {
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}",
            target_dir.to_str().unwrap()
        );
    }
    target_dir
}

fn setup_lib(lib_name: &str, lib_extension: &str, src_dir: &Path, target_dir: &Path) {
    let file_name = format!("{}{}", lib_name, lib_extension);
    let src_path = src_dir.join(&file_name);
    let target_path = target_dir.join(&file_name);
    fs::copy(&src_path, &target_path)
        .expect("Unable to copy Sepia2_Lib library to the target directory!");
    // FIXME: Understand why dylib linking doesn't work with DLL
    //println!("cargo:rustc-link-lib=dylib={}", lib_name);
}
