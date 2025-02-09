use core::net::SocketAddr;
use log::error;
use named_pipe_stream::get_named_pipe_transport;
use sepia2::api::*;
use tonic::{transport::Server, Request, Response, Status};
use stringcase::snake_case;

mod named_pipe_stream;

pub mod sepia2_rpc {
    tonic::include_proto!("sepia2.rpc");
}

use sepia2_rpc::{
    sepia2_server::{Sepia2, Sepia2Server},
    DeviceIdx, LibDecodeErrorResponse,
};

// ---------------------------------------------------------
// Server runtime
// ---------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: This address is for test, it must be replaced with the address configured from ENV or
    // clac parsing
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();
    let sepia2_service = Sepia2Service::default();
    let svc = Sepia2Server::new(sepia2_service);

    println!("Starting gRPC server for Sepia2 Lib");

    //Server::builder().add_service(svc).serve(addr).await?;
    Server::builder()
        .add_service(svc)
        .serve(addr)
        //.serve_with_incoming(get_named_pipe_transport("prima-sepia2"))
        .await?;
    Ok(())
}

// ---------------------------------------------------------
// Server endpoints
// ---------------------------------------------------------

// TODO: Add here any state that we need to keep track of about the Laser
#[derive(Debug, Default)]
struct Sepia2Service;

macro_rules! shim_connector_basic {
    ($func_grpc:ident, $func_sepia2:ident, sepia2_rpc::Empty, $output:ty) => {
        async fn $func_grpc(&self, request: Request<sepia2_rpc::Empty>) -> Result<Response<sepia2_rpc::Version>, Status> {
            println!("Got a request for {}: {:?}", stringify!($func_sepia2), request);
            match $func_sepia2() {
                Ok(result) => Ok(Response::new( $output { version: result } )),
                Err(e) => {
                    error!("Calling {}: {}", stringify!($func_sepia2), e);
                    Err(Status::new(tonic::Code::Internal, format!("Calling {}: {}", stringify!($func_sepia2), e)))
                }
            }
        }
    };
    ($func_grpc:ident ,$func_sepia2:ident, $input:ty, $output:ty) => {
        async fn $func_grpc(&self, request: Request<$input>) -> Result<Response<$output>, Status> {
            println!("Got a request for {}: {:?}", stringify!($func_sepia2), request);
            match $func_sepia2(request.into_inner()) {
                Ok(result) => Ok(Response::new($output { result })),
                Err(e) => {
                    error!("Calling {}: {}", stringify!($func_sepia2), e);
                    Err(Status::new(tonic::Code::Internal, format!("Calling {}: {}", stringify!($func_sepia2), e)))
                }
            }
        }
    };
}

#[tonic::async_trait]
impl Sepia2 for Sepia2Service {
    async fn lib_decode_error(
        &self,
        request: Request<sepia2_rpc::Error>,
    ) -> Result<Response<LibDecodeErrorResponse>, Status> {
        println!("Got a request for LIB_DecodeError: {:?}", request);
        match request.into_inner().error {
            Some(error) => match LIB_DecodeError(error) {
                Ok(err_str) => Ok(Response::new(sepia2_rpc::LibDecodeErrorResponse { err_str })),
                Err(e) => {
                    error!("Calling LIB_DecodeError: {}", e);
                    Err(Status::new(tonic::Code::Internal, format!("Calling LIB_DecodeError: {}", e)))
                }
            },
            None => Err(Status::new(
                tonic::Code::InvalidArgument,
                "No error provided",
            )),
        }
    }

    shim_connector_basic!(lib_get_version, LIB_GetVersion, sepia2_rpc::Empty, sepia2_rpc::Version);
}
