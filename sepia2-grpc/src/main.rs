mod server;
mod named_pipe_stream;

use server::{Sepia2Server, Sepia2Service};
use core::net::SocketAddr;

// ---------------------------------------------------------
// Server runtime
// ---------------------------------------------------------

struct Args {
    /// Choose TCP Socket as transport type and pick port to listen on
    #[arg(short, long, default_value_t = Some("[::1]:50051"))]
    tcp_socket: Option<String>,

    /// Choose named pipes as trasnport and pick named for the pipe
    #[arg(short, long, default_value_t = None)]
    pipename: Option<String>,

    /// Set port forwarding on the given machine
    #[arg(short, long, default_value_t = false)]
    forwarding: bool,

    /// Force system type, otherwise it is detected automatically
    #[arg(short, long, default_value_t = None)]
    system: Option<String>,

    /// Is the system running on wine? Consider how to setup port forwarding on the Linux host
    #[arg(short, long, default_value_t = false)]
    wine: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    assert!(args.tcp_socket.is_some() ^ args.pipename.is_some(), "Expecting mutually exclusive tcp_socket or pipename");

    // Setup port forwarding
    if args.forwarding {
        todo!("Setup port forwarding");
        if args.wine {
            todo!("Setup port forwarding on the linux host");
        }
    }

    // Setup Sepia Service
    let sepia2_service = Sepia2Service::default();
    let svc = Sepia2Server::new(sepia2_service);

    info!("Starting gRPC server for Sepia2 Lib");

    if let Some(addr) = args.tcp_socket {
        let addr: SocketAddr = addr.parse().unwrap();
        info!("Listening on {}", addr);
        Server::builder()
            .add_service(svc)
            .serve(addr)
            .await?;
    } else if let Some(pipename) = args.pipename {
        Server::builder()
            .add_service(svc)
            .serve_with_incoming(get_named_pipe_transport(pipename))
            .await?;
    } else {
        return Err("No transport selected".into());
    }

    Ok(())
}
