#![allow(non_snake_case)]

// Crate imports
use sepia2_sys::api::*;
use crate::error::Sepia2Error;
use crate::types::*;

// C types imports as casting helpers
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

type Result<T> = std::result::Result<T, Sepia2Error>;

fn to_string(c_str: *const c_char) -> String {
    unsafe { CStr::from_ptr(c_str) }
        .to_string_lossy()
        .into_owned()
}

// WARN: In the FFI that follows I used Rust primitives when possible instead of the std::os::raw
// Is there a concern about the type equivalence across architectures? i.e. `c_int === i32`

// TODO: Use constants from Sepia2_Def.h to set length of buffers instead of hardcoded values

// TODO: How to combine Sepia2Error into enum with any other possible Error type that is being used
// here, such that we can catch other errors as well instead calling unwrap?
// - ANSWER: Use Box<dyn std::error::Error> which accepts any type that implements the Error trait
pub fn LIB_DecodeError(err_code: i32) -> Result<String> {
    let mut c_err_string: [c_char; 64] = [0; 64];
    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_LIB_DecodeError(err_code, c_err_string.as_mut_ptr())
    }) {
        Ok(_) => Ok(to_string(c_err_string.as_ptr())),
        Err(e) => Err(e),
    }
}
pub fn LIB_GetVersion() -> Result<String> {
    let mut c_lib_version: [c_char; 12] = [0; 12];
    match Sepia2Error::from_raw(unsafe { SEPIA2.SEPIA2_LIB_GetVersion(c_lib_version.as_mut_ptr()) })
    {
        Ok(_) => Ok(to_string(c_lib_version.as_ptr())),
        Err(e) => Err(e),
    }
}
pub fn LIB_GetLibUSBVersion() -> Result<String> {
    let mut cLibUSBVersion: [c_char; 3] = [0; 3];
    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_LIB_GetLibUSBVersion(cLibUSBVersion.as_mut_ptr())
    }) {
        Ok(_) => Ok(to_string(cLibUSBVersion.as_ptr())),
        Err(e) => Err(e),
    }
}
pub fn LIB_IsRunningOnWine() -> Result<bool> {
    let mut pbIsRunningOnWine: u8 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_LIB_IsRunningOnWine(&mut pbIsRunningOnWine)
    }) {
        Ok(_) => Ok(pbIsRunningOnWine != 0),
        Err(e) => Err(e),
    }
}

pub fn USB_OpenDevice(dev_idx: i32) -> Result<UsbDevice> {
    let mut cProductModel: [c_char; 64] = [0; 64];
    let mut cSerialNumber: [c_char; 64] = [0; 64];

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_USB_OpenDevice(
            dev_idx, // iDevIdx,
            cProductModel.as_mut_ptr(),
            cSerialNumber.as_mut_ptr(),
        )
    }) {
        Ok(_) => Ok(UsbDevice {
            product_model: to_string(cProductModel.as_ptr()),
            serial_number: to_string(cSerialNumber.as_ptr()),
        }),
        Err(e) => Err(e),
    }
}
pub fn USB_OpenGetSerNumAndClose(dev_idx: i32) -> Result<UsbDevice> {
    let mut cProductModel: [c_char; 64] = [0; 64];
    let mut cSerialNumber: [c_char; 64] = [0; 64];

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_USB_OpenGetSerNumAndClose(
            dev_idx, // iDevIdx,
            cProductModel.as_mut_ptr(),
            cSerialNumber.as_mut_ptr(),
        )
    }) {
        Ok(_) => Ok(UsbDevice {
            product_model: to_string(cProductModel.as_ptr()),
            serial_number: to_string(cSerialNumber.as_ptr()),
        }),
        Err(e) => Err(e),
    }
}
pub fn USB_GetStrDescriptor(dev_idx: i32) -> Result<String> {
    let mut cDescriptor: [c_char; 64] = [0; 64];

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_USB_GetStrDescriptor(
            dev_idx, // iDevIdx,
            cDescriptor.as_mut_ptr(),
        )
    }) {
        Ok(_) => Ok(to_string(cDescriptor.as_ptr())),
        Err(e) => Err(e),
    }
}
pub fn USB_GetStrDescrByIdx(dev_idx: i32, descr_idx: i32) -> Result<String> {
    let mut cDescriptor: [c_char; 64] = [0; 64];

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_USB_GetStrDescrByIdx(
            dev_idx,   // iDevIdx,
            descr_idx, // iDescrIdx,
            cDescriptor.as_mut_ptr(),
        )
    }) {
        Ok(_) => Ok(to_string(cDescriptor.as_ptr())),
        Err(e) => Err(e),
    }
}
pub fn USB_IsOpenDevice(dev_idx: i32) -> Result<bool> {
    let mut pbIsOpenDevice: u8 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_USB_IsOpenDevice(
            dev_idx, // iDevIdx,
            &mut pbIsOpenDevice,
        )
    }) {
        Ok(_) => Ok(pbIsOpenDevice != 0),
        Err(e) => Err(e),
    }
}
pub fn USB_CloseDevice(dev_idx: i32) -> Result<()> {
    Sepia2Error::from_raw(unsafe { SEPIA2.SEPIA2_USB_CloseDevice(dev_idx) })
}
pub fn FWR_DecodeErrPhaseName(err_phase: i32) -> Result<String> {
    let mut cErrorPhase: [c_char; 64] = [0; 64];

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_FWR_DecodeErrPhaseName(
            err_phase, // iErrPhase,
            cErrorPhase.as_mut_ptr(),
        )
    }) {
        Ok(_) => Ok(to_string(cErrorPhase.as_ptr())),
        Err(e) => Err(e),
    }
}
pub fn FWR_GetVersion(dev_idx: i32) -> Result<String> {
    let mut cFWVersion: [c_char; 64] = [0; 64];

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_FWR_GetVersion(
            dev_idx, // iDevIdx,
            cFWVersion.as_mut_ptr(),
        )
    }) {
        Ok(_) => Ok(to_string(cFWVersion.as_ptr())),
        Err(e) => Err(e),
    }
}

