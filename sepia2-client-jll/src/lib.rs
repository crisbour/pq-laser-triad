#![allow(non_snake_case)]

use jlrs::prelude::*;
use jlrs::data::managed::value::{ValueRet, typed::{TypedValue, TypedValueRet}};
use jlrs::data::managed::ccall_ref::{CCallRef, CCallRefRet};
use jlrs::data::layout::{tuple::Tuple2, bool::Bool};
use jlrs::weak_handle;
use jlrs::data::types::construct_type::ConstructType;
use jlrs::data::managed::value::typed::AsTyped;
//use jlrs::data::managed::string::TypedValueRet<JuliaString>;
use sepia2_grpc::sepia2_client::Sepia2Client;
use std::sync::mpsc;
use oneshot;
use lazy_static::lazy_static;
use std::ffi::CString;

mod types;
pub use types::*;

// ----------------------------------------------------------------
// Channel to request client service querries to gRPC (tonic)
// ----------------------------------------------------------------

#[derive(Debug)]
enum ClientReq {
    LibDecodeError{ err_code: i32},
    LibGetVersion,
    LibGetLibUSBVersion,
    LibIsRunningOnWine,
    UsbOpenDevice{dev_idx: i32},
    UsbOpenGetSerNumAndClose{dev_idx: i32},
    UsbGetStrDescriptor{dev_idx: i32},
    UsbGetStrDescrByIdx{dev_idx: i32, descr_idx: i32},
    UsbIsOpenDevice{dev_idx: i32},
    UsbCloseDevice{dev_idx: i32},
    FwrDecodeErrPhaseName{err_phase: i32},
    FwrGetVersion{dev_idx: i32},
    FwrGetLastError{dev_idx: i32},
    FwrGetWorkingMode{dev_idx: i32},
    FwrSetWorkingMode{dev_idx: i32, mode: i32},
    FwrRollBackToPermanentValues{dev_idx: i32},
    FwrStoreAsPermanentValues{dev_idx: i32},
    FwrGetModuleMap{dev_idx: i32, perform_restart: bool},
    FwrGetModuleInfoByMapIdx{dev_idx: i32, map_idx: i32},
    FwrGetUptimeInfoByMapIdx{dev_idx: i32, map_idx: i32},
    FwrCreateSupportRequestText{dev_idx: i32, fwr_req: FwrRequestSupport},
    FwrFreeModuleMap{dev_idx: i32},
    PriGetDeviceInfo{dev_idx: i32, slot_id: i32},
    PriDecodeOperationMode{dev_idx: i32, slot_id: i32, oper_mode_idx: i32},
    PriGetOperationMode{dev_idx: i32, slot_id: i32},
    PriSetOperationMode{dev_idx: i32, slot_id: i32, oper_mode_idx: i32},
    PriDecodeTriggerSource{dev_idx: i32, slot_id: i32, trg_src_idx: i32},
    PriGetTriggerSource{dev_idx: i32, slot_id: i32},
    PriSetTriggerSource{dev_idx: i32, slot_id: i32, trg_src_idx: i32},
    PriGetTriggerLevelLimits{dev_idx: i32, slot_id: i32},
    PriGetTriggerLevel{dev_idx: i32, slot_id: i32},
    PriSetTriggerLevel{dev_idx: i32, slot_id: i32, trg_lvl: i32},
    PriGetFrequencyLimits{dev_idx: i32, slot_id: i32},
    PriGetFrequency{dev_idx: i32, slot_id: i32},
    PriSetFrequency{dev_idx: i32, slot_id: i32, frequency: i32},
    PriGetGatingLimits{dev_idx: i32, slot_id: i32},
    PriGetGatingData{dev_idx: i32, slot_id: i32},
    PriSetGatingData{dev_idx: i32, slot_id: i32, on_time: i32, off_time_factor: i32},
    PriGetGatingEnabled{dev_idx: i32, slot_id: i32},
    PriSetGatingEnabled{dev_idx: i32, slot_id: i32, gating_enabled: bool},
    PriGetGateHighImpedance{dev_idx: i32, slot_id: i32},
    PriSetGateHighImpedance{dev_idx: i32, slot_id: i32, high_impedance: bool},
    PriDecodeWavelength{dev_idx: i32, slot_id: i32, wl_idx: i32},
    PriGetWavelengthIdx{dev_idx: i32, slot_id: i32},
    PriSetWavelengthIdx{dev_idx: i32, slot_id: i32, wl_idx: i32},
    PriGetIntensity{dev_idx: i32, slot_id: i32, wl_idx: i32},
    PriSetIntensity{dev_idx: i32, slot_id: i32, wl_idx: i32, w_intensity: u16},
}

