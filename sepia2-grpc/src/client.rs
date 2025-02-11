use sepia2_rpc::Sepia2Client;

// TODO: Implement Client as a library with C-FFI exposed interface to call from any language
// WIP ...

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Sepia2Client::connect("http://[::1]:10000").await?;

    println!("*** SIMPLE RPC ***");
    let response = client
        .get_feature(Request::new(Point {
            latitude: 409_146_138,
            longitude: -746_188_906,
        }))
        .await?;
    println!("RESPONSE = {:?}", response);

    println!("\n*** SERVER STREAMING ***");
    print_features(&mut client).await?;

    println!("\n*** CLIENT STREAMING ***");
    run_record_route(&mut client).await?;

    println!("\n*** BIDIRECTIONAL STREAMING ***");
    run_route_chat(&mut client).await?;

    Ok(())
}