pub fn FWR_GetLastError(dev_idx: i32) -> Result<FwrError> {
    let mut piErrCode: i32 = 0;
    let mut piPhase: i32 = 0;
    let mut piLocation: i32 = 0;
    let mut piSlot: i32 = 0;
    let mut cCondition: [c_char; 64] = [0; 64];

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_FWR_GetLastError(
            dev_idx, // iDevIdx,
            &mut piErrCode,
            &mut piPhase,
            &mut piLocation,
            &mut piSlot,
            cCondition.as_mut_ptr(),
        )
    }) {
        Ok(_) => Ok(FwrError {
            err_code: piErrCode,
            phase: piPhase,
            location: piLocation,
            slot: piSlot,
            condition: to_string(cCondition.as_ptr()),
        }),
        Err(e) => Err(e),
    }
}
pub fn FWR_GetWorkingMode(dev_idx: i32) -> Result<i32> {
    let mut piMode: i32 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_FWR_GetWorkingMode(
            dev_idx, // iDevIdx,
            &mut piMode,
        )
    }) {
        Ok(_) => Ok(piMode),
        Err(e) => Err(e),
    }
}
pub fn FWR_SetWorkingMode(dev_idx: i32, mode: i32) -> Result<()> {
    Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_FWR_SetWorkingMode(
            dev_idx, // iDevIdx,
            mode,    // iMode,
        )
    })
}
pub fn FWR_RollBackToPermanentValues(dev_idx: i32) -> Result<()> {
    Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_FWR_RollBackToPermanentValues(
            dev_idx, // iDevIdx,
        )
    })
}
pub fn FWR_StoreAsPermanentValues(dev_idx: i32) -> Result<()> {
    Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_FWR_StoreAsPermanentValues(
            dev_idx, // iDevIdx,
        )
    })
}
pub fn FWR_GetModuleMap(dev_idx: i32, perform_restart: bool) -> Result<i32> {
    let mut piModuleCount: i32 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_FWR_GetModuleMap(
            dev_idx,                // iDevIdx,
            perform_restart as i32, // iPerformRestart,
            &mut piModuleCount,
        )
    }) {
        Ok(_) => Ok(piModuleCount),
        Err(e) => Err(e),
    }
}

