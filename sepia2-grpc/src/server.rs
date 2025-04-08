use clap::Parser;
use core::net::SocketAddr;
use log::{debug, error, info, warn};
use env_logger::Env;
use tonic::{transport::Server, Request, Response, Status};

use sepia2::api::*;

mod named_pipe_stream;
mod sepia2_rpc;

use named_pipe_stream::get_named_pipe_transport;
use sepia2_rpc::sepia2_server::{Sepia2, Sepia2Server};

// ---------------------------------------------------------
// Server runtime
// ---------------------------------------------------------

/// gRPC server that controls PQ Laser through the Sepia2 library
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Choose TCP Socket as transport type and pick port to listen on
    #[arg(short, long)]
    tcp_socket: Option<String>,

    /// Choose named pipes as trasnport and pick named for the pipe
    #[arg(short, long)]
    pipename: Option<String>,

    /// Set port forwarding on the given machine
    #[arg(short, long, default_value_t = false)]
    forwarding: bool,

    /// Force system type, otherwise it is detected automatically
    #[arg(short, long)]
    system: Option<String>,

    /// Is the system running on wine? Consider how to setup port forwarding on the Linux host
    #[arg(short, long, default_value_t = false)]
    wine: bool,

    /// Enable reflection for gRPC
    #[arg(short, long, default_value_t = false)]
    reflection: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //env_logger::init();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    println!("Env logger initialized");
    let args = Args::parse();

    // tpc_socket |-> ~pipename
    assert!(
        !args.tcp_socket.is_some() || !args.pipename.is_some(),
        "Expecting mutually exclusive tcp_socket or pipename"
    );

    // Setup port forwarding
    if args.forwarding {
        todo!("Setup port forwarding");
        if args.wine {
            todo!("Setup port forwarding on the linux host");
        }
    }

    let sepia2_service = Sepia2Service::default();
    let svc = Sepia2Server::new(sepia2_service);

    let mut server = Server::builder().add_service(svc);

    if args.reflection {
        info!("gRPC reflection enabled");
        let reflection_svc = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(sepia2_rpc::FILE_DESCRIPTOR_SET)
            .build_v1()?;
        server = server.add_service(reflection_svc);
    }

    info!("Starting gRPC server for Sepia2 Lib");

    if let Some(addr) = args.tcp_socket {
        debug!("Trying to bind to: {}", addr);
        let addr: SocketAddr = addr.parse()?;
        info!("Listening on {}", addr);
        server.serve(addr).await?;
    } else if let Some(pipename) = args.pipename {
        info!("Listening on pipe: {}", pipename);
        server
            .serve_with_incoming(get_named_pipe_transport(&pipename))
            .await?;
    } else {
        let addr: SocketAddr = "[::1]:50051".parse().unwrap();
        warn!("No socket chosen, default addr: {:?}", addr);
        server.serve(addr).await?;
    }

    Ok(())
}

// ---------------------------------------------------------
// Server endpoints
// ---------------------------------------------------------

// TODO: Add here any state that we need to keep track of about the Laser
#[derive(Debug, Default)]
pub struct Sepia2Service;

#[tonic::async_trait]
//TODO: #[expand_shim] macro to generate all the functions
impl Sepia2 for Sepia2Service {
    async fn lib_decode_error(
        &self,
        request: Request<sepia2_rpc::Error>,
    ) -> Result<Response<sepia2_rpc::LibDecodeErrorResponse>, Status> {
        println!("Got a request for LIB_DecodeError: {:?}", request);
        match request.into_inner().error {
            Some(error) => match LIB_DecodeError(error) {
                Ok(err_str) => Ok(Response::new(sepia2_rpc::LibDecodeErrorResponse {
                    err_str,
                })),
                Err(e) => {
                    error!("Calling LIB_DecodeError: {}", e);
                    Err(Status::new(
                        tonic::Code::Internal,
                        format!("Calling LIB_DecodeError: {}", e),
                    ))
                }
            },
            None => Err(Status::new(
                tonic::Code::InvalidArgument,
                "No error provided",
            )),
        }
    }

