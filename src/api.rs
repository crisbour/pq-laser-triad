
type Result<T> = Result<T, Sepia2Error>;

struct Sepia2State {
    sepia2_lib: Sepia2_Lib,
};

pub fn LIB_DecodeError(err_code: i32) -> Result<String> {
    let mut c_err_string: ::std::os::raw::c_char;
    match Sepia2Error::from_raw(
        unsafe { SEPIA2_LIB_DecodeError(err_code, c_err_string.as_ptr()) }
    ) {
        Ok(_) => Ok(to_string(c_err_string)),
        Err(e) => Err(e),
    }
}
pub fn LIB_GetVersion() -> Result<String> {
    let mut c_lib_version: ::std::os::raw::c_char;
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_LIB_GetVersion(c_lib_version.as_ptr())
    }) {
        Ok(_) => Ok(to_string(c_lib_version)),
        Err(e) => Err(e),
    }
}
pub fn LIB_GetLibUSBVersion() -> Result<String> {
    let mut cLibUSBVersion: ::std::os::raw::c_char;
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_LIB_GetLibUSBVersion(cLibUSBVersion.as_ptr())
    }) {
        Ok(_) => Ok(to_string(cLibUSBVersion)),
        Err(e) => Err(e),
    }
}
pub fn LIB_IsRunningOnWine() -> Result<bool> {

    let mut pbIsRunningOnWine: ::std::os::raw::c_uchar;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_LIB_IsRunningOnWine(
            pbIsRunningOnWine,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}

struct USBDevice {
    ProductModel: String,
    SerialNumber: String,
};
pub fn USB_OpenDevice(dev_idx: u32) -> Result<USBDevice> {

    let mut cProductModel: ::std::os::raw::c_char;
    let mut cSerialNumber: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_USB_OpenDevice(
            iDevIdx,
            cProductModel,
            cSerialNumber,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn USB_OpenGetSerNumAndClose(dev_idx: u32) -> Result<USBDevice> {

    let mut cProductModel: ::std::os::raw::c_char;
    let mut cSerialNumber: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_USB_OpenGetSerNumAndClose(
            iDevIdx,
            cProductModel,
            cSerialNumber,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn USB_GetStrDescriptor(dev_idx: u32) -> Result<String> {

    let mut cDescriptor: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_USB_GetStrDescriptor(
            iDevIdx,
            cDescriptor,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn USB_GetStrDescrByIdx(dev_idx: u32, descr_idx: u32) -> Result<String> {

    let mut cDescriptor: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_USB_GetStrDescrByIdx(
            iDevIdx,
            iDescrIdx,
            cDescriptor,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn USB_IsOpenDevice(dev_idx: u32) -> Result<bool> {

    let mut pbIsOpenDevice: ::std::os::raw::c_uchar;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_USB_IsOpenDevice(
            iDevIdx,
            pbIsOpenDevice,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn USB_CloseDevice(dev_idx: u32) -> Result<bool> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_USB_CloseDevice(iDevIdx: ::std::os::raw::c_int)
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn FWR_DecodeErrPhaseName(err_phase: i32) -> Result<String> {

    let mut cErrorPhase: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_FWR_DecodeErrPhaseName(
            iErrPhase,
            cErrorPhase,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn FWR_GetVersion(dev_idx: u32) -> Result<String> {

    let mut cFWVersion: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_FWR_GetVersion(
            iDevIdx,
            cFWVersion,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}

struct FwError {
    err_code: i32,
    phase: i32,
    location: i32,
    slot: i32,
    condition: String,
}

pub fn FWR_GetLastError(dev_idx: u32) -> Result<FwError> {

    let mut piErrCode: ::std::os::raw::c_int;
    let mut piPhase: ::std::os::raw::c_int;
    let mut piLocation: ::std::os::raw::c_int;
    let mut piSlot: ::std::os::raw::c_int;
    let mut cCondition: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_FWR_GetLastError(
            iDevIdx,
            piErrCode,
            piPhase,
            piLocation,
            piSlot,
            cCondition,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn FWR_GetWorkingMode(dev_idx: u32) -> Result<i32> {

    let mut piMode: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_FWR_GetWorkingMode(
            iDevIdx,
            piMode,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn FWR_SetWorkingMode(dev_idx: i32) -> Result<i32> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_FWR_SetWorkingMode(
            iDevIdx,
            iMode,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn FWR_RollBackToPermanentValues(dev_idx: u32) -> Result<()> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_FWR_RollBackToPermanentValues(
            iDevIdx,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn FWR_StoreAsPermanentValues(dev_idx: u32) -> Result<()> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_FWR_StoreAsPermanentValues(
            iDevIdx,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn FWR_GetModuleMap(dev_idx: u32) -> Result<()> {

    let mut piModuleCount: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_FWR_GetModuleMap(
            iDevIdx,
            iPerformRestart,
            piModuleCount,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}

struct ModuleInfo {
    slot_id: i32,
    is_primary: bool,
    is_back_plane: bool,
    has_UTC: bool,
};
pub fn FWR_GetModuleInfoByMapIdx(dev_idx: u32, map_idx: i32) -> Result<ModuleInfo> {

    let mut piSlotId: ::std::os::raw::c_int;
    let mut pbIsPrimary: ::std::os::raw::c_uchar;
    let mut pbIsBackPlane: ::std::os::raw::c_uchar;
    let mut pbHasUptimeCounter: ::std::os::raw::c_uchar;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_FWR_GetModuleInfoByMapIdx(
            iDevIdx,
            iMapIdx,
            piSlotId,
            pbIsPrimary,
            pbIsBackPlane,
            pbHasUptimeCounter,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}

struct UptimeInfo {
    main_pwr_up: u32,
    active_pwr_up: u32,
    scaled_pwr_up: u32,
}
pub fn FWR_GetUptimeInfoByMapIdx(dev_idx: u32, map_idx: i32) -> Result<UptimeInfo> {

    let mut pulMainPowerUp: ::std::os::raw::c_ulong;
    let mut pulActivePowerUp: ::std::os::raw::c_ulong;
    let mut pulScaledPowerUp: ::std::os::raw::c_ulong;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_FWR_GetUptimeInfoByMapIdx(
            iDevIdx,
            iMapIdx,
            pulMainPowerUp,
            pulActivePowerUp,
            pulScaledPowerUp,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}

struct FwrRequestSupport {
    preamble: String,
    calling_sw: String,
    options: u32,
    buffer: i32,
}
pub fn FWR_CreateSupportRequestText(dev_idx: i32, fwr_req: FwrRequestSupport) -> Result<String> {

    let mut cPreamble: ::std::os::raw::c_char;
    let mut cCallingSW: ::std::os::raw::c_char;
    let mut iOptions:::std::os::raw::c_int;
    let mut iBufferLen:::std::os::raw::c_int;
    let mut cBuffer: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_FWR_CreateSupportRequestText(
            iDevIdx,
            cPreamble,
            cCallingSW,
            iOptions,
            iBufferLen,
            cBuffer,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}

// NOTE: Call this at the end of program or end of use of the laser to dealloc memory for the
// ModuleMap
pub fn FWR_FreeModuleMap(dev_idx: i32) -> Result<()> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_FWR_FreeModuleMap(iDevIdx: ::std::os::raw::c_int)
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}

struct PrimaDevInfo{
    device_i_d: String,
    device_type: String,
    fw_version: String,
    wl_count: i32,
}
pub fn PRI_GetDeviceInfo(dev_idx: i32, slot_id: i32) -> Result<PrimaDevInfo> {

    let mut pcDeviceID: ::std::os::raw::c_char;
    let mut pcDeviceType: ::std::os::raw::c_char;
    let mut pcFW_Version: ::std::os::raw::c_char;
    let mut piWL_Count: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_GetDeviceInfo(
            iDevIdx,
            iSlotId,
            pcDeviceID,
            pcDeviceType,
            pcFW_Version,
            piWL_Count,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
struct PrimaModeInfo {
    oper_mode_idx: i32,
    oper_mode: String,
}
pub fn PRI_DecodeOperationMode(dev_idx: i32, slot_id: i32) -> Result<PrimaModeInfo> {

    let mut pcOperMode: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_DecodeOperationMode(
            iDevIdx,
            iSlotId,
            iOperModeIdx,
            pcOperMode,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetOperationMode(dev_idx: i32, slot_id: i32) -> Result<i32> {

    let mut piOperModeIdx: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_GetOperationMode(
            iDevIdx,
            iSlotId,
            piOperModeIdx,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetOperationMode(dev_idx: i32, slot_id: i32, oper_mode_idx: i32) -> Result<()> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_SetOperationMode(
            iDevIdx,
            iSlotId,
            iOperModeIdx,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}

struct TriggerInfo {
    trg_src_idx: i32,
    trg_src: String,
    frequency_enabled: bool,
    trig_level_enabled: bool,
}
pub fn PRI_DecodeTriggerSource(dev_idx: i32, slot_id: i32, trg_src_idx: i32) -> Result<TriggerInfo> {

    let mut pcTrgSrc: ::std::os::raw::c_char;
    let mut pbFrequencyEnabled: ::std::os::raw::c_uchar;
    let mut pbTrigLevelEnabled: ::std::os::raw::c_uchar;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_DecodeTriggerSource(
            iDevIdx,
            iSlotId,
            iTrgSrcIdx,
            pcTrgSrc,
            pbFrequencyEnabled,
            pbTrigLevelEnabled,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetTriggerSource(dev_idx: i32, slot_id: i32) -> Result<i32> {

    let mut piTrgSrcIdx: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_GetTriggerSource(
            iDevIdx,
            iSlotId,
            piTrgSrcIdx,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetTriggerSource(dev_idx: i32, slot_id: i32, trig_src_idx: i32) -> Result<()> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_SetTriggerSource(
            iDevIdx,
            iSlotId,
            iTrgSrcIdx,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}

struct TriggerLevelInfo {
    trg_min_lvl: i32,
    trg_max_lvl: i32,
    trg_lvl_res: i32,
}

pub fn PRI_GetTriggerLevelLimits(dev_idx: i32, slot_id: i32) -> Result<TriggerLevelInfo> {

    let mut piTrg_MinLvl: ::std::os::raw::c_int;
    let mut piTrg_MaxLvl: ::std::os::raw::c_int;
    let mut piTrg_LvlRes: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_GetTriggerLevelLimits(
            iDevIdx,
            iSlotId,
            piTrg_MinLvl,
            piTrg_MaxLvl,
            piTrg_LvlRes,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetTriggerLevel(dev_idx: i32, slot_id: i32) -> Result<i32> {

    let mut piTrgLevel: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_GetTriggerLevel(
            iDevIdx,
            iSlotId,
            piTrgLevel,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetTriggerLevel(dev_idx: i32, slot_id: i32, trg_level: i32) -> Result<()> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_SetTriggerLevel(
            iDevIdx,
            iSlotId,
            iTrgLevel,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetFrequencyLimits(dev_idx: i32, slot_id: i32) -> Result<(i32, i32)> {

    let mut piMinFreq: ::std::os::raw::c_int;
    let mut piMaxFreq: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_GetFrequencyLimits(
            iDevIdx,
            iSlotId,
            piMinFreq,
            piMaxFreq,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetFrequency(dev_idx: i32, slot_id: i32) -> Result<i32> {

    let mut piFrequency: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_GetFrequency(
            iDevIdx,
            iSlotId,
            piFrequency,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetFrequency(dev_idx: i32, slot_id: i32, frequency: i32) -> Result<()> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_SetFrequency(
            iDevIdx,
            iSlotId,
            iFrequency,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}

struct PriGatingInfo {
    min_on_time: i32,
    max_on_time: i32,
    min_off_time_factor: i32,
    max_off_time_factor: i32,
}
pub fn PRI_GetGatingLimits(dev_idx: i32, slot_id: i32) -> Result<PriGationInfo> {

    let mut piMinOnTime: ::std::os::raw::c_int;
    let mut piMaxOnTime: ::std::os::raw::c_int;
    let mut pbMinOffTimefactor: ::std::os::raw::c_int;
    let mut pbMaxOffTimefactor: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_GetGatingLimits(
            iDevIdx,
            iSlotId,
            piMinOnTime,
            piMaxOnTime,
            pbMinOffTimefactor,
            pbMaxOffTimefactor,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetGatingData(dev_idx: i32, slot_id: i32) -> Result<(i32, i32)> {

    let mut piOnTime: ::std::os::raw::c_int;
    let mut piOffTimeFact: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_GetGatingData(
            iDevIdx,
            iSlotId,
            piOnTime,
            piOffTimeFact,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetGatingData(dev_idx: i32, slot_id: i32, on_time: i32, off_time_factor: i32) -> Result<()> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_SetGatingData(
            iDevIdx,
            iSlotId,
            iOnTime,
            iOffTimeFact,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetGatingEnabled(dev_idx: i32, slot_id: i32) -> Result<bool> {

    let mut pbGatingEnabled: ::std::os::raw::c_uchar;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_GetGatingEnabled(
            iDevIdx,
            iSlotId,
            pbGatingEnabled,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetGatingEnabled(dev_idx: i32, slot_id: i32, gating_enabled: bool) -> Result<()> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_SetGatingEnabled(
            iDevIdx,
            iSlotId,
            bGatingEnabled,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetGateHighImpedance(dev_idx: i32, slot_id: i32) -> Result<bool> {

    let mut pbHighImpedance: ::std::os::raw::c_uchar;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_GetGateHighImpedance(
            iDevIdx,
            iSlotId,
            pbHighImpedance,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetGateHighImpedance(dev_idx: i32, slot_id: i32, high_impedance: bool) -> Result<()> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_SetGateHighImpedance(
            iDevIdx,
            iSlotId,
            bHighImpedance,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_DecodeWavelength(dev_idx: i32, slot_id: i32, wl_idx: i32) -> Result<i32> {

    let mut piWL: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_DecodeWavelength(
            iDevIdx,
            iSlotId,
            iWLIdx,
            piWL,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetWavelengthIdx(dev_idx: i32, slot_id: i32) -> Result<i32> {

    let mut piWLIdx: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_GetWavelengthIdx(
            iDevIdx,
            iSlotId,
            piWLIdx,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetWavelengthIdx(dev_idx: i32, slot_id: i32, wl_idx: i32) -> Result<()> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_SetWavelengthIdx(
            iDevIdx,
            iSlotId,
            iWLIdx,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetIntensity(dev_idx: i32, slot_id: i32, wl_idx: i32) -> Result<u16> {

    let mut pwIntensity: ::std::os::raw::c_ushort;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_GetIntensity(
            iDevIdx,
            iSlotId,
            iWLIdx,
            pwIntensity,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetIntensity(dev_idx: i32, slot_id: i32, wl_idx: i32, intensity: u16) -> Result<u16> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2_PRI_SetIntensity(
            iDevIdx,
            iSlotId,
            iWLIdx,
            wIntensity,
        )
    }) {
        Ok(_) => {
        },
        Err(e) => Err(e),
    }
}