#[derive(Debug)]
enum ClientResp {
    LibDecodeError(String),
    LibGetVersion(String),
    LibGetLibUSBVersion(String),
    LibIsRunningOnWine(bool),
    UsbOpenDevice(sepia2::types::UsbDevice),
    UsbOpenGetSerNumAndClose(sepia2::types::UsbDevice),
    UsbGetStrDescriptor(String),
    UsbGetStrDescrByIdx(String),
    UsbIsOpenDevice(bool),
    UsbCloseDevice(()),
    FwrDecodeErrPhaseName(String),
    FwrGetVersion(String),
    FwrGetLastError(sepia2::types::FwrError),
    FwrGetWorkingMode(i32),
    FwrSetWorkingMode(()),
    FwrRollBackToPermanentValues(()),
    FwrStoreAsPermanentValues(()),
    FwrGetModuleMap(i32),
    FwrGetModuleInfoByMapIdx(sepia2::types::ModuleInfo),
    FwrGetUptimeInfoByMapIdx(sepia2::types::UptimeInfo),
    FwrCreateSupportRequestText(String),
    FwrFreeModuleMap(()),
    PriGetDeviceInfo(sepia2::types::PrimaDevInfo),
    PriDecodeOperationMode(sepia2::types::PrimaModeInfo),
    PriGetOperationMode(i32),
    PriSetOperationMode(()),
    PriDecodeTriggerSource(sepia2::types::TriggerInfo),
    PriGetTriggerSource(i32),
    PriSetTriggerSource(()),
    PriGetTriggerLevelLimits(sepia2::types::TriggerLevelInfo),
    PriGetTriggerLevel(i32),
    PriSetTriggerLevel(()),
    PriGetFrequencyLimits((i32, i32)),
    PriGetFrequency(i32),
    PriSetFrequency(()),
    PriGetGatingLimits(sepia2::types::PrimaGatingInfo),
    PriGetGatingData((i32, i32)),
    PriSetGatingData(()),
    PriGetGatingEnabled(bool),
    PriSetGatingEnabled(()),
    PriGetGateHighImpedance(bool),
    PriSetGateHighImpedance(()),
    PriDecodeWavelength(i32),
    PriGetWavelengthIdx(i32),
    PriSetWavelengthIdx(()),
    PriGetIntensity(u16),
    PriSetIntensity(()),
}

lazy_static! {
    static ref CLIENT_SERVICE_CHANNEL: (mpsc::Sender<(ClientReq, oneshot::Sender<ClientResp>)>, mpsc::Receiver<(ClientReq, oneshot::Sender<ClientResp>)>) = mpsc::channel();
}
lazy_static! {
    static ref CLIENT_SERVICE_SENDER: mpsc::Sender<(ClientReq, oneshot::Sender<ClientResp>)> = CLIENT_SERVICE_CHANNEL.0.clone();
}
lazy_static! {
    static ref CLIENT_SERVICE_RECEIVER: mpsc::Receiver<(ClientReq, oneshot::Sender<ClientResp>)> = CLIENT_SERVICE_CHANNEL.1.clone();
}

macro_rules! unwrap_resp {
    ($func:ident, $resp:expr) => {
        match $resp {
            Ok(client_resp) => match client_resp {
                ClientResp::$func(inner_resp) => Ok(inner_resp),
                other_resp => Err(format!("Unexpected response: {:?}", other_resp).into()),
            },
            Err(e) => Err(format!("RecvError: {:?}", e)),
        }
    };
}

macro_rules! julia_result {
    ($expr:expr) => {
        match $expr {
            Ok(ret) => Ok(ret),
            Err(err_msg) => {
                match weak_handle!() {
                    Ok(handle) => {
                        // We're just throwing (and catching) an exception
                        let res = unsafe { Value::eval_string(&handle, format!("throw(ErrorException(\"{err_msg}\"))")) };
                        match res {
                            Ok(_) => panic!("Expected exception thrown"),
                            Err(e) => Err(e.leak()),
                        }
                    }
                    _ => panic!("not called from Julia"),
                }
            },
        }
    };
}

macro_rules! into_julia {
    ($val:expr) => {
        Ok($val.into())
    };
}

macro_rules! into_julia_string {
    ($val:expr) => {
        match weak_handle!() {
            Ok(handle) => {
                let str_val = JuliaString::new(handle, $val);
                Ok(TypedValue::new(handle, str_val).leak())
            }
            _ => panic!("not called from Julia"),
        }
    };
}

macro_rules! into_julia_tuple {
    ($val:expr) => {
        match weak_handle!() {
            Ok(handle) => {
                let tuple = Tuple2<i32,i32>::construct_type(handle, $val);
                Ok(TypedValue::new(handle, tuple).leak())
            }
            _ => panic!("not called from Julia"),
        }
    };
}

// ----------------------------------------------------------------
// Client thread channel command received query the service
// ----------------------------------------------------------------

