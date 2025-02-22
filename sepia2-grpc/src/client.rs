#![allow(non_snake_case)]

use crate::sepia2_rpc::{self, sepia2_client::Sepia2Client};
use tonic::{transport::Channel, Request};
use sepia2::types::*;

// TODO: Implement Client as a library with C-FFI exposed interface to call from any language

// --------------- Async Client ----------------

pub struct Sepia2Service {
    pub client: Sepia2Client<Channel>,
}

impl Sepia2Service {
    pub async fn new(dst: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // NOTE: Move string to owned type because connect required parameter to outlive 'static
        let client = Sepia2Client::connect(dst.to_string()).await?;
        Ok(Self { client })
    }

    // ------------- RPC ----------------

    pub async fn LIB_DecodeError(
        &mut self,
        err_code: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::Error{ error: Some(err_code) });
        let resp = self.client.lib_decode_error(grpc_req).await?;
        Ok(resp.into_inner().err_str.into())
    }

    pub async fn LIB_GetVersion(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let resp = self.client.lib_get_version(()).await?;
        Ok(resp.into_inner().version.into())
    }

    pub async fn LIB_GetLibUSBVersion(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let resp = self.client.lib_get_lib_usb_version(()).await?;
        Ok(resp.into_inner().version.into())
    }

    pub async fn LIB_IsRunningOnWine(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let resp = self.client.lib_is_running_on_wine(()).await?;
        Ok(resp.into_inner().value.into())
    }

    pub async fn USB_OpenDevice(
        &mut self,
        dev_idx: i32,
    ) -> Result<UsbDevice, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
        let resp = self.client.usb_open_device(grpc_req).await?;
        Ok(resp.into_inner().into())
    }
    pub async fn USB_OpenGetSerNumAndClose(
        &mut self,
        dev_idx: i32,
    ) -> Result<UsbDevice, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
        let resp = self.client.usb_open_get_ser_num_and_close(grpc_req).await?;
        Ok(resp.into_inner().into())
    }
    pub async fn USB_GetStrDescriptor(
        &mut self,
        dev_idx: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
        let resp = self.client.usb_get_str_descriptor(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn USB_GetStrDescrByIdx(
        &mut self,
        dev_idx: i32,
        descr_idx: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::GetStrDescrByIdxRequest { dev_idx, descr_idx });
        let resp = self.client.usb_get_str_descr_by_idx(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn USB_IsOpenDevice(
        &mut self,
        dev_idx: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
        let resp = self.client.usb_is_open_device(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn USB_CloseDevice(
        &mut self,
        dev_idx: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
        let _resp = self.client.usb_close_device(grpc_req).await?;
        Ok(())
    }
    pub async fn FWR_DecodeErrPhaseName(
        &mut self,
        err_phase: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::ErrPhaseRequest { err_phase });
        let resp = self.client.fwr_decode_err_phase_name(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn FWR_GetVersion(
        &mut self,
        dev_idx: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
        let resp = self.client.fwr_get_version(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn FWR_GetLastError(
        &mut self,
        dev_idx: i32,
    ) -> Result<FwrError, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
        let resp = self.client.fwr_get_last_error(grpc_req).await?;
        Ok(resp.into_inner().into())
    }
    pub async fn FWR_GetWorkingMode(
        &mut self,
        dev_idx: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
        let resp = self.client.fwr_get_working_mode(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn FWR_SetWorkingMode(
        &mut self,
        dev_idx: i32,
        mode: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::FwrSetWorkingModeRequest { dev_idx, mode });
        let _resp = self.client.fwr_set_working_mode(grpc_req).await?;
        Ok(())
    }
    pub async fn FWR_RollBackToPermanentValues(
        &mut self,
        dev_idx: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
        let _resp = self.client.fwr_roll_back_to_permanent_values(grpc_req).await?;
        Ok(())
    }
    pub async fn FWR_StoreAsPermanentValues(
        &mut self,
        dev_idx: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
        let _resp = self.client.fwr_store_as_permanent_values(grpc_req).await?;
        Ok(())
    }
    pub async fn FWR_GetModuleMap(
        &mut self,
        dev_idx: i32,
        perform_restart: bool,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::GetModuleMapRequest {
            dev_idx,
            perform_restart,
        });
        let resp = self.client.fwr_get_module_map(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn FWR_GetModuleInfoByMapIdx(
        &mut self,
        dev_idx: i32,
        map_idx: i32,
    ) -> Result<ModuleInfo, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::MapIdxRequest { dev_idx, map_idx });
        let resp = self.client.fwr_get_module_info_by_map_idx(grpc_req).await?;
        Ok(resp.into_inner().into())
    }
    pub async fn FWR_GetUptimeInfoByMapIdx(
        &mut self,
        dev_idx: i32,
        map_idx: i32,
    ) -> Result<UptimeInfo, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::MapIdxRequest { dev_idx, map_idx });
        let resp = self.client.fwr_get_uptime_info_by_map_idx(grpc_req).await?;
        Ok(resp.into_inner().into())
    }
    pub async fn FWR_CreateSupportRequestText(
        &mut self,
        dev_idx: i32,
        fwr_req: FwrRequestSupport,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::FwrRequestSupportRequest {
            dev_idx,
            fwr_req: Some(fwr_req.into()),
        });
        let resp = self
            .client
            .fwr_create_support_request_text(grpc_req)
            .await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn FWR_FreeModuleMap(
        &mut self,
        dev_idx: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
        let _resp = self.client.fwr_free_module_map(grpc_req).await?;
        Ok(())
    }

    pub async fn PRI_GetDeviceInfo(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
    ) -> Result<PrimaDevInfo, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
        let resp = self.client.pri_get_device_info(grpc_req).await?;
        Ok(resp.into_inner().into())
    }
    pub async fn PRI_DecodeOperationMode(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
        oper_mode_idx: i32,
    ) -> Result<PrimaModeInfo, Box<dyn std::error::Error>> {
        let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
        let grpc_req = Request::new(sepia2_rpc::OperationModeRequest {
            pri_req,
            oper_mode_idx,
        });
        let resp = self.client.pri_decode_operation_mode(grpc_req).await?;
        Ok(resp.into_inner().into())
    }
    pub async fn PRI_GetOperationMode(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
        let resp = self.client.pri_get_operation_mode(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn PRI_SetOperationMode(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
        oper_mode_idx: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
        let grpc_req = Request::new(sepia2_rpc::OperationModeRequest {
            pri_req,
            oper_mode_idx,
        });
        let _resp = self.client.pri_set_operation_mode(grpc_req).await?;
        Ok(())
    }
    pub async fn PRI_DecodeTriggerSource(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
        trg_src_idx: i32,
    ) -> Result<TriggerInfo, Box<dyn std::error::Error>> {
        let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
        let grpc_req = Request::new(sepia2_rpc::TriggerSourceRequest {
            pri_req,
            trg_src_idx,
        });
        let resp = self.client.pri_decode_trigger_source(grpc_req).await?;
        Ok(resp.into_inner().into())
    }
    pub async fn PRI_GetTriggerSource(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
        let resp = self.client.pri_get_trigger_source(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn PRI_SetTriggerSource(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
        trg_src_idx: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
        let grpc_req = Request::new(sepia2_rpc::TriggerSourceRequest {
            pri_req,
            trg_src_idx,
        });
        let _resp = self.client.pri_set_trigger_source(grpc_req).await?;
        Ok(())
    }
    pub async fn PRI_GetTriggerLevelLimits(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
    ) -> Result<TriggerLevelInfo, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
        let resp = self.client.pri_get_trigger_level_limits(grpc_req).await?;
        Ok(resp.into_inner().into())
    }
    pub async fn PRI_GetTriggerLevel(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
        let resp = self.client.pri_get_trigger_level(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn PRI_SetTriggerLevel(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
        trg_lvl: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
        let grpc_req = Request::new(sepia2_rpc::TriggerLevelRequest { pri_req, trg_lvl });
        let _resp = self.client.pri_set_trigger_level(grpc_req).await?;
        Ok(())
    }
    pub async fn PRI_GetFrequencyLimits(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
    ) -> Result<(i32, i32), Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
        let resp = self.client.pri_get_frequency_limits(grpc_req).await?;
        let freq_limit_resp = resp.into_inner();
        Ok((freq_limit_resp.min_freq, freq_limit_resp.max_freq))
    }
    pub async fn PRI_GetFrequency(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
        let resp = self.client.pri_get_frequency(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn PRI_SetFrequency(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
        frequency: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
        let grpc_req = Request::new(sepia2_rpc::SetFrequencyRequest { pri_req, frequency });
        let _resp = self.client.pri_set_frequency(grpc_req).await?;
        Ok(())
    }
    pub async fn PRI_GetGatingLimits(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
    ) -> Result<PrimaGatingInfo, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
        let resp = self.client.pri_get_gating_limits(grpc_req).await?;
        Ok(resp.into_inner().into())
    }
    pub async fn PRI_GetGatingData(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
    ) -> Result<(i32, i32), Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
        let resp = self.client.pri_get_gating_data(grpc_req).await?;
        let gating_data = resp.into_inner();
        Ok((gating_data.on_time, gating_data.off_time_factor))
    }
    pub async fn PRI_SetGatingData(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
        on_time: i32,
        off_time_factor: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
        let gating_data = Some(sepia2_rpc::GatingData {
            on_time,
            off_time_factor,
        });
        let grpc_req = Request::new(sepia2_rpc::GatingDataRequest {
            pri_req,
            gating_data,
        });
        let _resp = self.client.pri_set_gating_data(grpc_req).await?;
        Ok(())
    }
    pub async fn PRI_GetGatingEnabled(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
        let resp = self.client.pri_get_gating_enabled(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn PRI_SetGatingEnabled(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
        gating_enabled: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
        let grpc_req = Request::new(sepia2_rpc::GatingEnabledRequest {
            pri_req,
            gating_enabled,
        });
        let _resp = self.client.pri_set_gating_enabled(grpc_req).await?;
        Ok(())
    }
    pub async fn PRI_GetGateHighImpedance(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
        let resp = self.client.pri_get_gate_high_impedance(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn PRI_SetGateHighImpedance(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
        high_impedance: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
        let grpc_req = Request::new(sepia2_rpc::HighImpedanceRequest {
            pri_req,
            high_impedance,
        });
        let _resp = self.client.pri_set_gate_high_impedance(grpc_req).await?;
        Ok(())
    }
    pub async fn PRI_DecodeWavelength(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
        wl_idx: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
        let grpc_req = Request::new(sepia2_rpc::WavelengthRequest { pri_req, wl_idx });
        let resp = self.client.pri_decode_wavelength(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn PRI_GetWavelengthIdx(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
        let resp = self.client.pri_get_wavelength_idx(grpc_req).await?;
        Ok(resp.into_inner().value.into())
    }
    pub async fn PRI_SetWavelengthIdx(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
        wl_idx: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
        let grpc_req = Request::new(sepia2_rpc::WavelengthRequest { pri_req, wl_idx });
        let _resp = self.client.pri_set_wavelength_idx(grpc_req).await?;
        Ok(())
    }
    pub async fn PRI_GetIntensity(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
        wl_idx: i32,
    ) -> Result<u16, Box<dyn std::error::Error>> {
        let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
        let grpc_req = Request::new(sepia2_rpc::WavelengthRequest { pri_req, wl_idx });
        let resp = self.client.pri_get_intensity(grpc_req).await?;
        Ok(resp.into_inner().value as u16)
    }
    pub async fn PRI_SetIntensity(
        &mut self,
        dev_idx: i32,
        slot_id: i32,
        wl_idx: i32,
        w_intensity: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pri_req = Some(sepia2_rpc::PriRequest { dev_idx, slot_id });
        let wl_req = Some(sepia2_rpc::WavelengthRequest { pri_req, wl_idx });
        let grpc_req = Request::new(sepia2_rpc::SetIntensityRequest {
            wl_req,
            intensity: w_intensity as i32,
        });
        let _resp = self.client.pri_set_intensity(grpc_req).await?;
        Ok(())
    }
}

// --------------- C-API ----------------
// TODO: Use a proc_macro like `#[tonic::async_trait]` to transform calls into sync,
// removing all `async` and wrap all Future calls into a tokio runtime with `block_on`

lazy_static! {
    CLIENT_SERVICE_CH = mpsc::Channel<ClientReq, oneshot::Channel>;
}

async fn client_service(addr: &str) -> Result<(), Box<dyn std::error::Error>> {

    /// Wait for MPSC to receive instructions about adress to connect to
    // TODO: Read MPSC channel for ClientCommand::Connect
    let mut client = Sepia2Client::connect(addr.into()).await?;

    loop {
        (req, resp_channel) = CLIENT_SERVICE.recv().expect("Something gone wrong in the MPSC transmission");
        /// For each command received query the server through gRPC and return the result through a
        /// oneshot channel
        println!("*** SIMPLE RPC ***");
        let response = match req {
            LibDecodeError{err_code}                                     => func(),
            LibGetVersion                                                => func(),
            LibGetLibUSBVersion                                          => func(),
            LibIsRunningOnWine                                           => func(),
            UsbOpenDevice{dev_idx}                                       => func(),
            UsbOpenGetSerNumAndClose{dev_idx}                            => func(),
            UsbGetStrDescriptor{dev_idx}                                 => func(),
            UsbGetStrDescrByIdx{dev_idx, descr_idx}                      => func(),
            UsbIsOpenDevice{dev_idx}                                     => func(),
            UsbCloseDevice{dev_idx}                                      => func(),
            FwrDecodeErrPhaseName{err_phase}                             => func(),
            FwrGetVersion{dev_idx}                                       => func(),
            FwrGetLastError{dev_idx}                                     => func(),
            FwrGetWorkingMode{dev_idx}                                   => func(),
            FwrSetWorkingMode{dev_idx, mode}                             => func(),
            FwrRollBackToPermanentValues{dev_idx}                        => func(),
            FwrStoreAsPermanentValues{dev_idx}                           => func(),
            FwrGetModuleMap{dev_idx, perform_restart}                    => func(),
            FwrGetModuleInfoByMapIdx{dev_idx, map_idx}                   => func(),
            FwrGetUptimeInfoByMapIdx{dev_idx, map_idx}                   => func(),
            FwrCreateSupportRequestText{dev_idx, fwr_req}                => func(),
            FwrFreeModuleMap{dev_idx}                                    => func(),
            PriGetDeviceInfo{dev_idx, slot_id}                           => func(),
            PriDecodeOperationMode{dev_idx, slot_id, oper_mode_idx}      => func(),
            PriGetOperationMode{dev_idx, slot_id}                        => func(),
            PriSetOperationMode{dev_idx, slot_id, oper_mode_idx}         => func(),
            PriDecodeTriggerSource{dev_idx, slot_id, trg_src_idx}        => func(),
            PriGetTriggerSource{dev_idx, slot_id}                        => func(),
            PriSetTriggerSource{dev_idx, slot_id, trg_src_idx}           => func(),
            PriGetTriggerLevelLimits{dev_idx, slot_id}                   => func(),
            PriGetTriggerLevel{dev_idx, slot_id}                         => func(),
            PriSetTriggerLevel{dev_idx, slot_id, trg_lvl}                => func(),
            PriGetFrequencyLimits{dev_idx, slot_id}                      => func(),
            PriGetFrequency{dev_idx, slot_id}                            => func(),
            PriSetFrequency{dev_idx, slot_id, frequency}                 => func(),
            PriGetGatingLimits{dev_idx, slot_id}                         => func(),
            PriGetGatingData{dev_idx, slot_id}                           => func(),
            PriSetGatingData{dev_idx, slot_id, on_time, off_time_factor} => func(),
            PriGetGatingEnabled{dev_idx, slot_id}                        => func(),
            PriSetGatingEnabled{dev_idx, slot_id, gating_enabled}        => func(),
            PriGetGateHighImpedance{dev_idx, slot_id}                    => func(),
            PriSetGateHighImpedance{dev_idx, slot_id, high_impedance}    => func(),
            PriDecodeWavelength{dev_idx, slot_id, wl_idx}                => func(),
            PriGetWavelengthIdx{dev_idx, slot_id}                        => func(),
            PriSetWavelengthIdx{dev_idx, slot_id, wl_idx}                => func(),
            PriGetIntensity{dev_idx, slot_id, wl_idx}                    => func(),
            PriSetIntensity{dev_idx, slot_id, wl_idx, w_intensity}       => func(),


        }.await?;
        let response = client
            .get_feature(Request::new(Point {
                latitude: 409_146_138,
                longitude: -746_188_906,
            }))
            .await?;
        resp_channel.send(response).expect("Something gone wrong in the oneshot transmission");
        println!("RESPONSE = {:?}", response);
    }
    Ok(())
}

pub extern "C" fn SpawnClient(addr: CString) {
    CLIENT_SERVICE = Sepia2Service::new(addr.into()).expect("Couldn't connect to gRPC server {addr}");
}

pub extern "C" fn LIB_DecodeError(
    err_code: i32,
    err_str: *mut c_char,
    api_err_str: *mut c_char,
) -> Result<String, Box<dyn std::error::Error>> {
    let resp_channel = oneshot::channel();
    CLIENT_SERVICE.send(ClientReq::LIB_DecodeError(err_code), resp_channel);
    let resp = resp_channel.receiv().expect("Something gone wrong in the oneshot tranmission");
    match resp {
        Ok(resp_inner) => {
            err_str = resp_inner.into()
            api_err_code = 0;
        },
        Err(e) => {
            err_str = "".into();
            api_err_code = e.into();
        }
    }
}

pub extern "C" fn LIB_GetVersion(&mut self) -> Result<String, Box<dyn std::error::Error>> {
    let resp = self.client.lib_get_version(()).await?;
    Ok(resp.into_inner().version.into())
}

pub extern "C" fn LIB_GetLibUSBVersion(&mut self) -> Result<String, Box<dyn std::error::Error>> {
    let resp = self.client.lib_get_lib_usb_version(()).await?;
    Ok(resp.into_inner().version.into())
}

pub extern "C" fn LIB_IsRunningOnWine(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
    let resp = self.client.lib_is_running_on_wine(()).await?;
    Ok(resp.into_inner().value.into())
}

pub extern "C" fn USB_OpenDevice(
    &mut self,
    dev_idx: i32,
) -> Result<UsbDevice, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
    let resp = self.client.usb_open_device(grpc_req).await?;
    Ok(resp.into_inner().into())
}
pub extern "C" fn USB_OpenGetSerNumAndClose(
    &mut self,
    dev_idx: i32,
) -> Result<UsbDevice, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
    let resp = self.client.usb_open_get_ser_num_and_close(grpc_req).await?;
    Ok(resp.into_inner().into())
}
pub extern "C" fn USB_GetStrDescriptor(
    &mut self,
    dev_idx: i32,
) -> Result<String, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
    let resp = self.client.usb_get_str_descriptor(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn USB_GetStrDescrByIdx(
    &mut self,
    dev_idx: i32,
    descr_idx: i32,
) -> Result<String, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::GetStrDescrByIdxRequest { dev_idx, descr_idx });
    let resp = self.client.usb_get_str_descr_by_idx(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn USB_IsOpenDevice(
    &mut self,
    dev_idx: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
    let resp = self.client.usb_is_open_device(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn USB_CloseDevice(
    &mut self,
    dev_idx: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
    let _resp = self.client.usb_close_device(grpc_req).await?;
    Ok(())
}
pub extern "C" fn FWR_DecodeErrPhaseName(
    &mut self,
    err_phase: i32,
) -> Result<String, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::ErrPhaseRequest { err_phase });
    let resp = self.client.fwr_decode_err_phase_name(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn FWR_GetVersion(
    &mut self,
    dev_idx: i32,
) -> Result<String, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
    let resp = self.client.fwr_get_version(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn FWR_GetLastError(
    &mut self,
    dev_idx: i32,
) -> Result<FwrError, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
    let resp = self.client.fwr_get_last_error(grpc_req).await?;
    Ok(resp.into_inner().into())
}
pub extern "C" fn FWR_GetWorkingMode(
    &mut self,
    dev_idx: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
    let resp = self.client.fwr_get_working_mode(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn FWR_SetWorkingMode(
    &mut self,
    dev_idx: i32,
    mode: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::FwrSetWorkingModeRequest { dev_idx, mode });
    let _resp = self.client.fwr_set_working_mode(grpc_req).await?;
    Ok(())
}
pub extern "C" fn FWR_RollBackToPermanentValues(
    &mut self,
    dev_idx: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
    let _resp = self.client.fwr_roll_back_to_permanent_values(grpc_req).await?;
    Ok(())
}
pub extern "C" fn FWR_StoreAsPermanentValues(
    &mut self,
    dev_idx: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
    let _resp = self.client.fwr_store_as_permanent_values(grpc_req).await?;
    Ok(())
}
pub extern "C" fn FWR_GetModuleMap(
    &mut self,
    dev_idx: i32,
    perform_restart: bool,
) -> Result<i32, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::GetModuleMapRequest {
        dev_idx,
        perform_restart,
    });
    let resp = self.client.fwr_get_module_map(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn FWR_GetModuleInfoByMapIdx(
    &mut self,
    dev_idx: i32,
    map_idx: i32,
) -> Result<ModuleInfo, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::MapIdxRequest { dev_idx, map_idx });
    let resp = self.client.fwr_get_module_info_by_map_idx(grpc_req).await?;
    Ok(resp.into_inner().into())
}
pub extern "C" fn FWR_GetUptimeInfoByMapIdx(
    &mut self,
    dev_idx: i32,
    map_idx: i32,
) -> Result<UptimeInfo, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::MapIdxRequest { dev_idx, map_idx });
    let resp = self.client.fwr_get_uptime_info_by_map_idx(grpc_req).await?;
    Ok(resp.into_inner().into())
}
pub extern "C" fn FWR_CreateSupportRequestText(
    &mut self,
    dev_idx: i32,
    fwr_req: FwrRequestSupport,
) -> Result<String, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::FwrRequestSupportRequest {
        dev_idx,
        fwr_req: Some(fwr_req.into()),
    });
    let resp = self
        .client
        .fwr_create_support_request_text(grpc_req)
        .await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn FWR_FreeModuleMap(
    &mut self,
    dev_idx: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::DeviceIdx { dev_idx });
    let _resp = self.client.fwr_free_module_map(grpc_req).await?;
    Ok(())
}

pub extern "C" fn PRI_GetDeviceInfo(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
) -> Result<PrimaDevInfo, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
    let resp = self.client.pri_get_device_info(grpc_req).await?;
    Ok(resp.into_inner().into())
}
pub extern "C" fn PRI_DecodeOperationMode(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
    oper_mode_idx: i32,
) -> Result<PrimaModeInfo, Box<dyn std::error::Error>> {
    let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
    let grpc_req = Request::new(sepia2_rpc::OperationModeRequest {
        pri_req,
        oper_mode_idx,
    });
    let resp = self.client.pri_decode_operation_mode(grpc_req).await?;
    Ok(resp.into_inner().into())
}
pub extern "C" fn PRI_GetOperationMode(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
    let resp = self.client.pri_get_operation_mode(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn PRI_SetOperationMode(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
    oper_mode_idx: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
    let grpc_req = Request::new(sepia2_rpc::OperationModeRequest {
        pri_req,
        oper_mode_idx,
    });
    let _resp = self.client.pri_set_operation_mode(grpc_req).await?;
    Ok(())
}
pub extern "C" fn PRI_DecodeTriggerSource(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
    trg_src_idx: i32,
) -> Result<TriggerInfo, Box<dyn std::error::Error>> {
    let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
    let grpc_req = Request::new(sepia2_rpc::TriggerSourceRequest {
        pri_req,
        trg_src_idx,
    });
    let resp = self.client.pri_decode_trigger_source(grpc_req).await?;
    Ok(resp.into_inner().into())
}
pub extern "C" fn PRI_GetTriggerSource(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
    let resp = self.client.pri_get_trigger_source(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn PRI_SetTriggerSource(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
    trg_src_idx: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
    let grpc_req = Request::new(sepia2_rpc::TriggerSourceRequest {
        pri_req,
        trg_src_idx,
    });
    let _resp = self.client.pri_set_trigger_source(grpc_req).await?;
    Ok(())
}
pub extern "C" fn PRI_GetTriggerLevelLimits(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
) -> Result<TriggerLevelInfo, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
    let resp = self.client.pri_get_trigger_level_limits(grpc_req).await?;
    Ok(resp.into_inner().into())
}
pub extern "C" fn PRI_GetTriggerLevel(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
    let resp = self.client.pri_get_trigger_level(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn PRI_SetTriggerLevel(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
    trg_lvl: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
    let grpc_req = Request::new(sepia2_rpc::TriggerLevelRequest { pri_req, trg_lvl });
    let _resp = self.client.pri_set_trigger_level(grpc_req).await?;
    Ok(())
}
pub extern "C" fn PRI_GetFrequencyLimits(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
    let resp = self.client.pri_get_frequency_limits(grpc_req).await?;
    let freq_limit_resp = resp.into_inner();
    Ok((freq_limit_resp.min_freq, freq_limit_resp.max_freq))
}
pub extern "C" fn PRI_GetFrequency(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
    let resp = self.client.pri_get_frequency(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn PRI_SetFrequency(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
    frequency: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
    let grpc_req = Request::new(sepia2_rpc::SetFrequencyRequest { pri_req, frequency });
    let _resp = self.client.pri_set_frequency(grpc_req).await?;
    Ok(())
}
pub extern "C" fn PRI_GetGatingLimits(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
) -> Result<PrimaGatingInfo, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
    let resp = self.client.pri_get_gating_limits(grpc_req).await?;
    Ok(resp.into_inner().into())
}
pub extern "C" fn PRI_GetGatingData(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
) -> Result<(i32, i32), Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
    let resp = self.client.pri_get_gating_data(grpc_req).await?;
    let gating_data = resp.into_inner();
    Ok((gating_data.on_time, gating_data.off_time_factor))
}
pub extern "C" fn PRI_SetGatingData(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
    on_time: i32,
    off_time_factor: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
    let gating_data = Some(sepia2_rpc::GatingData {
        on_time,
        off_time_factor,
    });
    let grpc_req = Request::new(sepia2_rpc::GatingDataRequest {
        pri_req,
        gating_data,
    });
    let _resp = self.client.pri_set_gating_data(grpc_req).await?;
    Ok(())
}
pub extern "C" fn PRI_GetGatingEnabled(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
    let resp = self.client.pri_get_gating_enabled(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn PRI_SetGatingEnabled(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
    gating_enabled: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
    let grpc_req = Request::new(sepia2_rpc::GatingEnabledRequest {
        pri_req,
        gating_enabled,
    });
    let _resp = self.client.pri_set_gating_enabled(grpc_req).await?;
    Ok(())
}
pub extern "C" fn PRI_GetGateHighImpedance(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
    let resp = self.client.pri_get_gate_high_impedance(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn PRI_SetGateHighImpedance(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
    high_impedance: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
    let grpc_req = Request::new(sepia2_rpc::HighImpedanceRequest {
        pri_req,
        high_impedance,
    });
    let _resp = self.client.pri_set_gate_high_impedance(grpc_req).await?;
    Ok(())
}
pub extern "C" fn PRI_DecodeWavelength(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
    wl_idx: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
    let grpc_req = Request::new(sepia2_rpc::WavelengthRequest { pri_req, wl_idx });
    let resp = self.client.pri_decode_wavelength(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn PRI_GetWavelengthIdx(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    let grpc_req = Request::new(sepia2_rpc::PriRequest { dev_idx, slot_id });
    let resp = self.client.pri_get_wavelength_idx(grpc_req).await?;
    Ok(resp.into_inner().value.into())
}
pub extern "C" fn PRI_SetWavelengthIdx(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
    wl_idx: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
    let grpc_req = Request::new(sepia2_rpc::WavelengthRequest { pri_req, wl_idx });
    let _resp = self.client.pri_set_wavelength_idx(grpc_req).await?;
    Ok(())
}
pub extern "C" fn PRI_GetIntensity(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
    wl_idx: i32,
) -> Result<u16, Box<dyn std::error::Error>> {
    let pri_req = sepia2_rpc::PriRequest { dev_idx, slot_id }.into();
    let grpc_req = Request::new(sepia2_rpc::WavelengthRequest { pri_req, wl_idx });
    let resp = self.client.pri_get_intensity(grpc_req).await?;
    Ok(resp.into_inner().value as u16)
}
pub extern "C" fn PRI_SetIntensity(
    &mut self,
    dev_idx: i32,
    slot_id: i32,
    wl_idx: i32,
    w_intensity: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let pri_req = Some(sepia2_rpc::PriRequest { dev_idx, slot_id });
    let wl_req = Some(sepia2_rpc::WavelengthRequest { pri_req, wl_idx });
    let grpc_req = Request::new(sepia2_rpc::SetIntensityRequest {
        wl_req,
        intensity: w_intensity as i32,
    });
    let _resp = self.client.pri_set_intensity(grpc_req).await?;
    Ok(())
}

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

macro_rules! return_type_of {
    LibDecodeError{ err_code: i32}                                                   => String,
    LibGetVersion                                                                    => String,
    LibGetLibUSBVersion                                                              => String,
    LibIsRunningOnWine                                                               => bool,
    UsbOpenDevice{dev_idx: i32}                                                      => UsbDevice,
    UsbOpenGetSerNumAndClose{dev_idx: i32}                                           => UsbDevice,
    UsbGetStrDescriptor{dev_idx: i32}                                                => String,
    UsbGetStrDescrByIdx{dev_idx: i32, descr_idx: i32}                                => String,
    UsbIsOpenDevice{dev_idx: i32}                                                    => bool,
    UsbCloseDevice{dev_idx: i32}                                                     => (),
    FwrDecodeErrPhaseName{err_phase: i32}                                            => String,
    FwrGetVersion{dev_idx: i32}                                                      => String,
    FwrGetLastError{dev_idx: i32}                                                    => FwrError,
    FwrGetWorkingMode{dev_idx: i32}                                                  => i32,
    FwrSetWorkingMode{dev_idx: i32, mode: i32}                                       => (),
    FwrRollBackToPermanentValues{dev_idx: i32}                                       => (),
    FwrStoreAsPermanentValues{dev_idx: i32}                                          => (),
    FwrGetModuleMap{dev_idx: i32, perform_restart: bool}                             => i32,
    FwrGetModuleInfoByMapIdx{dev_idx: i32, map_idx: i32}                             => ModuleInfo,
    FwrGetUptimeInfoByMapIdx{dev_idx: i32, map_idx: i32}                             => UptimeInfo,
    FwrCreateSupportRequestText{dev_idx: i32, fwr_req: FwrRequestSupport}            => String,
    FwrFreeModuleMap{dev_idx: i32}                                                   => (),
    PriGetDeviceInfo{dev_idx: i32, slot_id: i32}                                     => PrimaDevInfo,
    PriDecodeOperationMode{dev_idx: i32, slot_id: i32, oper_mode_idx: i32}           => PrimaModeInfo,
    PriGetOperationMode{dev_idx: i32, slot_id: i32}                                  => i32,
    PriSetOperationMode{dev_idx: i32, slot_id: i32, oper_mode_idx: i32}              => (),
    PriDecodeTriggerSource{dev_idx: i32, slot_id: i32, trg_src_idx: i32}             => TriggerInfo,
    PriGetTriggerSource{dev_idx: i32, slot_id: i32}                                  => i32,
    PriSetTriggerSource{dev_idx: i32, slot_id: i32, trg_src_idx: i32}                => (),
    PriGetTriggerLevelLimits{dev_idx: i32, slot_id: i32}                             => TriggerLevelInfo,
    PriGetTriggerLevel{dev_idx: i32, slot_id: i32}                                   => i32,
    PriSetTriggerLevel{dev_idx: i32, slot_id: i32, trg_lvl: i32}                     => (),
    PriGetFrequencyLimits{dev_idx: i32, slot_id: i32}                                => (i32, i32),
    PriGetFrequency{dev_idx: i32, slot_id: i32}                                      => i32,
    PriSetFrequency{dev_idx: i32, slot_id: i32, frequency: i32}                      => (),
    PriGetGatingLimits{dev_idx: i32, slot_id: i32}                                   => PrimaGatingInfo,
    PriGetGatingData{dev_idx: i32, slot_id: i32}                                     => (i32, i32),
    PriSetGatingData{dev_idx: i32, slot_id: i32, on_time: i32, off_time_factor: i32} => (),
    PriGetGatingEnabled{dev_idx: i32, slot_id: i32}                                  => bool,
    PriSetGatingEnabled{dev_idx: i32, slot_id: i32, gating_enabled: bool}            => (),
    PriGetGateHighImpedance{dev_idx: i32, slot_id: i32}                              => bool,
    PriSetGateHighImpedance{dev_idx: i32, slot_id: i32, high_impedance: bool}        => (),
    PriDecodeWavelength{dev_idx: i32, slot_id: i32, wl_idx: i32}                     => i32,
    PriGetWavelengthIdx{dev_idx: i32, slot_id: i32}                                  => i32,
    PriSetWavelengthIdx{dev_idx: i32, slot_id: i32, wl_idx: i32}                     => (),
    PriGetIntensity{dev_idx: i32, slot_id: i32, wl_idx: i32}                         => u16,
    PriSetIntensity{dev_idx: i32, slot_id: i32, wl_idx: i32, w_intensity: u16}       => (),
}