pub fn FWR_GetModuleInfoByMapIdx(dev_idx: i32, map_idx: i32) -> Result<ModuleInfo> {
    let mut piSlotId: i32 = 0;
    let mut pbIsPrimary: u8 = 0;
    let mut pbIsBackPlane: u8 = 0;
    let mut pbHasUptimeCounter: u8 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_FWR_GetModuleInfoByMapIdx(
            dev_idx, // iDevIdx,
            map_idx, // iMapIdx,
            &mut piSlotId,
            &mut pbIsPrimary,
            &mut pbIsBackPlane,
            &mut pbHasUptimeCounter,
        )
    }) {
        Ok(_) => Ok(ModuleInfo {
            slot_id: piSlotId,
            is_primary: pbIsPrimary != 0,
            is_back_plane: pbIsBackPlane != 0,
            has_utc: pbHasUptimeCounter != 0,
        }),
        Err(e) => Err(e),
    }
}

pub fn FWR_GetUptimeInfoByMapIdx(dev_idx: i32, map_idx: i32) -> Result<UptimeInfo> {
    let mut pulMainPowerUp: u32 = 0;
    let mut pulActivePowerUp: u32 = 0;
    let mut pulScaledPowerUp: u32 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_FWR_GetUptimeInfoByMapIdx(
            dev_idx, // iDevIdx,
            map_idx, // iMapIdx,
            &mut pulMainPowerUp,
            &mut pulActivePowerUp,
            &mut pulScaledPowerUp,
        )
    }) {
        Ok(_) => Ok(UptimeInfo {
            main_pwr_up: pulMainPowerUp,
            active_pwr_up: pulActivePowerUp,
            scaled_pwr_up: pulScaledPowerUp,
        }),
        Err(e) => Err(e),
    }
}

pub fn FWR_CreateSupportRequestText(dev_idx: i32, fwr_req: FwrRequestSupport) -> Result<String> {
    let mut cBuffer: [c_char; 64] = [0; 64];

    let preamble = CString::new(fwr_req.preamble).expect("preamble contains null bytes");
    let calling_sw = CString::new(fwr_req.calling_sw).expect("preamble contains null bytes");

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_FWR_CreateSupportRequestText(
            dev_idx, // iDevIdx,
            preamble.into_raw(),   // cPreamble,
            calling_sw.into_raw(), // cCallingSW,
            fwr_req.options,             // iOptions,
            fwr_req.buffer,              // iBufferLen,
            cBuffer.as_mut_ptr(),
        )
    }) {
        Ok(_) => Ok(to_string(cBuffer.as_ptr())),
        Err(e) => Err(e),
    }
}

// NOTE: Call this at the end of program or end of use of the laser to dealloc memory for the
// ModuleMap
pub fn FWR_FreeModuleMap(dev_idx: i32) -> Result<()> {
    Sepia2Error::from_raw(unsafe { SEPIA2.SEPIA2_FWR_FreeModuleMap(dev_idx) })
}

pub fn PRI_GetDeviceInfo(dev_idx: i32, slot_id: i32) -> Result<PrimaDevInfo> {
    let mut pcDeviceID: [c_char; 64] = [0; 64];
    let mut pcDeviceType: [c_char; 64] = [0; 64];
    let mut pcFW_Version: [c_char; 64] = [0; 64];
    let mut piWL_Count: i32 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_GetDeviceInfo(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            pcDeviceID.as_mut_ptr(),
            pcDeviceType.as_mut_ptr(),
            pcFW_Version.as_mut_ptr(),
            &mut piWL_Count,
        )
    }) {
        Ok(_) => Ok(PrimaDevInfo {
            device_id: to_string(pcDeviceID.as_ptr()),
            device_type: to_string(pcDeviceType.as_ptr()),
            fw_version: to_string(pcFW_Version.as_ptr()),
            wl_count: piWL_Count,
        }),
        Err(e) => Err(e),
    }
}

