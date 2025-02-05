#![allow(non_snake_case)]

use crate::bindings::api::*;

type Result<T> = std::result::Result<T, Sepia2Error>;

struct Sepia2State {
    sepia2_lib: Sepia2_Lib,
}

pub fn LIB_DecodeError(err_code: i32) -> Result<String> {
    let mut c_err_string: ::std::os::raw::c_char;
    match Sepia2Error::from_raw(
        unsafe { SEPIA2.SEPIA2_LIB_DecodeError(err_code, c_err_string.as_ptr()) }
    ) {
        Ok(_) => Ok(to_string(c_err_string)),
        Err(e) => Err(e),
    }
}
pub fn LIB_GetVersion() -> Result<String> {
    let mut c_lib_version: ::std::os::raw::c_char;
    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_LIB_GetVersion(c_lib_version.as_ptr())
    }) {
        Ok(_) => Ok(to_string(c_lib_version)),
        Err(e) => Err(e),
    }
}
pub fn LIB_GetLibUSBVersion() -> Result<String> {
    let mut cLibUSBVersion: ::std::os::raw::c_char;
    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_LIB_GetLibUSBVersion(cLibUSBVersion.as_ptr())
    }) {
        Ok(_) => Ok(to_string(cLibUSBVersion)),
        Err(e) => Err(e),
    }
}
pub fn LIB_IsRunningOnWine() -> Result<bool> {

    let mut pbIsRunningOnWine: u8,

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_LIB_IsRunningOnWine(
            pbIsRunningOnWine,
        )
    }) {
        Ok(_) => {
            Ok(pbIsRunningOnWine != 0)
        },
        Err(e) => Err(e),
    }
}

struct USBDevice {
    ProductModel: String,
    SerialNumber: String,
}
pub fn USB_OpenDevice(dev_idx: u32) -> Result<USBDevice> {

    let mut cProductModel: ::std::os::raw::c_char;
    let mut cSerialNumber: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_USB_OpenDevice(
            dev_idx, // iDevIdx,
            cProductModel.as_mut_ptr(),
            cSerialNumber.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok( USBDevice {
                ProductModel: to_string(cProductModel),
                SerialNumber: to_string(cSerialNumber),
            })
        },
        Err(e) => Err(e),
    }
}
pub fn USB_OpenGetSerNumAndClose(dev_idx: u32) -> Result<USBDevice> {

    let mut cProductModel: ::std::os::raw::c_char;
    let mut cSerialNumber: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_USB_OpenGetSerNumAndClose(
            dev_idx, // iDevIdx,
            cProductModel.as_mut_ptr(),
            cSerialNumber.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok( USBDevice {
                ProductModel: to_string(cProductModel),
                SerialNumber: to_string(cSerialNumber),
            })
        },
        Err(e) => Err(e),
    }
}
pub fn USB_GetStrDescriptor(dev_idx: u32) -> Result<String> {

    let mut cDescriptor: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_USB_GetStrDescriptor(
            dev_idx, // iDevIdx,
            cDescriptor.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(to_string(cDescriptor))
        },
        Err(e) => Err(e),
    }
}
pub fn USB_GetStrDescrByIdx(dev_idx: u32, descr_idx: u32) -> Result<String> {

    let mut cDescriptor: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_USB_GetStrDescrByIdx(
            dev_idx, // iDevIdx,
            descr_idx, // iDescrIdx,
            cDescriptor,
        )
    }) {
        Ok(_) => {
            Ok(to_string(cDescriptor))
        },
        Err(e) => Err(e),
    }
}
pub fn USB_IsOpenDevice(dev_idx: u32) -> Result<bool> {

    let mut pbIsOpenDevice: u8;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_USB_IsOpenDevice(
            dev_idx, // iDevIdx,
            pbIsOpenDevice.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(pbIsOpenDevice)
        },
        Err(e) => Err(e),
    }
}
pub fn USB_CloseDevice(dev_idx: u32) -> Result<()> {
    unsafe {
        SEPIA2.SEPIA2_USB_CloseDevice(dev_idx)
    }
}
pub fn FWR_DecodeErrPhaseName(err_phase: i32) -> Result<String> {

    let mut cErrorPhase: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_FWR_DecodeErrPhaseName(
            err_phase, // iErrPhase,
            cErrorPhase,
        )
    }) {
        Ok(_) => {
            Ok(to_string(cErrorPhase))
        },
        Err(e) => Err(e),
    }
}
pub fn FWR_GetVersion(dev_idx: u32) -> Result<String> {

    let mut cFWVersion: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_FWR_GetVersion(
            dev_idx, // iDevIdx,
            cFWVersion.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(to_string(cFWVersion))
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
        SEPIA2.SEPIA2_FWR_GetLastError(
            dev_idx, // iDevIdx,
            piErrCode.as_mut_ptr(),
            piPhase.as_mut_ptr(),
            piLocation.as_mut_ptr(),
            piSlot.as_mut_ptr(),
            cCondition.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok( FwError {
                err_code: piErrCode,
                phase: piPhase,
                location: piLocation,
                slot: piSlot,
                condition: to_string(cCondition),
            })
        },
        Err(e) => Err(e),
    }
}
pub fn FWR_GetWorkingMode(dev_idx: u32) -> Result<i32> {

    let mut piMode: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_FWR_GetWorkingMode(
            dev_idx, // iDevIdx,
            piMode.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(piMode)
        },
        Err(e) => Err(e),
    }
}
pub fn FWR_SetWorkingMode(dev_idx: i32, mode: i32) -> Result<()> {
    unsafe {
        SEPIA2.SEPIA2_FWR_SetWorkingMode(
            dev_idx, // iDevIdx,
            mode, // iMode,
        )
    }
}
pub fn FWR_RollBackToPermanentValues(dev_idx: u32) -> Result<()> {
    unsafe {
        SEPIA2.SEPIA2_FWR_RollBackToPermanentValues(
            dev_idx, // iDevIdx,
        )
    }
}
pub fn FWR_StoreAsPermanentValues(dev_idx: u32) -> Result<()> {
    unsafe {
        SEPIA2.SEPIA2_FWR_StoreAsPermanentValues(
            dev_idx, // iDevIdx,
        )
    }
}
pub fn FWR_GetModuleMap(dev_idx: u32, perform_restart: bool) -> Result<i32> {

    let mut piModuleCount: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_FWR_GetModuleMap(
            dev_idx, // iDevIdx,
            perform_restart, // iPerformRestart,
            piModuleCount.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(piModuleCount)
        },
        Err(e) => Err(e),
    }
}