    // --- No parameters calls ---
    // FIXME: proc_macro expands before, hence need to understand what tonic::async_trait does,
    // and emulate it for:
    // shim_connector_basic!(lib_get_version, LIB_GetVersion, sepia2_rpc::Version);

    async fn lib_get_version(
        &self,
        _: Request<()>,
    ) -> Result<Response<sepia2_rpc::Version>, Status> {
        println!("Got a request for {}", "LIB_GetVersion");
        match LIB_GetVersion() {
            Ok(result) => Ok(Response::new(sepia2_rpc::Version { version: result })),
            Err(e) => {
                error!("Calling {}: {}", "LIB_GetVersion", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "LIB_GetVersion", e),
                ))
            }
        }
    }

    async fn lib_get_lib_usb_version(
        &self,
        _: Request<()>,
    ) -> Result<Response<sepia2_rpc::Version>, Status> {
        println!("Got a request for {}", "LIB_GetLibUSBVersion");
        match LIB_GetLibUSBVersion() {
            Ok(result) => Ok(Response::new(sepia2_rpc::Version { version: result })),
            Err(e) => {
                error!("Calling {}: {}", "LIB_GetLibUSBVersion", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "LIB_GetLibUSBVersion", e),
                ))
            }
        }
    }

    async fn lib_is_running_on_wine(
        &self,
        _: Request<()>,
    ) -> Result<Response<sepia2_rpc::Bool>, Status> {
        println!("Got a request for {}", "LIB_IsRunningOnWine");
        match LIB_IsRunningOnWine() {
            Ok(result) => Ok(Response::new(sepia2_rpc::Bool::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "LIB_IsRunningOnWine", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "LIB_IsRunningOnWine", e),
                ))
            }
        }
    }

    // --- DeviceIdx calls ---
    async fn usb_open_device(
        &self,
        req: Request<sepia2_rpc::DeviceIdx>,
    ) -> Result<Response<sepia2_rpc::UsbDevice>, Status> {
        println!("Got a request for {}: {:?}", "USB_OpenDevice", req);
        match USB_OpenDevice(req.into_inner().dev_idx) {
            Ok(result) => Ok(Response::new(sepia2_rpc::UsbDevice::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "USB_OpenDevice", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "USB_OpenDevice", e),
                ))
            }
        }
    }
    async fn usb_open_get_ser_num_and_close(
        &self,
        req: Request<sepia2_rpc::DeviceIdx>,
    ) -> Result<Response<sepia2_rpc::UsbDevice>, Status> {
        println!(
            "Got a request for {}: {:?}",
            "USB_OpenGetSerNumAndClose", req
        );
        match USB_OpenGetSerNumAndClose(req.into_inner().dev_idx) {
            Ok(result) => Ok(Response::new(sepia2_rpc::UsbDevice::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "USB_OpenGetSerNumAndClose", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "USB_OpenGetSerNumAndClose", e),
                ))
            }
        }
    }
    async fn usb_get_str_descriptor(
        &self,
        req: Request<sepia2_rpc::DeviceIdx>,
    ) -> Result<Response<sepia2_rpc::String>, Status> {
        println!("Got a request for {}: {:?}", "USB_GetStrDescriptor", req);
        match USB_GetStrDescriptor(req.into_inner().dev_idx) {
            Ok(result) => Ok(Response::new(sepia2_rpc::String::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "USB_GetStrDescriptor", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "USB_GetStrDescriptor", e),
                ))
            }
        }
    }
    async fn usb_get_str_descr_by_idx(
        &self,
        req: Request<sepia2_rpc::GetStrDescrByIdxRequest>,
    ) -> Result<Response<sepia2_rpc::String>, Status> {
        println!("Got a request for {}: {:?}", "USB_GetStrDescrByIdx", req);
        let req = req.into_inner();
        match USB_GetStrDescrByIdx(req.dev_idx, req.descr_idx) {
            Ok(result) => Ok(Response::new(sepia2_rpc::String::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "USB_GetStrDescrByIdx", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "USB_GetStrDescrByIdx", e),
                ))
            }
        }
    }
    async fn usb_is_open_device(
        &self,
        req: Request<sepia2_rpc::DeviceIdx>,
    ) -> Result<Response<sepia2_rpc::Bool>, Status> {
        println!("Got a request for {}: {:?}", "USB_IsOpenDevice", req);
        match USB_IsOpenDevice(req.into_inner().dev_idx) {
            Ok(result) => Ok(Response::new(sepia2_rpc::Bool::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "USB_IsOpenDevice", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "USB_IsOpenDevice", e),
                ))
            }
        }
    }
    async fn usb_close_device(
        &self,
        req: Request<sepia2_rpc::DeviceIdx>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request for {}: {:?}", "USB_CloseDevice", req);
        match USB_CloseDevice(req.into_inner().dev_idx) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                error!("Calling {}: {}", "USB_CloseDevice", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "USB_CloseDevice", e),
                ))
            }
        }
    }
    async fn fwr_get_version(
        &self,
        req: Request<sepia2_rpc::DeviceIdx>,
    ) -> Result<Response<sepia2_rpc::String>, Status> {
        println!("Got a request for {}: {:?}", "FWR_GetVersion", req);
        match FWR_GetVersion(req.into_inner().dev_idx) {
            Ok(result) => Ok(Response::new(sepia2_rpc::String::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "FWR_GetVersion", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "FWR_GetVersion", e),
                ))
            }
        }
    }
    async fn fwr_get_last_error(
        &self,
        req: Request<sepia2_rpc::DeviceIdx>,
    ) -> Result<Response<sepia2_rpc::FwrError>, Status> {
        println!("Got a request for {}: {:?}", "FWR_GetLastError", req);
        match FWR_GetLastError(req.into_inner().dev_idx) {
            Ok(result) => Ok(Response::new(sepia2_rpc::FwrError::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "FWR_GetLastError", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "FWR_GetLastError", e),
                ))
            }
        }
    }
    async fn fwr_get_working_mode(
        &self,
        req: Request<sepia2_rpc::DeviceIdx>,
    ) -> Result<Response<sepia2_rpc::Int32>, Status> {
        println!("Got a request for {}: {:?}", "FWR_GetWorkingMode", req);
        match FWR_GetWorkingMode(req.into_inner().dev_idx) {
            Ok(result) => Ok(Response::new(sepia2_rpc::Int32::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "FWR_GetWorkingMode", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "FWR_GetWorkingMode", e),
                ))
            }
        }
    }
    async fn fwr_roll_back_to_permanent_values(
        &self,
        req: Request<sepia2_rpc::DeviceIdx>,
    ) -> Result<Response<()>, Status> {
        println!(
            "Got a request for {}: {:?}",
            "FWR_RollBackToPermanentValues", req
        );
        match FWR_RollBackToPermanentValues(req.into_inner().dev_idx) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                error!("Calling {}: {}", "FWR_RollBackToPermanentValues", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "FWR_RollBackToPermanentValues", e),
                ))
            }
        }
    }
    async fn fwr_store_as_permanent_values(
        &self,
        req: Request<sepia2_rpc::DeviceIdx>,
    ) -> Result<Response<()>, Status> {
        println!(
            "Got a request for {}: {:?}",
            "FWR_StoreAsPermanentValues", req
        );
        match FWR_StoreAsPermanentValues(req.into_inner().dev_idx) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                error!("Calling {}: {}", "FWR_StoreAsPermanentValues", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "FWR_StoreAsPermanentValues", e),
                ))
            }
        }
    }
    async fn fwr_free_module_map(
        &self,
        req: Request<sepia2_rpc::DeviceIdx>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request for {}: {:?}", "FWR_FreeModuleMap", req);
        match FWR_FreeModuleMap(req.into_inner().dev_idx) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                error!("Calling {}: {}", "FWR_FreeModuleMap", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "FWR_FreeModuleMap", e),
                ))
            }
        }
    }

    // --- PriRequest calls ---
    async fn pri_get_device_info(
        &self,
        req: Request<sepia2_rpc::PriRequest>,
    ) -> Result<Response<sepia2_rpc::PrimaDevInfo>, Status> {
        println!("Got a request for {}: {:?}", "PRI_GetDeviceInfo", req);
        let req = req.into_inner();
        match PRI_GetDeviceInfo(req.dev_idx, req.slot_id) {
            Ok(result) => Ok(Response::new(sepia2_rpc::PrimaDevInfo::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "PRI_GetDeviceInfo", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_GetDeviceInfo", e),
                ))
            }
        }
    }
    async fn pri_get_operation_mode(
        &self,
        req: Request<sepia2_rpc::PriRequest>,
    ) -> Result<Response<sepia2_rpc::Int32>, Status> {
        println!("Got a request for {}: {:?}", "PRI_GetOperationMode", req);
        let req = req.into_inner();
        match PRI_GetOperationMode(req.dev_idx, req.slot_id) {
            Ok(result) => Ok(Response::new(sepia2_rpc::Int32::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "PRI_GetOperationMode", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_GetOperationMode", e),
                ))
            }
        }
    }
    async fn pri_get_trigger_source(
        &self,
        req: Request<sepia2_rpc::PriRequest>,
    ) -> Result<Response<sepia2_rpc::Int32>, Status> {
        println!("Got a request for {}: {:?}", "PRI_GetTriggerSource", req);
        let req = req.into_inner();
        match PRI_GetTriggerSource(req.dev_idx, req.slot_id) {
            Ok(result) => Ok(Response::new(sepia2_rpc::Int32::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "PRI_GetTriggerSource", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_GetTriggerSource", e),
                ))
            }
        }
    }
    async fn pri_get_trigger_level_limits(
        &self,
        req: Request<sepia2_rpc::PriRequest>,
    ) -> Result<Response<sepia2_rpc::TriggerLevelInfo>, Status> {
        println!(
            "Got a request for {}: {:?}",
            "PRI_GetTriggerLevelLimits", req
        );
        let req = req.into_inner();
        match PRI_GetTriggerLevelLimits(req.dev_idx, req.slot_id) {
            Ok(result) => Ok(Response::new(sepia2_rpc::TriggerLevelInfo::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "PRI_GetTriggerLevelLimits", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_GetTriggerLevelLimits", e),
                ))
            }
        }
    }
    async fn pri_get_trigger_level(
        &self,
        req: Request<sepia2_rpc::PriRequest>,
    ) -> Result<Response<sepia2_rpc::Int32>, Status> {
        println!("Got a request for {}: {:?}", "PRI_GetTriggerLevel", req);
        let req = req.into_inner();
        match PRI_GetTriggerLevel(req.dev_idx, req.slot_id) {
            Ok(result) => Ok(Response::new(sepia2_rpc::Int32::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "PRI_GetTriggerLevel", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_GetTriggerLevel", e),
                ))
            }
        }
    }
    async fn pri_get_frequency_limits(
        &self,
        req: Request<sepia2_rpc::PriRequest>,
    ) -> Result<Response<sepia2_rpc::FrequencyLimitsResponse>, Status> {
        println!("Got a request for {}: {:?}", "PRI_GetFrequencyLimits", req);
        let req = req.into_inner();
        match PRI_GetFrequencyLimits(req.dev_idx, req.slot_id) {
            Ok(result) => Ok(Response::new(sepia2_rpc::FrequencyLimitsResponse {
                min_freq: result.0,
                max_freq: result.1,
            })),
            Err(e) => {
                error!("Calling {}: {}", "PRI_GetFrequencyLimits", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_GetFrequencyLimits", e),
                ))
            }
        }
    }
    async fn pri_get_frequency(
        &self,
        req: Request<sepia2_rpc::PriRequest>,
    ) -> Result<Response<sepia2_rpc::Int32>, Status> {
        println!("Got a request for {}: {:?}", "PRI_GetFrequency", req);
        let req = req.into_inner();
        match PRI_GetFrequency(req.dev_idx, req.slot_id) {
            Ok(result) => Ok(Response::new(sepia2_rpc::Int32::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "PRI_GetFrequency", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_GetFrequency", e),
                ))
            }
        }
    }
    async fn pri_get_gating_limits(
        &self,
        req: Request<sepia2_rpc::PriRequest>,
    ) -> Result<Response<sepia2_rpc::PrimaGatingInfo>, Status> {
        println!("Got a request for {}: {:?}", "PRI_GetGatingLimits", req);
        let req = req.into_inner();
        match PRI_GetGatingLimits(req.dev_idx, req.slot_id) {
            Ok(result) => Ok(Response::new(sepia2_rpc::PrimaGatingInfo::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "PRI_GetGatingLimits", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_GetGatingLimits", e),
                ))
            }
        }
    }
    async fn pri_get_gating_data(
        &self,
        req: Request<sepia2_rpc::PriRequest>,
    ) -> Result<Response<sepia2_rpc::GatingData>, Status> {
        println!("Got a request for {}: {:?}", "PRI_GetGatingData", req);
        let req = req.into_inner();
        match PRI_GetGatingData(req.dev_idx, req.slot_id) {
            Ok(result) => Ok(Response::new(sepia2_rpc::GatingData {
                on_time: result.0,
                off_time_factor: result.1,
            })),
            Err(e) => {
                error!("Calling {}: {}", "PRI_GetGatingData", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_GetGatingData", e),
                ))
            }
        }
    }
    async fn pri_get_gating_enabled(
        &self,
        req: Request<sepia2_rpc::PriRequest>,
    ) -> Result<Response<sepia2_rpc::Bool>, Status> {
        println!("Got a request for {}: {:?}", "PRI_GetGatingEnabled", req);
        let req = req.into_inner();
        match PRI_GetGatingEnabled(req.dev_idx, req.slot_id) {
            Ok(result) => Ok(Response::new(sepia2_rpc::Bool::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "PRI_GetGatingEnabled", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_GetGatingEnabled", e),
                ))
            }
        }
    }
    async fn pri_get_gate_high_impedance(
        &self,
        req: Request<sepia2_rpc::PriRequest>,
    ) -> Result<Response<sepia2_rpc::Bool>, Status> {
        println!(
            "Got a request for {}: {:?}",
            "PRI_GetGateHighImpedance", req
        );
        let req = req.into_inner();
        match PRI_GetGateHighImpedance(req.dev_idx, req.slot_id) {
            Ok(result) => Ok(Response::new(sepia2_rpc::Bool::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "PRI_GetGateHighImpedance", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_GetGateHighImpedance", e),
                ))
            }
        }
    }
    async fn pri_get_wavelength_idx(
        &self,
        req: Request<sepia2_rpc::PriRequest>,
    ) -> Result<Response<sepia2_rpc::Int32>, Status> {
        println!("Got a request for {}: {:?}", "PRI_GetWavelengthIdx", req);
        let req = req.into_inner();
        match PRI_GetWavelengthIdx(req.dev_idx, req.slot_id) {
            Ok(result) => Ok(Response::new(sepia2_rpc::Int32::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "PRI_GetWavelengthIdx", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_GetWavelengthIdx", e),
                ))
            }
        }
    }

    //// --- MapIdx calls ---

    async fn fwr_get_module_info_by_map_idx(
        &self,
        req: Request<sepia2_rpc::MapIdxRequest>,
    ) -> Result<Response<sepia2_rpc::ModuleInfo>, Status> {
        println!(
            "Got a request for {}: {:?}",
            "FWR_GetModuleInfoByMapIdx", req
        );
        let req = req.into_inner();
        match FWR_GetModuleInfoByMapIdx(req.dev_idx, req.map_idx) {
            Ok(result) => Ok(Response::new(sepia2_rpc::ModuleInfo::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "FWR_GetModuleInfoByMapIdx", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "FWR_GetModuleInfoByMapIdx", e),
                ))
            }
        }
    }

    async fn fwr_get_uptime_info_by_map_idx(
        &self,
        req: Request<sepia2_rpc::MapIdxRequest>,
    ) -> Result<Response<sepia2_rpc::UptimeInfo>, Status> {
        println!(
            "Got a request for {}: {:?}",
            "FWR_GetUptimeInfoByMapIdx", req
        );
        let req = req.into_inner();
        match FWR_GetUptimeInfoByMapIdx(req.dev_idx, req.map_idx) {
            Ok(result) => Ok(Response::new(sepia2_rpc::UptimeInfo::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "FWR_GetUptimeInfoByMapIdx", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "FWR_GetUptimeInfoByMapIdx", e),
                ))
            }
        }
    }

    //// --- TriggerSourceRequest ---

    async fn pri_decode_trigger_source(
        &self,
        req: Request<sepia2_rpc::TriggerSourceRequest>,
    ) -> Result<Response<sepia2_rpc::TriggerInfo>, Status> {
        println!("Got a request for {}: {:?}", "PRI_DecodeTriggerSource", req);
        let req = req.into_inner();
        let pri_req = match req.pri_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No PriRequest{dev_idx, slot_id} provided",
                ))
            }
        };
        match PRI_DecodeTriggerSource(pri_req.dev_idx, pri_req.slot_id, req.trg_src_idx) {
            Ok(result) => Ok(Response::new(sepia2_rpc::TriggerInfo::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "PRI_DecodeTriggerSource", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_DecodeTriggerSource", e),
                ))
            }
        }
    }
    async fn pri_set_trigger_source(
        &self,
        req: Request<sepia2_rpc::TriggerSourceRequest>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request for {}: {:?}", "PRI_SetTriggerSource", req);
        let req = req.into_inner();
        let pri_req = match req.pri_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No PriRequest{dev_idx, slot_id} provided",
                ))
            }
        };
        match PRI_SetTriggerSource(pri_req.dev_idx, pri_req.slot_id, req.trg_src_idx) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                error!("Calling {}: {}", "PRI_SetTriggerSource", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_SetTriggerSource", e),
                ))
            }
        }
    }

    //// --- WavelengthRequest ---

    async fn pri_decode_wavelength(
        &self,
        req: Request<sepia2_rpc::WavelengthRequest>,
    ) -> Result<Response<sepia2_rpc::Int32>, Status> {
        println!("Got a request for {}: {:?}", "PRI_DecodeWavelength", req);
        let req = req.into_inner();
        let pri_req = match req.pri_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No PriRequest{dev_idx, slot_id} provided",
                ))
            }
        };
        match PRI_DecodeWavelength(pri_req.dev_idx, pri_req.slot_id, req.wl_idx) {
            Ok(result) => Ok(Response::new(sepia2_rpc::Int32::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "PRI_DecodeWavelength", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_DecodeWavelength", e),
                ))
            }
        }
    }
    async fn pri_set_wavelength_idx(
        &self,
        req: Request<sepia2_rpc::WavelengthRequest>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request for {}: {:?}", "PRI_SetWavelengthIdx", req);
        let req = req.into_inner();
        let pri_req = match req.pri_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No PriRequest{dev_idx, slot_id} provided",
                ))
            }
        };
        match PRI_SetWavelengthIdx(pri_req.dev_idx, pri_req.slot_id, req.wl_idx) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                error!("Calling {}: {}", "PRI_SetWavelengthIdx", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_SetWavelengthIdx", e),
                ))
            }
        }
    }
    async fn pri_get_intensity(
        &self,
        req: Request<sepia2_rpc::WavelengthRequest>,
    ) -> Result<Response<sepia2_rpc::Uint32>, Status> {
        println!("Got a request for {}: {:?}", "PRI_GetIntensity", req);
        let req = req.into_inner();
        let pri_req = match req.pri_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No PriRequest{dev_idx, slot_id} provided",
                ))
            }
        };
        match PRI_GetIntensity(pri_req.dev_idx, pri_req.slot_id, req.wl_idx) {
            Ok(result) => Ok(Response::new(sepia2_rpc::Uint32::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "PRI_GetIntensity", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_GetIntensity", e),
                ))
            }
        }
    }

    //// --- OperationModeRequest ---

    async fn pri_decode_operation_mode(
        &self,
        req: Request<sepia2_rpc::OperationModeRequest>,
    ) -> Result<Response<sepia2_rpc::PrimaModeInfo>, Status> {
        println!("Got a request for {}: {:?}", "PRI_DecodeOperationMode", req);
        let req = req.into_inner();
        let pri_req = match req.pri_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No PriRequest{dev_idx, slot_id} provided",
                ))
            }
        };
        match PRI_DecodeOperationMode(pri_req.dev_idx, pri_req.slot_id, req.oper_mode_idx) {
            Ok(result) => Ok(Response::new(sepia2_rpc::PrimaModeInfo::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "PRI_DecodeOperationMode", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_DecodeOperationMode", e),
                ))
            }
        }
    }
    async fn pri_set_operation_mode(
        &self,
        req: Request<sepia2_rpc::OperationModeRequest>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request for {}: {:?}", "PRI_SetOperationMode", req);
        let req = req.into_inner();
        let pri_req = match req.pri_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No PriRequest{dev_idx, slot_id} provided",
                ))
            }
        };
        match PRI_SetOperationMode(pri_req.dev_idx, pri_req.slot_id, req.oper_mode_idx) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                error!("Calling {}: {}", "PRI_SetOperationMode", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_SetOperationMode", e),
                ))
            }
        }
    }

    // --- Other Requests

    async fn fwr_decode_err_phase_name(
        &self,
        req: Request<sepia2_rpc::ErrPhaseRequest>,
    ) -> Result<Response<sepia2_rpc::String>, Status> {
        println!("Got a request for {}: {:?}", "FWR_DecodeErrPhaseName", req);
        let req = req.into_inner();
        match FWR_DecodeErrPhaseName(req.err_phase) {
            Ok(result) => Ok(Response::new(sepia2_rpc::String::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "FWR_DecodeErrPhaseName", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "FWR_DecodeErrPhaseName", e),
                ))
            }
        }
    }
    async fn fwr_set_working_mode(
        &self,
        req: Request<sepia2_rpc::FwrSetWorkingModeRequest>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request for {}: {:?}", "FWR_SetWorkingMode", req);
        let req = req.into_inner();
        match FWR_SetWorkingMode(req.dev_idx, req.mode) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                error!("Calling {}: {}", "FWR_SetWorkingMode", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "FWR_SetWorkingMode", e),
                ))
            }
        }
    }
    async fn fwr_get_module_map(
        &self,
        req: Request<sepia2_rpc::GetModuleMapRequest>,
    ) -> Result<Response<sepia2_rpc::Int32>, Status> {
        println!("Got a request for {}: {:?}", "FWR_GetModuleMap", req);
        let req = req.into_inner();
        match FWR_GetModuleMap(req.dev_idx, req.perform_restart) {
            Ok(result) => Ok(Response::new(sepia2_rpc::Int32::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "FWR_GetModuleMap", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "FWR_GetModuleMap", e),
                ))
            }
        }
    }
    async fn fwr_create_support_request_text(
        &self,
        req: Request<sepia2_rpc::FwrRequestSupportRequest>,
    ) -> Result<Response<sepia2_rpc::String>, Status> {
        println!(
            "Got a request for {}: {:?}",
            "FWR_CreateSupportRequestText", req
        );
        let req = req.into_inner();
        let fwr_req = match req.fwr_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No FweRequestSupport{preamable, calling_sw, options, buffer} provided",
                ))
            }
        };
        match FWR_CreateSupportRequestText(req.dev_idx, fwr_req.into()) {
            Ok(result) => Ok(Response::new(sepia2_rpc::String::from(result))),
            Err(e) => {
                error!("Calling {}: {}", "FWR_CreateSupportRequestText", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "FWR_CreateSupportRequestText", e),
                ))
            }
        }
    }
    async fn pri_set_trigger_level(
        &self,
        req: Request<sepia2_rpc::TriggerLevelRequest>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request for {}: {:?}", "PRI_SetTriggerLevel", req);
        let req = req.into_inner();
        let pri_req = match req.pri_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No PriRequest{dev_idx, slot_id} provided",
                ))
            }
        };
        match PRI_SetTriggerLevel(pri_req.dev_idx, pri_req.slot_id, req.trg_lvl) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                error!("Calling {}: {}", "PRI_SetTriggerLevel", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_SetTriggerLevel", e),
                ))
            }
        }
    }
    async fn pri_set_frequency(
        &self,
        req: Request<sepia2_rpc::SetFrequencyRequest>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request for {}: {:?}", "PRI_SetFrequency", req);
        let req = req.into_inner();
        let pri_req = match req.pri_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No PriRequest{dev_idx, slot_id} provided",
                ))
            }
        };
        match PRI_SetFrequency(pri_req.dev_idx, pri_req.slot_id, req.frequency) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                error!("Calling {}: {}", "PRI_SetFrequency", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_SetFrequency", e),
                ))
            }
        }
    }
    async fn pri_set_gating_data(
        &self,
        req: Request<sepia2_rpc::GatingDataRequest>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request for {}: {:?}", "PRI_SetGatingData", req);
        let req = req.into_inner();
        let pri_req = match req.pri_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No PriRequest{dev_idx, slot_id} provided",
                ))
            }
        };
        let gating_data = match req.gating_data {
            Some(gating_data) => gating_data,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No GatingData{on_time, off_time} provided",
                ))
            }
        };
        match PRI_SetGatingData(
            pri_req.dev_idx,
            pri_req.slot_id,
            gating_data.on_time,
            gating_data.off_time_factor,
        ) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                error!("Calling {}: {}", "PRI_SetGatingData", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_SetGatingData", e),
                ))
            }
        }
    }
    async fn pri_set_gating_enabled(
        &self,
        req: Request<sepia2_rpc::GatingEnabledRequest>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request for {}: {:?}", "PRI_SetGatingEnabled", req);
        let req = req.into_inner();
        let pri_req = match req.pri_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No PriRequest{dev_idx, slot_id} provided",
                ))
            }
        };
        match PRI_SetGatingEnabled(pri_req.dev_idx, pri_req.slot_id, req.gating_enabled) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                error!("Calling {}: {}", "PRI_SetGatingEnabled", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_SetGatingEnabled", e),
                ))
            }
        }
    }
    async fn pri_set_gate_high_impedance(
        &self,
        req: Request<sepia2_rpc::HighImpedanceRequest>,
    ) -> Result<Response<()>, Status> {
        println!(
            "Got a request for {}: {:?}",
            "PRI_SetGateHighImpedance", req
        );
        let req = req.into_inner();
        let pri_req = match req.pri_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No PriRequest{dev_idx, slot_id} provided",
                ))
            }
        };
        match PRI_SetGateHighImpedance(pri_req.dev_idx, pri_req.slot_id, req.high_impedance) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                error!("Calling {}: {}", "PRI_SetGateHighImpedance", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_SetGateHighImpedance", e),
                ))
            }
        }
    }
    async fn pri_set_intensity(
        &self,
        req: Request<sepia2_rpc::SetIntensityRequest>,
    ) -> Result<Response<()>, Status> {
        println!("Got a request for {}: {:?}", "PRI_SetIntensity", req);
        let req = req.into_inner();
        let wl_req = match req.wl_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No WavelengthRequest{PriRequest, wl_idx} provided",
                ))
            }
        };
        let pri_req = match wl_req.pri_req {
            Some(pri_req) => pri_req,
            None => {
                return Err(Status::new(
                    tonic::Code::InvalidArgument,
                    "No PriRequest{dev_idx, slot_id} provided",
                ))
            }
        };
        match PRI_SetIntensity(
            pri_req.dev_idx,
            pri_req.slot_id,
            wl_req.wl_idx,
            req.intensity as u16,
        ) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => {
                error!("Calling {}: {}", "PRI_SetIntensity", e);
                Err(Status::new(
                    tonic::Code::Internal,
                    format!("Calling {}: {}", "PRI_SetIntensity", e),
                ))
            }
        }
    }
}