async fn client_service(addr: &str) -> Result<(), Box<dyn std::error::Error>> {

    /// Wait for MPSC to receive instructions about adress to connect to
    // TODO: Read MPSC channel for ClientCommand::Connect
    let mut client = Sepia2Client::connect(addr.into()).await?;

    loop {
        let (req, resp_sender) = CLIENT_SERVICE_RECEIVER.recv().expect("Something gone wrong in the MPSC transmission");
        /// For each command received query the server through gRPC and return the result through a
        /// oneshot channel
        println!("*** SIMPLE RPC ***");
        println!("REQUEST = {:?}", req);
        let response = match req {
            // TODO: Write macro to tidy this up
            ClientReq::LibDecodeError{err_code}                                     => ClientResp::LibDecodeError              (client.LIB_DecodeError(err_code)                                    .await?),
            ClientReq::LibGetVersion                                                => ClientResp::LibGetVersion               (client.LIB_GetVersion()                                             .await?),
            ClientReq::LibGetLibUSBVersion                                          => ClientResp::LibGetLibUSBVersion         (client.LIB_GetLibUSBVersion()                                       .await?),
            ClientReq::LibIsRunningOnWine                                           => ClientResp::LibIsRunningOnWine          (client.LIB_IsRunningOnWine()                                        .await?),
            ClientReq::UsbOpenDevice{dev_idx}                                       => ClientResp::UsbOpenDevice               (client.USB_OpenDevice(dev_idx)                                      .await?),
            ClientReq::UsbOpenGetSerNumAndClose{dev_idx}                            => ClientResp::UsbOpenGetSerNumAndClose    (client.USB_OpenGetSerNumAndClose(dev_idx)                           .await?),
            ClientReq::UsbGetStrDescriptor{dev_idx}                                 => ClientResp::UsbGetStrDescriptor         (client.USB_GetStrDescriptor(dev_idx)                                .await?),
            ClientReq::UsbGetStrDescrByIdx{dev_idx, descr_idx}                      => ClientResp::UsbGetStrDescrByIdx         (client.USB_GetStrDescrByIdx(dev_idx, descr_idx)                     .await?),
            ClientReq::UsbIsOpenDevice{dev_idx}                                     => ClientResp::UsbIsOpenDevice             (client.USB_IsOpenDevice(dev_idx)                                    .await?),
            ClientReq::UsbCloseDevice{dev_idx}                                      => ClientResp::UsbCloseDevice              (client.USB_CloseDevice(dev_idx)                                     .await?),
            ClientReq::FwrDecodeErrPhaseName{err_phase}                             => ClientResp::FwrDecodeErrPhaseName       (client.FWR_DecodeErrPhaseName(err_phase)                            .await?),
            ClientReq::FwrGetVersion{dev_idx}                                       => ClientResp::FwrGetVersion               (client.FWR_GetVersion(dev_idx)                                      .await?),
            ClientReq::FwrGetLastError{dev_idx}                                     => ClientResp::FwrGetLastError             (client.FWR_GetLastError(dev_idx)                                    .await?),
            ClientReq::FwrGetWorkingMode{dev_idx}                                   => ClientResp::FwrGetWorkingMode           (client.FWR_GetWorkingMode(dev_idx)                                  .await?),
            ClientReq::FwrSetWorkingMode{dev_idx, mode}                             => ClientResp::FwrSetWorkingMode           (client.FWR_SetWorkingMode(dev_idx, mode)                            .await?),
            ClientReq::FwrRollBackToPermanentValues{dev_idx}                        => ClientResp::FwrRollBackToPermanentValues(client.FWR_RollBackToPermanentValues(dev_idx)                       .await?),
            ClientReq::FwrStoreAsPermanentValues{dev_idx}                           => ClientResp::FwrStoreAsPermanentValues   (client.FWR_StoreAsPermanentValues(dev_idx)                          .await?),
            ClientReq::FwrGetModuleMap{dev_idx, perform_restart}                    => ClientResp::FwrGetModuleMap             (client.FWR_GetModuleMap(dev_idx, perform_restart)                   .await?),
            ClientReq::FwrGetModuleInfoByMapIdx{dev_idx, map_idx}                   => ClientResp::FwrGetModuleInfoByMapIdx    (client.FWR_GetModuleInfoByMapIdx(dev_idx, map_idx)                  .await?),
            ClientReq::FwrGetUptimeInfoByMapIdx{dev_idx, map_idx}                   => ClientResp::FwrGetUptimeInfoByMapIdx    (client.FWR_GetUptimeInfoByMapIdx(dev_idx, map_idx)                  .await?),
            ClientReq::FwrCreateSupportRequestText{dev_idx, fwr_req}                => ClientResp::FwrCreateSupportRequestText (client.FWR_CreateSupportRequestText(dev_idx, fwr_req)               .await?),
            ClientReq::FwrFreeModuleMap{dev_idx}                                    => ClientResp::FwrFreeModuleMap            (client.FWR_FreeModuleMap(dev_idx)                                   .await?),
            ClientReq::PriGetDeviceInfo{dev_idx, slot_id}                           => ClientResp::PriGetDeviceInfo            (client.PRI_GetDeviceInfo(dev_idx, slot_id)                          .await?),
            ClientReq::PriDecodeOperationMode{dev_idx, slot_id, oper_mode_idx}      => ClientResp::PriDecodeOperationMode      (client.PRI_DecodeOperationMode(dev_idx, slot_id, oper_mode_idx)     .await?),
            ClientReq::PriGetOperationMode{dev_idx, slot_id}                        => ClientResp::PriGetOperationMode         (client.PRI_GetOperationMode(dev_idx, slot_id)                       .await?),
            ClientReq::PriSetOperationMode{dev_idx, slot_id, oper_mode_idx}         => ClientResp::PriSetOperationMode         (client.PRI_SetOperationMode(dev_idx, slot_id, oper_mode_idx)        .await?),
            ClientReq::PriDecodeTriggerSource{dev_idx, slot_id, trg_src_idx}        => ClientResp::PriDecodeTriggerSource      (client.PRI_DecodeTriggerSource(dev_idx, slot_id, trg_src_idx)       .await?),
            ClientReq::PriGetTriggerSource{dev_idx, slot_id}                        => ClientResp::PriGetTriggerSource         (client.PRI_GetTriggerSource(dev_idx, slot_id)                       .await?),
            ClientReq::PriSetTriggerSource{dev_idx, slot_id, trg_src_idx}           => ClientResp::PriSetTriggerSource         (client.PRI_SetTriggerSource(dev_idx, slot_id, trg_src_idx)          .await?),
            ClientReq::PriGetTriggerLevelLimits{dev_idx, slot_id}                   => ClientResp::PriGetTriggerLevelLimits    (client.PRI_GetTriggerLevelLimits(dev_idx, slot_id)                  .await?),
            ClientReq::PriGetTriggerLevel{dev_idx, slot_id}                         => ClientResp::PriGetTriggerLevel          (client.PRI_GetTriggerLevel(dev_idx, slot_id)                        .await?),
            ClientReq::PriSetTriggerLevel{dev_idx, slot_id, trg_lvl}                => ClientResp::PriSetTriggerLevel          (client.PRI_SetTriggerLevel(dev_idx, slot_id, trg_lvl)               .await?),
            ClientReq::PriGetFrequencyLimits{dev_idx, slot_id}                      => ClientResp::PriGetFrequencyLimits       (client.PRI_GetFrequencyLimits(dev_idx, slot_id)                     .await?),
            ClientReq::PriGetFrequency{dev_idx, slot_id}                            => ClientResp::PriGetFrequency             (client.PRI_GetFrequency(dev_idx, slot_id)                           .await?),
            ClientReq::PriSetFrequency{dev_idx, slot_id, frequency}                 => ClientResp::PriSetFrequency             (client.PRI_SetFrequency(dev_idx, slot_id, frequency)                .await?),
            ClientReq::PriGetGatingLimits{dev_idx, slot_id}                         => ClientResp::PriGetGatingLimits          (client.PRI_GetGatingLimits(dev_idx, slot_id)                        .await?),
            ClientReq::PriGetGatingData{dev_idx, slot_id}                           => ClientResp::PriGetGatingData            (client.PRI_GetGatingData(dev_idx, slot_id)                          .await?),
            ClientReq::PriSetGatingData{dev_idx, slot_id, on_time, off_time_factor} => ClientResp::PriSetGatingData            (client.PRI_SetGatingData(dev_idx, slot_id, on_time, off_time_factor).await?),
            ClientReq::PriGetGatingEnabled{dev_idx, slot_id}                        => ClientResp::PriGetGatingEnabled         (client.PRI_GetGatingEnabled(dev_idx, slot_id)                       .await?),
            ClientReq::PriSetGatingEnabled{dev_idx, slot_id, gating_enabled}        => ClientResp::PriSetGatingEnabled         (client.PRI_SetGatingEnabled(dev_idx, slot_id, gating_enabled)       .await?),
            ClientReq::PriGetGateHighImpedance{dev_idx, slot_id}                    => ClientResp::PriGetGateHighImpedance     (client.PRI_GetGateHighImpedance(dev_idx, slot_id)                   .await?),
            ClientReq::PriSetGateHighImpedance{dev_idx, slot_id, high_impedance}    => ClientResp::PriSetGateHighImpedance     (client.PRI_SetGateHighImpedance(dev_idx, slot_id, high_impedance)   .await?),
            ClientReq::PriDecodeWavelength{dev_idx, slot_id, wl_idx}                => ClientResp::PriDecodeWavelength         (client.PRI_DecodeWavelength(dev_idx, slot_id, wl_idx)               .await?),
            ClientReq::PriGetWavelengthIdx{dev_idx, slot_id}                        => ClientResp::PriGetWavelengthIdx         (client.PRI_GetWavelengthIdx(dev_idx, slot_id)                       .await?),
            ClientReq::PriSetWavelengthIdx{dev_idx, slot_id, wl_idx}                => ClientResp::PriSetWavelengthIdx         (client.PRI_SetWavelengthIdx(dev_idx, slot_id, wl_idx)               .await?),
            ClientReq::PriGetIntensity{dev_idx, slot_id, wl_idx}                    => ClientResp::PriGetIntensity             (client.PRI_GetIntensity(dev_idx, slot_id, wl_idx)                   .await?),
            ClientReq::PriSetIntensity{dev_idx, slot_id, wl_idx, w_intensity}       => ClientResp::PriSetIntensity             (client.PRI_SetIntensity(dev_idx, slot_id, wl_idx, w_intensity)      .await?),
        };
        println!("RESPONSE = {:?}", response);
        resp_sender.send(response)?;
    }
}

