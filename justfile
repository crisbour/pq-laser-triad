
# Build the Windows target release with server-bin features
build:
    cd sepia2-grpc && cargo build --release --target x86_64-pc-windows-gnu --bin sepia2-grpc-server --features server-bin