pub fn PRI_DecodeOperationMode(
    dev_idx: i32,
    slot_id: i32,
    oper_mode_idx: i32,
) -> Result<PrimaModeInfo> {
    let mut pcOperMode: [c_char; 64] = [0; 64];

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_DecodeOperationMode(
            dev_idx,       // iDevIdx,
            slot_id,       // iSlotId,
            oper_mode_idx, // iOperModeIdx,
            pcOperMode.as_mut_ptr(),
        )
    }) {
        Ok(_) => Ok(PrimaModeInfo {
            oper_mode_idx,
            oper_mode: to_string(pcOperMode.as_ptr()),
        }),
        Err(e) => Err(e),
    }
}
pub fn PRI_GetOperationMode(dev_idx: i32, slot_id: i32) -> Result<i32> {
    let mut piOperModeIdx: i32 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_GetOperationMode(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            &mut piOperModeIdx,
        )
    }) {
        Ok(_) => Ok(piOperModeIdx),
        Err(e) => Err(e),
    }
}
pub fn PRI_SetOperationMode(dev_idx: i32, slot_id: i32, oper_mode_idx: i32) -> Result<()> {
    Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_SetOperationMode(
            dev_idx,       // iDevIdx,
            slot_id,       // iSlotId,
            oper_mode_idx, // iOperModeIdx,
        )
    })
}

pub fn PRI_DecodeTriggerSource(
    dev_idx: i32,
    slot_id: i32,
    trg_src_idx: i32,
) -> Result<TriggerInfo> {
    let mut pcTrgSrc: [c_char; 64] = [0; 64];
    let mut pbFrequencyEnabled: u8 = 0;
    let mut pbTrigLevelEnabled: u8 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_DecodeTriggerSource(
            dev_idx,     // iDevIdx,
            slot_id,     // iSlotId,
            trg_src_idx, // iTrgSrcIdx,
            pcTrgSrc.as_mut_ptr(),
            &mut pbFrequencyEnabled,
            &mut pbTrigLevelEnabled,
        )
    }) {
        Ok(_) => Ok(TriggerInfo {
            trg_src_idx,
            trg_src: to_string(pcTrgSrc.as_ptr()),
            frequency_enabled: pbFrequencyEnabled != 0,
            trig_level_enabled: pbTrigLevelEnabled != 0,
        }),
        Err(e) => Err(e),
    }
}
pub fn PRI_GetTriggerSource(dev_idx: i32, slot_id: i32) -> Result<i32> {
    let mut piTrgSrcIdx: i32 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_GetTriggerSource(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            &mut piTrgSrcIdx,
        )
    }) {
        Ok(_) => Ok(piTrgSrcIdx),
        Err(e) => Err(e),
    }
}
pub fn PRI_SetTriggerSource(dev_idx: i32, slot_id: i32, trg_src_idx: i32) -> Result<()> {
    Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_SetTriggerSource(
            dev_idx,     // iDevIdx,
            slot_id,     // iSlotId,
            trg_src_idx, // iTrgSrcIdx,
        )
    })
}

pub fn PRI_GetTriggerLevelLimits(dev_idx: i32, slot_id: i32) -> Result<TriggerLevelInfo> {
    let mut piTrg_MinLvl: i32 = 0;
    let mut piTrg_MaxLvl: i32 = 0;
    let mut piTrg_LvlRes: i32 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_GetTriggerLevelLimits(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            &mut piTrg_MinLvl,
            &mut piTrg_MaxLvl,
            &mut piTrg_LvlRes,
        )
    }) {
        Ok(_) => Ok(TriggerLevelInfo {
            trg_min_lvl: piTrg_MinLvl,
            trg_max_lvl: piTrg_MaxLvl,
            trg_lvl_res: piTrg_LvlRes,
        }),
        Err(e) => Err(e),
    }
}
pub fn PRI_GetTriggerLevel(dev_idx: i32, slot_id: i32) -> Result<i32> {
    let mut piTrgLevel: i32 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_GetTriggerLevel(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            &mut piTrgLevel,
        )
    }) {
        Ok(_) => Ok(piTrgLevel),
        Err(e) => Err(e),
    }
}
pub fn PRI_SetTriggerLevel(dev_idx: i32, slot_id: i32, trg_level: i32) -> Result<()> {
    Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_SetTriggerLevel(
            dev_idx,   // iDevIdx,
            slot_id,   // iSlotId,
            trg_level, // iTrgLevel,
        )
    })
}
pub fn PRI_GetFrequencyLimits(dev_idx: i32, slot_id: i32) -> Result<(i32, i32)> {
    let mut piMinFreq: i32 = 0;
    let mut piMaxFreq: i32 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_GetFrequencyLimits(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            &mut piMinFreq,
            &mut piMaxFreq,
        )
    }) {
        Ok(_) => Ok((piMinFreq, piMaxFreq)),
        Err(e) => Err(e),
    }
}
pub fn PRI_GetFrequency(dev_idx: i32, slot_id: i32) -> Result<i32> {
    let mut piFrequency: i32 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_GetFrequency(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            &mut piFrequency,
        )
    }) {
        Ok(_) => Ok(piFrequency),
        Err(e) => Err(e),
    }
}
pub fn PRI_SetFrequency(dev_idx: i32, slot_id: i32, frequency: i32) -> Result<()> {
    Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_SetFrequency(
            dev_idx,   // iDevIdx,
            slot_id,   // iSlotId,
            frequency, // iFrequency,
        )
    })
}