// ----------------------------------------------------------------
// Tonic stub API
// ----------------------------------------------------------------

fn Spawn_Client(addr: JuliaString) {
    // TODO: Spawn a new thread for the client service
    //Sepia2Client::new(addr.into()).expect("Couldn't connect to gRPC server {addr}");
    todo!()
}

// TODO: Replace all this with a macro_rule defined for all functions

//fn LIB_DecodeError( err_code: i32,) -> Result<TypedValueRet<JuliaString>, ValueRet> {
//    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::LibDecodeError),ValueRet>>
//    CLIENT_SERVICE_SENDER.send((ClientReq::LibDecodeError{ err_code }, resp_sender));
//    julia_result!(unwrap_resp!(LibDecodeError, resp_receiver.recv()))
//}

//fn LIB_GetVersion() -> Result<TypedValueRet<JuliaString>, ValueRet> {
//    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::LibGetVersion),ValueRet>>
//    CLIENT_SERVICE_SENDER.send((ClientReq::LibGetVersion, resp_sender));
//    julia_result!(unwrap_resp!(LibGetVersion, resp_receiver.recv()))
//}

//fn LIB_GetLibUSBVersion() -> Result<TypedValueRet<JuliaString>, ValueRet> {
//    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::LibGetLibUSBVersion),ValueRet>>
//    CLIENT_SERVICE_SENDER.send((ClientReq::LibGetLibUSBVersion, resp_sender));
//    julia_result!(unwrap_resp!(LibGetLibUSBVersion, resp_receiver.recv()))
//}

fn LIB_IsRunningOnWine() -> Result<bool, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::LibIsRunningOnWine),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::LibIsRunningOnWine, resp_sender));
    julia_result!(unwrap_resp!(LibIsRunningOnWine, resp_receiver.recv()))
}

fn USB_OpenDevice(
    dev_idx: i32,
) -> Result<UsbDevice, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::UsbOpenDevice),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::UsbOpenDevice{ dev_idx }, resp_sender));
    into_julia!(julia_result!(unwrap_resp!(UsbOpenDevice, resp_receiver.recv()))?)
}
fn USB_OpenGetSerNumAndClose(
    dev_idx: i32,
) -> Result<UsbDevice, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::UsbOpenGetSerNumAndClose),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::UsbOpenGetSerNumAndClose{ dev_idx }, resp_sender));
    into_julia!(julia_result!(unwrap_resp!(UsbOpenGetSerNumAndClose, resp_receiver.recv()))?)
