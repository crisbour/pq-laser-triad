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
    (UsbDevice, product_model)             => { String };
    (UsbDevice, serial_number)             => { String };
    (FwrError, err_code)                   => { i32 };
    (FwrError, phase)                      => { i32 };
    (FwrError, location)                   => { i32 };
    (FwrError, slot)                       => { i32 };
    (FwrError, condition)                  => { String };
    (ModuleInfo, slot_id)                  => { i32 };
    (ModuleInfo, is_primary)               => { bool }; // Convert bool to jlrs::Bool
    (ModuleInfo, is_back_plane)            => { bool }; // Convert bool to jlrs::Bool
    (ModuleInfo, has_utc)                  => { bool }; // Convert bool to jlrs::Bool
    (UptimeInfo, main_pwr_up)              => { u32 };
    (UptimeInfo, active_pwr_up)            => { u32 };
    (UptimeInfo, scaled_pwr_up)            => { u32 };
    (FwrRequestSupport, preamble)          => { String };
    (FwrRequestSupport, calling_sw)        => { String };
    (FwrRequestSupport, options)           => { i32 };
    (FwrRequestSupport, buffer)            => { i32 };
    (PrimaDevInfo, device_id)              => { String };
    (PrimaDevInfo, device_type)            => { String };
    (PrimaDevInfo, fw_version)             => { String };
    (PrimaDevInfo, wl_count)               => { i32 };
    (PrimaModeInfo, oper_mode_idx)         => { i32 };
    (PrimaModeInfo, oper_mode)             => { String };
    (TriggerInfo, trg_src_idx)             => { i32 };
    (TriggerInfo, trg_src)                 => { String };
    (TriggerInfo, frequency_enabled)       => { bool }; // Convert bool to jlrs::Bool
    (TriggerInfo, trig_level_enabled)      => { bool }; // Convert bool to jlrs::Bool
    (TriggerLevelInfo, trg_min_lvl)        => { i32 };
    (TriggerLevelInfo, trg_max_lvl)        => { i32 };
    (TriggerLevelInfo, trg_lvl_res)        => { i32 };
    (PrimaGatingInfo, min_on_time)         => { i32 };
    (PrimaGatingInfo, max_on_time)         => { i32 };
    (PrimaGatingInfo, min_off_time_factor) => { i32 };
    (PrimaGatingInfo, max_off_time_factor) => { i32 };
}


macro_rules! julia_type_equivalent {
    ($type:ident, $($field:ident),*) => {
        #[repr(C)]
        #[derive(Clone, Debug, IsBits, ConstructType)]
        #[jlrs(
            julia_type = stringify!(Main.Sepia2Client.$type),
        )]
        pub struct $type {
            // TODO: Fill in all fields from sepia2::types::$sepia2_type
            // apart from bool, which becomes Bool
            $( $field: typeof_member!($type, $field), )*
        }

        unsafe impl Send for $type {}
        unsafe impl Sync for $type {}

        unsafe impl OpaqueType for OpaqueInt {}

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