pub fn PRI_GetGatingLimits(dev_idx: i32, slot_id: i32) -> Result<PrimaGatingInfo> {
    let mut piMinOnTime: i32 = 0;
    let mut piMaxOnTime: i32 = 0;
    let mut pbMinOffTimefactor: i32 = 0;
    let mut pbMaxOffTimefactor: i32 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_GetGatingLimits(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            &mut piMinOnTime,
            &mut piMaxOnTime,
            &mut pbMinOffTimefactor,
            &mut pbMaxOffTimefactor,
        )
    }) {
        Ok(_) => Ok(PrimaGatingInfo {
            min_on_time: piMinOnTime,
            max_on_time: piMaxOnTime,
            min_off_time_factor: pbMinOffTimefactor,
            max_off_time_factor: pbMaxOffTimefactor,
        }),
        Err(e) => Err(e),
    }
}
pub fn PRI_GetGatingData(dev_idx: i32, slot_id: i32) -> Result<(i32, i32)> {
    let mut piOnTime: i32 = 0;
    let mut piOffTimeFact: i32 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_GetGatingData(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            &mut piOnTime,
            &mut piOffTimeFact,
        )
    }) {
        Ok(_) => Ok((piOnTime, piOffTimeFact)),
        Err(e) => Err(e),
    }
}
pub fn PRI_SetGatingData(
    dev_idx: i32,
    slot_id: i32,
    on_time: i32,
    off_time_factor: i32,
) -> Result<()> {
    Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_SetGatingData(
            dev_idx,         // iDevIdx,
            slot_id,         // iSlotId,
            on_time,         // iOnTime,
            off_time_factor, // iOffTimeFact,
        )
    })
}
pub fn PRI_GetGatingEnabled(dev_idx: i32, slot_id: i32) -> Result<bool> {
    let mut pbGatingEnabled: u8 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_GetGatingEnabled(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            &mut pbGatingEnabled,
        )
    }) {
        Ok(_) => Ok(pbGatingEnabled != 0),
        Err(e) => Err(e),
    }
}
pub fn PRI_SetGatingEnabled(dev_idx: i32, slot_id: i32, gating_enabled: bool) -> Result<()> {
    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_SetGatingEnabled(
            dev_idx,              // iDevIdx,
            slot_id,              // iSlotId,
            gating_enabled as u8, // bGatingEnabled,
        )
    }) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
pub fn PRI_GetGateHighImpedance(dev_idx: i32, slot_id: i32) -> Result<bool> {
    let mut pbHighImpedance: u8 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_GetGateHighImpedance(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            &mut pbHighImpedance,
        )
    }) {
        Ok(_) => Ok(pbHighImpedance != 0),
        Err(e) => Err(e),
    }
}
pub fn PRI_SetGateHighImpedance(dev_idx: i32, slot_id: i32, high_impedance: bool) -> Result<()> {
    Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_SetGateHighImpedance(
            dev_idx,              // iDevIdx,
            slot_id,              // iSlotId,
            high_impedance as u8, // bHighImpedance,
        )
    })
}
pub fn PRI_DecodeWavelength(dev_idx: i32, slot_id: i32, wl_idx: i32) -> Result<i32> {
    let mut piWL: i32 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_DecodeWavelength(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            wl_idx,  // iWLIdx,
            &mut piWL,
        )
    }) {
        Ok(_) => Ok(piWL),
        Err(e) => Err(e),
    }
}
pub fn PRI_GetWavelengthIdx(dev_idx: i32, slot_id: i32) -> Result<i32> {
    let mut piWLIdx: i32 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_GetWavelengthIdx(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            &mut piWLIdx,
        )
    }) {
        Ok(_) => Ok(piWLIdx),
        Err(e) => Err(e),
    }
}
pub fn PRI_SetWavelengthIdx(dev_idx: i32, slot_id: i32, wl_idx: i32) -> Result<()> {
    Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_SetWavelengthIdx(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            wl_idx,  // iWLIdx,
        )
    })
}
pub fn PRI_GetIntensity(dev_idx: i32, slot_id: i32, wl_idx: i32) -> Result<u16> {
    let mut pwIntensity: u16 = 0;

    match Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_GetIntensity(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            wl_idx,  // iWLIdx,
            &mut pwIntensity,
        )
    }) {
        Ok(_) => Ok(pwIntensity),
        Err(e) => Err(e),
    }
}
pub fn PRI_SetIntensity(dev_idx: i32, slot_id: i32, wl_idx: i32, w_intensity: u16) -> Result<()> {
    Sepia2Error::from_raw(unsafe {
        SEPIA2.SEPIA2_PRI_SetIntensity(
            dev_idx,     // iDevIdx,
            slot_id,     // iSlotId,
            wl_idx,      // iWLIdx,
            w_intensity, // wIntensity,
        )
    })
}