pub }
//fn USB_GetStrDescriptor(
//    dev_idx: i32,
//) -> Result<TypedValueRet<JuliaString>, ValueRet> {
//    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::UsbGetStrDescriptor),ValueRet>>
//    CLIENT_SERVICE_SENDER.send((ClientReq::UsbGetStrDescriptor{ dev_idx, }, resp_sender));
//    julia_result!(unwrap_resp!(UsbGetStrDescriptor, resp_receiver.recv()))
//}
//fn USB_GetStrDescrByIdx(
//    dev_idx: i32,
//    descr_idx: i32,
//) -> Result<TypedValueRet<JuliaString>, ValueRet> {
//    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::UsbGetStrDescrByIdx),ValueRet>>
//    CLIENT_SERVICE_SENDER.send((ClientReq::UsbGetStrDescrByIdx{ dev_idx, descr_idx, }, resp_sender));
//    julia_result!(unwrap_resp!(UsbGetStrDescrByIdx, resp_receiver.recv()))
//}
fn USB_IsOpenDevice(
    dev_idx: i32,
) -> Result<bool, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::UsbIsOpenDevice),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::UsbIsOpenDevice{ dev_idx, }, resp_sender));
    julia_result!(unwrap_resp!(UsbIsOpenDevice, resp_receiver.recv()))
}
fn USB_CloseDevice(
    dev_idx: i32,
) -> Result<(), ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::UsbCloseDevice),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::UsbCloseDevice{ dev_idx, }, resp_sender));
    julia_result!(unwrap_resp!(UsbCloseDevice, resp_receiver.recv()))
}
//fn FWR_DecodeErrPhaseName(
//    err_phase: i32,
//) -> Result<TypedValueRet<JuliaString>, ValueRet> {
//    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::FwrDecodeErrPhaseName),ValueRet>>
//    CLIENT_SERVICE_SENDER.send((ClientReq::FwrDecodeErrPhaseName{ err_phase, }, resp_sender));
//    julia_result!(unwrap_resp!(FwrDecodeErrPhaseName, resp_receiver.recv()))
//}
//fn FWR_GetVersion(
//    dev_idx: i32,
//) -> Result<TypedValueRet<JuliaString>, ValueRet> {
//    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::FwrGetVersion),ValueRet>>
//    CLIENT_SERVICE_SENDER.send((ClientReq::FwrGetVersion{ dev_idx, }, resp_sender));
//    into_julia_string!(julia_result!(unwrap_resp!(FwrGetVersion, resp_receiver.recv()))?)
//}
fn FWR_GetLastError(
    dev_idx: i32,
) -> Result<FwrError, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::FwrGetLastError),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::FwrGetLastError{ dev_idx, }, resp_sender));
    into_julia!(julia_result!(unwrap_resp!(FwrGetLastError, resp_receiver.recv()))?)
}
fn FWR_GetWorkingMode(
    dev_idx: i32,
) -> Result<i32, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::FwrGetWorkingMode),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::FwrGetWorkingMode{ dev_idx, }, resp_sender));
    julia_result!(unwrap_resp!(FwrGetWorkingMode, resp_receiver.recv()))
}
fn FWR_SetWorkingMode(
    dev_idx: i32,
    mode: i32,
) -> Result<(), ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::FwrSetWorkingMode),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::FwrSetWorkingMode{ dev_idx, mode, }, resp_sender));
    julia_result!(unwrap_resp!(FwrSetWorkingMode, resp_receiver.recv()))
}
fn FWR_RollBackToPermanentValues(
    dev_idx: i32,
) -> Result<(), ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::FwrRollBackToPermanentValues),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::FwrRollBackToPermanentValues{ dev_idx, }, resp_sender));
    julia_result!(unwrap_resp!(FwrRollBackToPermanentValues, resp_receiver.recv()))
}
fn FWR_StoreAsPermanentValues(
    dev_idx: i32,
) -> Result<(), ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::FwrStoreAsPermanentValues),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::FwrStoreAsPermanentValues{ dev_idx, }, resp_sender));
    julia_result!(unwrap_resp!(FwrStoreAsPermanentValues, resp_receiver.recv()))
}
fn FWR_GetModuleMap(
    dev_idx: i32,
    perform_restart: Bool,
) -> Result<i32, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::FwrGetModuleMap),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::FwrGetModuleMap{ dev_idx, perform_restart: perform_restart.as_bool(), }, resp_sender));
    julia_result!(unwrap_resp!(FwrGetModuleMap, resp_receiver.recv()))
}
fn FWR_GetModuleInfoByMapIdx(
    dev_idx: i32,
    map_idx: i32,
) -> Result<ModuleInfo, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::FwrGetModuleInfoByMapIdx),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::FwrGetModuleInfoByMapIdx{ dev_idx, map_idx, }, resp_sender));
    into_julia!(julia_result!(unwrap_resp!(FwrGetModuleInfoByMapIdx, resp_receiver.recv()))?)
}
fn FWR_GetUptimeInfoByMapIdx(
    dev_idx: i32,
    map_idx: i32,
) -> Result<UptimeInfo, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::FwrGetUptimeInfoByMapIdx),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::FwrGetUptimeInfoByMapIdx{ dev_idx, map_idx, }, resp_sender));
    into_julia!(julia_result!(unwrap_resp!(FwrGetUptimeInfoByMapIdx, resp_receiver.recv()))?)
}
//fn FWR_CreateSupportRequestText(
//    dev_idx: i32,
//    fwr_req: FwrRequestSupport,
//) -> Result<TypedValueRet<JuliaString>, ValueRet> {
//    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::FwrCreateSupportRequestText),ValueRet>>
//    CLIENT_SERVICE_SENDER.send((ClientReq::FwrCreateSupportRequestText{ dev_idx, fwr_req, }, resp_sender));
//    into_julia_string!(julia_result!(unwrap_resp!(FwrCreateSupportRequestText, resp_receiver.recv()))?)
//}
fn FWR_FreeModuleMap(
    dev_idx: i32,
) -> Result<(), ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::FwrFreeModuleMap),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::FwrFreeModuleMap{ dev_idx, }, resp_sender));
    julia_result!(unwrap_resp!(FwrFreeModuleMap, resp_receiver.recv()))
}

