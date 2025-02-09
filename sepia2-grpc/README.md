# Tonic (gRPC) service for Sepia2

- [ ] Configure feature for `sepia2-sys` to run for single-threaded or multi-threaded, based on how the static of the libloading is defined
    - simply lazy_static
    - lazy_static of wrapped `Arc<Mutex<Library>>`
- [ ] Define service call routines based on `.proto` definition
- [ ] Implement logging channel that client can subscribe to
- [ ] Implement error types for response messages


## Resources

- [gRPC Status Codes](https://grpc.io/docs/guides/status-codes/) remote error handling 
- [build gRPC APIs with
Rust](https://konghq.com/blog/engineering/building-grpc-apis-with-rust)
    - [Insomnia testing gRPC](https://insomnia.rest/)
- [gRPC Status details](https://ericb.xyz/posts/rust-tonic-grpc-errors/)
incorporate details for error codes and messages, i.e. useful for stack
unwrapping
- [Rust Macros](https://doc.rust-lang.org/reference/macros-by-example.html)
Macros documentation used to develop shim connection macros `proto API` <->
`sepia2 API`