// -----------------------------------------------------------------------------
// Test functions
// -----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use log::{error, info, warn, trace};
    use test_log;

    #[test]
    fn test_decode_error() {
        let err_str = match LIB_DecodeError(-1001) {
            Ok(result) => result,
            Err(e) => panic!("Error: {:?}", e),
        };
        assert_eq!(err_str, "FW: memory allocation error");
    }

    #[test]
    fn test_lib_version() {
        let version = match LIB_GetVersion() {
            Ok(result) => result,
            Err(e) => panic!("Error: {:?}", e),
        };
        info!("Sepia2_Lib Version: {}", version);
        // NOTE: Not expecting any updates
        assert!(version.contains("1.2.32") || version.contains("1.2.64"));
    }

    #[test]
    fn test_usb_version() {
        let usb_version = match LIB_GetLibUSBVersion() {
            Ok(result) => result,
            Err(e) => panic!("Error: {:?}", e),
        };
        info!("LibUSB Version: {}", usb_version);
    }

    #[test]
    fn test_wine() {
        let wine_bool = match LIB_IsRunningOnWine() {
            Ok(result) => result,
            Err(e) => panic!("Error: {:?}", e),
        };
        match wine_bool {
            true => info!("Running on Wine"),
            false => info!("Not running on Wine"),
        }
    }

    #[test]
    fn test_open_and_query_usb() {
        for _ in 0..10 {
            if USB_IsOpenDevice(0).expect("Couldn't querry state of USB device") {
                break;
            }
            trace!("Wait for USB(0) to be accessible");
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        let usb_device = match USB_OpenDevice(0) {
            Ok(result) => Some(result),
            Err(e) => {
                warn!("Couldn't open USB(0): {:?}", e);
                None
            }
        };
        if let Some(usb_device) = usb_device {
            let fwr_version = FWR_GetVersion(0);
            let usb_descriptor = USB_GetStrDescriptor(0);
            info!("USB index = {}", 0);
            info!("USB descriptor = {:?}", usb_descriptor);
            info!("FW-Version: {:?}", fwr_version);
            info!("USB device: {:?}", usb_device);

            let mut module_cnt = 0;
            match FWR_GetModuleMap(0, false) {
                Ok(cnt) => {
                    module_cnt = cnt;
                    info!("ModuleMap has been created")
                },
                Err(e) => error!("Couldn't create ModuleMap: {:?}", e),
            };

            for map_idx in 0..module_cnt {
                let module_info = FWR_GetModuleInfoByMapIdx(0, map_idx);
                info!("ModuleInfo: {:?}", module_info);
                if let Err(_) = module_info {
                    continue;
                }
            }

            // NOTE: Need to decode the module info to indentify PRI slot_id
            match PRI_GetFrequencyLimits(0, 100) {
                Ok(result) => info!("Frequency limits: {:?}", result),
                Err(e) => error!("Couldn't get frequency limits: {:?}", e),
            };

            info!("Freeing ModuleMap {:?}", FWR_FreeModuleMap(0));

            match USB_CloseDevice(0) {
                Ok(_) => info!("USB(0) device: {:?} has been closed successfuly", usb_device),
                Err(e) => {
                    panic!("Couldn't close device: {}", e);
                }
            }
        }
    }
}