fn PRI_GetDeviceInfo(
    dev_idx: i32,
    slot_id: i32,
) -> Result<PrimaDevInfo, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriGetDeviceInfo),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriGetDeviceInfo{ dev_idx, slot_id, }, resp_sender));
    into_julia!(julia_result!(unwrap_resp!(PriGetDeviceInfo, resp_receiver.recv()))?)
}
fn PRI_DecodeOperationMode(
    dev_idx: i32,
    slot_id: i32,
    oper_mode_idx: i32,
) -> Result<PrimaModeInfo, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriDecodeOperationMode),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriDecodeOperationMode{ dev_idx, slot_id, oper_mode_idx, }, resp_sender));
    into_julia!(julia_result!(unwrap_resp!(PriDecodeOperationMode, resp_receiver.recv()))?)
}
fn PRI_GetOperationMode(
    dev_idx: i32,
    slot_id: i32,
) -> Result<i32, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriGetOperationMode),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriGetOperationMode{ dev_idx, slot_id, }, resp_sender));
    julia_result!(unwrap_resp!(PriGetOperationMode, resp_receiver.recv()))
}
fn PRI_SetOperationMode(
    dev_idx: i32,
    slot_id: i32,
    oper_mode_idx: i32,
) -> Result<(), ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriSetOperationMode),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriSetOperationMode{ dev_idx, slot_id, oper_mode_idx, }, resp_sender));
    julia_result!(unwrap_resp!(PriSetOperationMode, resp_receiver.recv()))
}
fn PRI_DecodeTriggerSource(
    dev_idx: i32,
    slot_id: i32,
    trg_src_idx: i32,
) -> Result<TriggerInfo, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriDecodeTriggerSource),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriDecodeTriggerSource{ dev_idx, slot_id, trg_src_idx, }, resp_sender));
    into_julia!(julia_result!(unwrap_resp!(PriDecodeTriggerSource, resp_receiver.recv()))?)
}
fn PRI_GetTriggerSource(
    dev_idx: i32,
    slot_id: i32,
) -> Result<i32, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriGetTriggerSource),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriGetTriggerSource{ dev_idx, slot_id, }, resp_sender));
    julia_result!(unwrap_resp!(PriGetTriggerSource, resp_receiver.recv()))
}
fn PRI_SetTriggerSource(
    dev_idx: i32,
    slot_id: i32,
    trg_src_idx: i32,
) -> Result<(), ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriSetTriggerSource),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriSetTriggerSource{ dev_idx, slot_id, trg_src_idx, }, resp_sender));
    julia_result!(unwrap_resp!(PriSetTriggerSource, resp_receiver.recv()))
}
fn PRI_GetTriggerLevelLimits(
    dev_idx: i32,
    slot_id: i32,
) -> Result<TriggerLevelInfo, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriGetTriggerLevelLimits),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriGetTriggerLevelLimits{ dev_idx, slot_id, }, resp_sender));
    into_julia!(julia_result!(unwrap_resp!(PriGetTriggerLevelLimits, resp_receiver.recv()))?)
}
fn PRI_GetTriggerLevel(
    dev_idx: i32,
    slot_id: i32,
) -> Result<i32, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriGetTriggerLevel),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriGetTriggerLevel{ dev_idx, slot_id, }, resp_sender));
    julia_result!(unwrap_resp!(PriGetTriggerLevel, resp_receiver.recv()))
}
fn PRI_SetTriggerLevel(
    dev_idx: i32,
    slot_id: i32,
    trg_lvl: i32,
) -> Result<(), ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriSetTriggerLevel),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriSetTriggerLevel{ dev_idx, slot_id, trg_lvl, }, resp_sender));
    julia_result!(unwrap_resp!(PriSetTriggerLevel, resp_receiver.recv()))
}
fn PRI_GetFrequencyLimits(
    dev_idx: i32,
    slot_id: i32,
) -> Result<TypedValueRet<Tuple2<i32, i32>>, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriGetFrequencyLimits),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriGetFrequencyLimits{ dev_idx, slot_id, }, resp_sender));
    into_julia_tuple!(julia_result!(unwrap_resp!(PriGetFrequencyLimits, resp_receiver.recv()))?)
}
fn PRI_GetFrequency(
    dev_idx: i32,
    slot_id: i32,
) -> Result<i32, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriGetFrequency),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriGetFrequency{ dev_idx, slot_id, }, resp_sender));
    julia_result!(unwrap_resp!(PriGetFrequency, resp_receiver.recv()))
}
fn PRI_SetFrequency(
    dev_idx: i32,
    slot_id: i32,
    frequency: i32,
) -> Result<(), ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriSetFrequency),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriSetFrequency{ dev_idx, slot_id, frequency, }, resp_sender));
    julia_result!(unwrap_resp!(PriSetFrequency, resp_receiver.recv()))
}
fn PRI_GetGatingLimits(
    dev_idx: i32,
    slot_id: i32,
) -> Result<PrimaGatingInfo, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriGetGatingLimits),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriGetGatingLimits{ dev_idx, slot_id, }, resp_sender));
    into_julia!(julia_result!(unwrap_resp!(PriGetGatingLimits, resp_receiver.recv()))?)
}
fn PRI_GetGatingData(
    dev_idx: i32,
    slot_id: i32,
) -> Result<TypedValueRet<Tuple2<i32, i32>>, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriGetGatingData),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriGetGatingData{ dev_idx, slot_id, }, resp_sender));
    into_julia_tuple!(julia_result!(unwrap_resp!(PriGetGatingData, resp_receiver.recv()))?)
}
fn PRI_SetGatingData(
    dev_idx: i32,
    slot_id: i32,
    on_time: i32,
    off_time_factor: i32,
) -> Result<(), ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriSetGatingData),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriSetGatingData{ dev_idx, slot_id, on_time, off_time_factor, }, resp_sender));
    julia_result!(unwrap_resp!(PriSetGatingData, resp_receiver.recv()))
}
fn PRI_GetGatingEnabled(
    dev_idx: i32,
    slot_id: i32,
) -> Result<bool, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriGetGatingEnabled),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriGetGatingEnabled{ dev_idx, slot_id, }, resp_sender));
    julia_result!(unwrap_resp!(PriGetGatingEnabled, resp_receiver.recv()))
}
fn PRI_SetGatingEnabled(
    dev_idx: i32,
    slot_id: i32,
    gating_enabled: Bool,
) -> Result<(), ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriSetGatingEnabled),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriSetGatingEnabled{ dev_idx, slot_id, gating_enabled: gating_enabled.as_bool(), }, resp_sender));
    julia_result!(unwrap_resp!(PriSetGatingEnabled, resp_receiver.recv()))
}
fn PRI_GetGateHighImpedance(
    dev_idx: i32,
    slot_id: i32,
) -> Result<bool, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriGetGateHighImpedance),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriGetGateHighImpedance{ dev_idx, slot_id, }, resp_sender));
    julia_result!(unwrap_resp!(PriGetGateHighImpedance, resp_receiver.recv()))
}
fn PRI_SetGateHighImpedance(
    dev_idx: i32,
    slot_id: i32,
    high_impedance: Bool,
) -> Result<(), ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriSetGateHighImpedance),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriSetGateHighImpedance{ dev_idx, slot_id, high_impedance: high_impedance.as_bool(), }, resp_sender));
    julia_result!(unwrap_resp!(PriSetGateHighImpedance, resp_receiver.recv()))
}
fn PRI_DecodeWavelength(
    dev_idx: i32,
    slot_id: i32,
    wl_idx: i32,
) -> Result<i32, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriDecodeWavelength),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriDecodeWavelength{ dev_idx, slot_id, wl_idx, }, resp_sender));
    julia_result!(unwrap_resp!(PriDecodeWavelength, resp_receiver.recv()))
}
fn PRI_GetWavelengthIdx(
    dev_idx: i32,
    slot_id: i32,
) -> Result<i32, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriGetWavelengthIdx),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriGetWavelengthIdx{ dev_idx, slot_id, }, resp_sender));
    julia_result!(unwrap_resp!(PriGetWavelengthIdx, resp_receiver.recv()))
}
fn PRI_SetWavelengthIdx(
    dev_idx: i32,
    slot_id: i32,
    wl_idx: i32,
) -> Result<(), ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriSetWavelengthIdx),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriSetWavelengthIdx{ dev_idx, slot_id, wl_idx, }, resp_sender));
    julia_result!(unwrap_resp!(PriSetWavelengthIdx, resp_receiver.recv()))
}
fn PRI_GetIntensity(
    dev_idx: i32,
    slot_id: i32,
    wl_idx: i32,
) -> Result<u16, ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriGetIntensity),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriGetIntensity{ dev_idx, slot_id, wl_idx, }, resp_sender));
    julia_result!(unwrap_resp!(PriGetIntensity, resp_receiver.recv()))
}
fn PRI_SetIntensity(
    dev_idx: i32,
    slot_id: i32,
    wl_idx: i32,
    w_intensity: u16,
) -> Result<(), ValueRet> {
    let (resp_sender, resp_receiver) = oneshot::channel(); // oneshot::Channel<Result<return_type_of!(ClientReq::PriSetIntensity),ValueRet>>
    CLIENT_SERVICE_SENDER.send((ClientReq::PriSetIntensity{ dev_idx, slot_id, wl_idx, w_intensity, }, resp_sender));
    julia_result!(unwrap_resp!(PriSetIntensity, resp_receiver.recv()))
}

