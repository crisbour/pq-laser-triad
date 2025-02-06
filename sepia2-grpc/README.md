# Tonic (gRPC) service for Sepia2

- [ ] Configure feature for `sepia2-sys` to run for single-threaded or multi-threaded, based on how the static of the libloading is defined
    - simply lazy_static
    - lazy_static of wrapped `Arc<Mutex<Library>>`
- [ ] Define service call routines based on `.proto` definition
- [ ] Implement logging channel that client can subscribe to
- [ ] Implement error types for response messages