struct ModuleInfo {
    slot_id: i32,
    is_primary: bool,
    is_back_plane: bool,
    has_UTC: bool,
}
pub fn FWR_GetModuleInfoByMapIdx(dev_idx: u32, map_idx: i32) -> Result<ModuleInfo> {

    let mut piSlotId: ::std::os::raw::c_int;
    let mut pbIsPrimary: ::std::os::raw::c_uchar;
    let mut pbIsBackPlane: ::std::os::raw::c_uchar;
    let mut pbHasUptimeCounter: ::std::os::raw::c_uchar;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_FWR_GetModuleInfoByMapIdx(
            dev_idx, // iDevIdx,
            map_idx, // iMapIdx,
            piSlotId.as_mut_ptr(),
            pbIsPrimary.as_mut_ptr(),
            pbIsBackPlane.as_mut_ptr(),
            pbHasUptimeCounter.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok( ModuleInfo {
                slot_id: piSlotId,
                is_primary: pbIsPrimary,
                is_back_plane: pbIsBackPlane,
                has_UTC: pbHasUptimeCounter,
            })
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
        SEPIA2.SEPIA2_FWR_GetUptimeInfoByMapIdx(
            dev_idx, // iDevIdx,
            map_idx, // iMapIdx,
            pulMainPowerUp.as_mut_ptr(),
            pulActivePowerUp.as_mut_ptr(),
            pulScaledPowerUp.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok( UptimeInfo {
                main_pwr_up: pulMainPowerUp,
                active_pwr_up: pulActivePowerUp,
                scaled_pwr_up: pulScaledPowerUp,
            })
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
        SEPIA2.SEPIA2_FWR_CreateSupportRequestText(
            dev_idx, // iDevIdx,
            fwr_req.preamble.as_ptr(), // cPreamble,
            fwr_req.preamble.as_ptr(), // cCallingSW,
            fwr_req.preamble.as_ptr(), // iOptions,
            fwr_req.preamble.as_ptr(), // iBufferLen,
            cBuffer.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(cBuffer)
        },
        Err(e) => Err(e),
    }
}

// NOTE: Call this at the end of program or end of use of the laser to dealloc memory for the
// ModuleMap
pub fn FWR_FreeModuleMap(dev_idx: i32) -> Result<()> {
    unsafe {
        SEPIA2.SEPIA2_FWR_FreeModuleMap(dev_idx)
    }
}

struct PrimaDevInfo{
    device_id: String,
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
        SEPIA2.SEPIA2_PRI_GetDeviceInfo(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            pcDeviceID.as_mut_ptr(),
            pcDeviceType.as_mut_ptr(),
            pcFW_Version.as_mut_ptr(),
            piWL_Count.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok( PrimaDevInfo {
                device_id: to_string(pcDeviceID),
                device_type: to_string(pcDeviceType),
                fw_version: to_string(pcFW_Version),
                wl_count: piWL_Count,
            })
        },
        Err(e) => Err(e),
    }
}
struct PrimaModeInfo {
    oper_mode_idx: i32,
    oper_mode: String,
}
pub fn PRI_DecodeOperationMode(dev_idx: i32, slot_id: i32, oper_mode_idx: i32) -> Result<PrimaModeInfo> {

    let mut pcOperMode: ::std::os::raw::c_char;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_PRI_DecodeOperationMode(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            oper_mode_idx, // iOperModeIdx,
            pcOperMode.as_mut_ptr,
        )
    }) {
        Ok(_) => {
            Ok(PrimaModeInfo { oper_mode_idx, oper_mode: pcOperMode })
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetOperationMode(dev_idx: i32, slot_id: i32) -> Result<i32> {

    let mut piOperModeIdx: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_PRI_GetOperationMode(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            piOperModeIdx.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(piOperModeIdx)
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetOperationMode(dev_idx: i32, slot_id: i32, oper_mode_idx: i32) -> Result<()> {
    unsafe {
        SEPIA2.SEPIA2_PRI_SetOperationMode(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            oper_mode_idx, // iOperModeIdx,
        )
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
        SEPIA2.SEPIA2_PRI_DecodeTriggerSource(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            trg_src_idx, // iTrgSrcIdx,
            pcTrgSrc.as_mut_ptr(),
            pbFrequencyEnabled.as_mut_ptr(),
            pbTrigLevelEnabled.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok( TriggerInfo {
                trg_src_idx,
                trg_src: to_string(pcTrgSrc),
                frequency_enabled: pbFrequencyEnabled != 0,
                trig_level_enabled: pbTrigLevelEnabled != 0,
            })
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetTriggerSource(dev_idx: i32, slot_id: i32) -> Result<i32> {

    let mut piTrgSrcIdx: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_PRI_GetTriggerSource(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            piTrgSrcIdx.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(piTrgSrcIdx)
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetTriggerSource(dev_idx: i32, slot_id: i32, trig_src_idx: i32) -> Result<()> {
    unsafe {
        SEPIA2.SEPIA2_PRI_SetTriggerSource(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            trg_src_idx, // iTrgSrcIdx,
        )
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
        SEPIA2.SEPIA2_PRI_GetTriggerLevelLimits(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            piTrg_MinLvl.as_mut_ptr(),
            piTrg_MaxLvl.as_mut_ptr(),
            piTrg_LvlRes.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok( TriggerLevelInfo{
                    trg_min_lvl: piTrg_MinLvl,
                    trg_max_lvl: piTrg_MaxLvl,
                    trg_lvl_res: piTrg_LvlRes,
            })
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetTriggerLevel(dev_idx: i32, slot_id: i32) -> Result<i32> {

    let mut piTrgLevel: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_PRI_GetTriggerLevel(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            piTrgLevel.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(piTrgLevel)
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetTriggerLevel(dev_idx: i32, slot_id: i32, trg_level: i32) -> Result<()> {
    unsafe {
        SEPIA2.SEPIA2_PRI_SetTriggerLevel(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            trg_level, // iTrgLevel,
        )
    }
}
pub fn PRI_GetFrequencyLimits(dev_idx: i32, slot_id: i32) -> Result<(i32, i32)> {

    let mut piMinFreq: ::std::os::raw::c_int;
    let mut piMaxFreq: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_PRI_GetFrequencyLimits(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            piMinFreq.as_mut_ptr(),
            piMaxFreq.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok((piMinFreq, piMaxFreq))
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetFrequency(dev_idx: i32, slot_id: i32) -> Result<i32> {

    let mut piFrequency: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_PRI_GetFrequency(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            piFrequency.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(piFrequency)
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetFrequency(dev_idx: i32, slot_id: i32, frequency: i32) -> Result<()> {
    unsafe {
        SEPIA2.SEPIA2_PRI_SetFrequency(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            frequency, // iFrequency,
        )
    }
}

struct PrimaGatingInfo {
    min_on_time: i32,
    max_on_time: i32,
    min_off_time_factor: i32,
    max_off_time_factor: i32,
}
pub fn PRI_GetGatingLimits(dev_idx: i32, slot_id: i32) -> Result<PrimaGationInfo> {

    let mut piMinOnTime: ::std::os::raw::c_int;
    let mut piMaxOnTime: ::std::os::raw::c_int;
    let mut pbMinOffTimefactor: ::std::os::raw::c_int;
    let mut pbMaxOffTimefactor: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_PRI_GetGatingLimits(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            piMinOnTime.as_mut_ptr(),
            piMaxOnTime.as_mut_ptr(),
            pbMinOffTimefactor.as_mut_ptr(),
            pbMaxOffTimefactor.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok( PrimaGatingInfo {
                min_on_time: piMinOnTime,
                max_on_time: piMaxOnTime,
                min_off_time_factor: pbMinOffTimefactor,
                max_off_time_factor: pbMaxOffTimefactor,
            })
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetGatingData(dev_idx: i32, slot_id: i32) -> Result<(i32, i32)> {

    let mut piOnTime: ::std::os::raw::c_int;
    let mut piOffTimeFact: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_PRI_GetGatingData(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            piOnTime.as_mut_ptr(),
            piOffTimeFact.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok((piOnTime, piOffTimeFact))
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetGatingData(dev_idx: i32, slot_id: i32, on_time: i32, off_time_factor: i32) -> Result<()> {
    unsafe {
        SEPIA2.SEPIA2_PRI_SetGatingData(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            on_time, // iOnTime,
            off_time_factor, // iOffTimeFact,
        )
    }
}
pub fn PRI_GetGatingEnabled(dev_idx: i32, slot_id: i32) -> Result<bool> {

    let mut pbGatingEnabled: ::std::os::raw::c_uchar;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_PRI_GetGatingEnabled(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            pbGatingEnabled.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(pbGatingEnabled != 0)
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetGatingEnabled(dev_idx: i32, slot_id: i32, gating_enabled: bool) -> Result<()> {
    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_PRI_SetGatingEnabled(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            gating_enabled, // bGatingEnabled,
        )
    }) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
pub fn PRI_GetGateHighImpedance(dev_idx: i32, slot_id: i32) -> Result<bool> {

    let mut pbHighImpedance: u8;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_PRI_GetGateHighImpedance(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            pbHighImpedance.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(pbHighImpedance != 0)
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetGateHighImpedance(dev_idx: i32, slot_id: i32, high_impedance: bool) -> Result<()> {
    unsafe {
        SEPIA2.SEPIA2_PRI_SetGateHighImpedance(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            high_impedance, // bHighImpedance,
        )
    }
}
pub fn PRI_DecodeWavelength(dev_idx: i32, slot_id: i32, wl_idx: i32) -> Result<i32> {

    let mut piWL: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_PRI_DecodeWavelength(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            wl_idx, // iWLIdx,
            piWL.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(piWL)
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_GetWavelengthIdx(dev_idx: i32, slot_id: i32) -> Result<i32> {

    let mut piWLIdx: ::std::os::raw::c_int;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_PRI_GetWavelengthIdx(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            piWLIdx.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(piWLIdx)
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetWavelengthIdx(dev_idx: i32, slot_id: i32, wl_idx: i32) -> Result<()> {
   unsafe {
        SEPIA2.SEPIA2_PRI_SetWavelengthIdx(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            wl_idx, // iWLIdx,
        )
    }
}
pub fn PRI_GetIntensity(dev_idx: i32, slot_id: i32, wl_idx: i32) -> Result<u16> {

    let mut pwIntensity: ::std::os::raw::c_ushort;

    match Sepia2Error::from_raw( unsafe {
        SEPIA2.SEPIA2_PRI_GetIntensity(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            wl_idx, // iWLIdx,
            pwIntensity.as_mut_ptr(),
        )
    }) {
        Ok(_) => {
            Ok(pwIntensity)
        },
        Err(e) => Err(e),
    }
}
pub fn PRI_SetIntensity(dev_idx: i32, slot_id: i32, wl_idx: i32, w_intensity: u16) -> Result<()> {
    unsafe {
        SEPIA2.SEPIA2_PRI_SetIntensity(
            dev_idx, // iDevIdx,
            slot_id, // iSlotId,
            wl_idx, // iWLIdx,
            w_intensity, // wIntensity,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_log::test;

    #[test]
    fn test_default_settings() {
    }
}