julia_module! {
    become sepia2_client_init_fn;

    struct UsbDevice;
    struct FwrError;
    struct FwrRequestSupport;
    struct ModuleInfo;
    struct UptimeInfo;
    struct PrimaDevInfo;
    struct PrimaModeInfo;
    struct TriggerInfo;
    struct TriggerLevelInfo;
    struct PrimaGatingInfo;

    fn Spawn_Client(addr: JuliaString) ;
//    fn LIB_DecodeError(err_code: i32,)                        -> Result<TypedValueRet<JuliaString>, ValueRet> ;
//    fn LIB_GetVersion()                                       -> Result<TypedValueRet<JuliaString>, ValueRet> ;
//    fn LIB_GetLibUSBVersion()                                 -> Result<TypedValueRet<JuliaString>, ValueRet> ;
    fn LIB_IsRunningOnWine()                                  -> Result<bool, ValueRet> ;
    fn USB_OpenDevice(dev_idx: i32,)                          -> Result<UsbDevice, ValueRet> ;
    fn USB_OpenGetSerNumAndClose(dev_idx: i32,)               -> Result<UsbDevice, ValueRet> ;
//    fn USB_GetStrDescriptor(dev_idx: i32,)                    -> Result<TypedValueRet<JuliaString>, ValueRet> ;
//    fn USB_GetStrDescrByIdx(dev_idx: i32, descr_idx: i32,)    -> Result<TypedValueRet<JuliaString>, ValueRet> ;
    fn USB_IsOpenDevice(dev_idx: i32,)                        -> Result<bool, ValueRet> ;
    fn USB_CloseDevice(dev_idx: i32,)                         -> Result<(), ValueRet> ;
//    fn FWR_DecodeErrPhaseName(err_phase: i32,)                -> Result<TypedValueRet<JuliaString>, ValueRet> ;
//    fn FWR_GetVersion(dev_idx: i32,)                          -> Result<TypedValueRet<JuliaString>, ValueRet> ;
    fn FWR_GetLastError(dev_idx: i32,)                        -> Result<FwrError, ValueRet> ;
    fn FWR_GetWorkingMode(dev_idx: i32,)                      -> Result<i32, ValueRet> ;
    fn FWR_SetWorkingMode(dev_idx: i32, mode: i32,)           -> Result<(), ValueRet> ;
    fn FWR_RollBackToPermanentValues(dev_idx: i32,)           -> Result<(), ValueRet> ;
    fn FWR_StoreAsPermanentValues(dev_idx: i32)              -> Result<(), ValueRet> ;
    fn FWR_GetModuleMap(dev_idx: i32, perform_restart: Bool) -> Result<i32, ValueRet> ;
    fn FWR_GetModuleInfoByMapIdx( dev_idx: i32, map_idx: i32) -> Result<ModuleInfo, ValueRet> ;
    fn FWR_GetUptimeInfoByMapIdx( dev_idx: i32, map_idx: i32) -> Result<UptimeInfo, ValueRet> ;

//    fn FWR_CreateSupportRequestText( dev_idx: i32, fwr_req: FwrRequestSupport) -> Result<TypedValueRet<JuliaString>, ValueRet>;

    fn FWR_FreeModuleMap( dev_idx: i32,)                                         -> Result<(), ValueRet> ;
    fn PRI_GetDeviceInfo( dev_idx: i32, slot_id: i32,)                           -> Result<PrimaDevInfo, ValueRet> ;
    fn PRI_DecodeOperationMode( dev_idx: i32, slot_id: i32, oper_mode_idx: i32,) -> Result<PrimaModeInfo, ValueRet> ;
    fn PRI_GetOperationMode( dev_idx: i32, slot_id: i32,)                        -> Result<i32, ValueRet> ;
    fn PRI_SetOperationMode( dev_idx: i32, slot_id: i32, oper_mode_idx: i32,)    -> Result<(), ValueRet> ;
    fn PRI_DecodeTriggerSource( dev_idx: i32, slot_id: i32, trg_src_idx: i32,)   -> Result<TriggerInfo, ValueRet> ;
    fn PRI_GetTriggerSource( dev_idx: i32, slot_id: i32,)                        -> Result<i32, ValueRet> ;
    fn PRI_SetTriggerSource( dev_idx: i32, slot_id: i32, trg_src_idx: i32,)      -> Result<(), ValueRet> ;
    fn PRI_GetTriggerLevelLimits( dev_idx: i32, slot_id: i32,)                   -> Result<TriggerLevelInfo, ValueRet> ;
    fn PRI_GetTriggerLevel( dev_idx: i32, slot_id: i32,)                         -> Result<i32, ValueRet> ;
    fn PRI_SetTriggerLevel( dev_idx: i32, slot_id: i32, trg_lvl: i32,)           -> Result<(), ValueRet> ;
    fn PRI_GetFrequencyLimits( dev_idx: i32, slot_id: i32,)                      -> Result<TypedValueRet<Tuple2<i32, i32>>, ValueRet> ;
    fn PRI_GetFrequency( dev_idx: i32, slot_id: i32,)                            -> Result<i32, ValueRet> ;
    fn PRI_SetFrequency( dev_idx: i32, slot_id: i32, frequency: i32,)            -> Result<(), ValueRet> ;
    fn PRI_GetGatingLimits( dev_idx: i32, slot_id: i32,)                         -> Result<PrimaGatingInfo, ValueRet> ;
    fn PRI_GetGatingData( dev_idx: i32, slot_id: i32,)                           -> Result<TypedValueRet<Tuple2<i32, i32>>, ValueRet> ;
    fn PRI_SetGatingData( dev_idx: i32, slot_id: i32, on_time: i32, off_time_factor: i32,) -> Result<(), ValueRet> ;

    fn PRI_GetGatingEnabled( dev_idx: i32, slot_id: i32,)                           -> Result<bool, ValueRet> ;
    fn PRI_SetGatingEnabled( dev_idx: i32, slot_id: i32, gating_enabled: Bool,)     -> Result<(), ValueRet> ;
    fn PRI_GetGateHighImpedance( dev_idx: i32, slot_id: i32,)                       -> Result<bool, ValueRet> ;
    fn PRI_SetGateHighImpedance( dev_idx: i32, slot_id: i32, high_impedance: Bool,) -> Result<(), ValueRet> ;
    fn PRI_DecodeWavelength( dev_idx: i32, slot_id: i32, wl_idx: i32,)              -> Result<i32, ValueRet> ;
    fn PRI_GetWavelengthIdx( dev_idx: i32, slot_id: i32,)                           -> Result<i32, ValueRet> ;
    fn PRI_SetWavelengthIdx( dev_idx: i32, slot_id: i32, wl_idx: i32,)              -> Result<(), ValueRet> ;
    fn PRI_GetIntensity( dev_idx: i32, slot_id: i32, wl_idx: i32,)                  -> Result<u16, ValueRet> ;

    fn PRI_SetIntensity( dev_idx: i32, slot_id: i32, wl_idx: i32, w_intensity: u16,) -> Result<(), ValueRet> ;
}
