use derive_more::TryFrom;

#[derive(Debug, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
pub enum OperationMode {
    Off = 0,
    NarrowPulse = 1,
    BroadPulse = 2,
    CW = 3,
}

#[derive(Debug)]
pub struct UsbDevice {
    pub product_model: String,
    pub serial_number: String,
}

#[derive(Debug)]
pub struct FwrError {
    pub err_code: i32,
    pub phase: i32,
    pub location: i32,
    pub slot: i32,
    pub condition: String,
}

#[derive(Debug)]
pub struct ModuleInfo {
    pub slot_id: i32,
    pub is_primary: bool,
    pub is_back_plane: bool,
    pub has_utc: bool,
}

#[derive(Debug)]
pub struct UptimeInfo {
    pub main_pwr_up: u32,
    pub active_pwr_up: u32,
    pub scaled_pwr_up: u32,
}

#[derive(Debug)]
pub struct FwrRequestSupport {
    pub preamble: String,
    pub calling_sw: String,
    pub options: i32,
    pub buffer: i32,
}

#[derive(Debug)]
pub struct PrimaDevInfo {
    pub device_id: String,
    pub device_type: String,
    pub fw_version: String,
    pub wl_count: i32,
}

#[derive(Debug)]
pub struct PrimaModeInfo {
    pub oper_mode_idx: i32,
    pub oper_mode: String,
}

#[derive(Debug)]
pub struct TriggerInfo {
    pub trg_src_idx: i32,
    pub trg_src: String,
    pub frequency_enabled: bool,
    pub trig_level_enabled: bool,
}

#[derive(Debug)]
pub struct TriggerLevelInfo {
    pub trg_min_lvl: i32,
    pub trg_max_lvl: i32,
    pub trg_lvl_res: i32,
}

#[derive(Debug)]
pub struct PrimaGatingInfo {
    pub min_on_time: i32,
    pub max_on_time: i32,
    pub min_off_time_factor: i32,
    pub max_off_time_factor: i32,
}
