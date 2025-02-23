use jlrs::{
    data::{
        managed::value::{
            typed::{TypedValue, TypedValueRet},
            ValueRet,
        },
        types::foreign_type::ForeignType,
    },
    memory::PTls,
    memory::gc::{mark_queue_obj, write_barrier},
    prelude::*,
    weak_handle,
};

macro_rules! typeof_member {
    (sepia2::types::UsbDevice, product_model) => { JuliaString };
    (sepia2::types::UsbDevice, serial_number) => { JuliaString };
    (sepia2::types::FwrError, err_code) => { i32 };
    (sepia2::types::FwrError, phase) => { i32 };
    (sepia2::types::FwrError, location) => { i32 };
    (sepia2::types::FwrError, slot) => { i32 };
    (sepia2::types::FwrError, condition) => { JuliaString };
    (sepia2::types::ModuleInfo, slot_id) => { i32 };
    (sepia2::types::ModuleInfo, is_primary) => { Bool }; // Convert bool to jlrs::Bool
    (sepia2::types::ModuleInfo, is_back_plane) => { Bool }; // Convert bool to jlrs::Bool
    (sepia2::types::ModuleInfo, has_utc) => { Bool }; // Convert bool to jlrs::Bool
    (sepia2::types::UptimeInfo, main_pwr_up) => { u32 };
    (sepia2::types::UptimeInfo, active_pwr_up) => { u32 };
    (sepia2::types::UptimeInfo, scaled_pwr_up) => { u32 };
    (sepia2::types::FwrRequestSupport, preamble) => { JuliaString };
    (sepia2::types::FwrRequestSupport, calling_sw) => { JuliaString };
    (sepia2::types::FwrRequestSupport, options) => { i32 };
    (sepia2::types::FwrRequestSupport, buffer) => { i32 };
    (sepia2::types::PrimaDevInfo, device_id) => { JuliaString };
    (sepia2::types::PrimaDevInfo, device_type) => { JuliaString };
    (sepia2::types::PrimaDevInfo, fw_version) => { JuliaString };
    (sepia2::types::PrimaDevInfo, wl_count) => { i32 };
    (sepia2::types::PrimaModeInfo, oper_mode_idx) => { i32 };
    (sepia2::types::PrimaModeInfo, oper_mode) => { JuliaString };
    (sepia2::types::TriggerInfo, trg_src_idx) => { i32 };
    (sepia2::types::TriggerInfo, trg_src) => { JuliaString };
    (sepia2::types::TriggerInfo, frequency_enabled) => { Bool }; // Convert bool to jlrs::Bool
    (sepia2::types::TriggerInfo, trig_level_enabled) => { Bool }; // Convert bool to jlrs::Bool
    (sepia2::types::TriggerLevelInfo, trg_min_lvl) => { i32 };
    (sepia2::types::TriggerLevelInfo, trg_max_lvl) => { i32 };
    (sepia2::types::TriggerLevelInfo, trg_lvl_res) => { i32 };
    (sepia2::types::PrimaGatingInfo, min_on_time) => { i32 };
    (sepia2::types::PrimaGatingInfo, max_on_time) => { i32 };
    (sepia2::types::PrimaGatingInfo, min_off_time_factor) => { i32 };
    (sepia2::types::PrimaGatingInfo, max_off_time_factor) => { i32 };
}

macro_rules! julia_type_equivalent {
    ($type:ident, $($field:ident),*) => {
        struct $type {
            // TODO: Fill in all fields from sepia2::types::$sepia2_type
            // apart from bool, which becomes Bool
            $( $field: TypedValueRet<typeof_member!(sepia2::types::$type, $field)>, )*
        }

        unsafe impl Send for $type {}
        unsafe impl Sync for $type {}

        unsafe impl ForeignType for $type {
            fn mark(ptls: PTls, data: &Self) -> usize {
                // Safety: We mark all referenced managed data.
                unsafe {
                    let mut n_marked = 0;
                    $( n_marked += mark_queue_obj(ptls, data.$field) as usize; )*
                    n_marked
                }
            }
        }


        impl From<sepia2::types::$type> for $type {
            fn from(item: sepia2::types::$type) -> $type {
                Self {
                    $( $field:  item.$field, )*
                }
            }
        }

    };
}

julia_type_equivalent!(UsbDevice,  product_model, serial_number);
julia_type_equivalent!(FwrError,  err_code, phase, location, slot, condition);
julia_type_equivalent!(FwrRequestSupport, preamble, calling_sw, options, buffer);
julia_type_equivalent!(ModuleInfo, slot_id, is_primary, is_back_plane, has_utc);
julia_type_equivalent!(UptimeInfo, main_pwr_up, active_pwr_up, scaled_pwr_up);
julia_type_equivalent!(PrimaDevInfo, device_id, device_type, fw_version, wl_count);
julia_type_equivalent!(PrimaModeInfo, oper_mode_idx, oper_mode);
julia_type_equivalent!(TriggerInfo, trg_src_idx, trg_src, frequency_enabled, trig_level_enabled);
julia_type_equivalent!(TriggerLevelInfo, trg_min_lvl, trg_max_lvl, trg_lvl_res);
julia_type_equivalent!(PrimaGatingInfo, min_on_time, max_on_time, min_off_time_factor, max_off_time_factor);
