tonic::include_proto!("sepia2.rpc");

// This is only used by server, so don't throw a warning when lib is compiled for client
#[allow(dead_code)]
pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("sepia2_rpc_descriptor");

// ----------- Glue logic -------------

// FIXME: These can be handled much easier by using serde
macro_rules! convert_from {
    ($ty:ident, $($field:ident),*) => {
        impl From<sepia2::types::$ty> for $ty{
            fn from(item: sepia2::types::$ty) -> $ty {
                Self {
                    $( $field: item.$field, )*
                }
            }
        }
    };
}
macro_rules! convert_into {
    ($ty:ident, $($field:ident),*) => {
        impl Into<sepia2::types::$ty> for $ty{
            fn into(self: $ty ) -> sepia2::types::$ty {
                sepia2::types::$ty {
                    $( $field: self.$field, )*
                }
            }
        }
    };
}

macro_rules! convert_struct {
    ($ty:ident, $($field:ident),*) => {
        convert_from!($ty, $($field),*);
        convert_into!($ty, $($field),*);
    };
}


impl From<bool> for Bool {
    fn from(item: bool) -> Self {
        Self { value: item }
    }
}

// FIXME: Better use distinct names for common types to avoid clashes in Rust & Julia
impl From<i32> for Int32{
    fn from(item: i32) -> Self {
        Self { value: item }
    }
}

impl From<u16> for Uint32{
    fn from(item: u16) -> Self {
        Self { value: item as u32 }
    }
}

// NOTE: Here String=sepia2_rpc::String
impl From<std::string::String> for String {
    fn from(item: std::string::String) -> Self {
        Self { value: item }
    }
}

impl From<(i32, i32)> for GatingData {
    fn from(item: (i32, i32)) -> Self {
        Self { on_time: item.0, off_time_factor: item.1}
    }
}

impl From<sepia2::types::OperationMode> for OperationModeResponse {
    fn from(item: sepia2::types::OperationMode) -> Self {
        // TODO: Check if direct conversion works
        Self { value: item as i32 }
    }
}

convert_struct!(UsbDevice,  product_model, serial_number);
convert_struct!(FwrError,  err_code, phase, location, slot, condition);
convert_struct!(FwrRequestSupport, preamble, calling_sw, options, buffer);
convert_struct!(ModuleInfo, slot_id, is_primary, is_back_plane, has_utc);
convert_struct!(UptimeInfo, main_pwr_up, active_pwr_up, scaled_pwr_up);
convert_struct!(PrimaDevInfo, device_id, device_type, fw_version, wl_count);
convert_struct!(PrimaModeInfo, oper_mode_idx, oper_mode);
convert_struct!(TriggerInfo, trg_src_idx, trg_src, frequency_enabled, trig_level_enabled);
convert_struct!(TriggerLevelInfo, trg_min_lvl, trg_max_lvl, trg_lvl_res);
convert_struct!(PrimaGatingInfo, min_on_time, max_on_time, min_off_time_factor, max_off_time_factor);
