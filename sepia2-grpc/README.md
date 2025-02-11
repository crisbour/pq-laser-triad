# Tonic (gRPC) service for Sepia2

- [ ] Configure feature for `sepia2-sys` to run for single-threaded or multi-threaded, based on how the static of the libloading is defined
    - simply lazy_static
    - lazy_static of wrapped `Arc<Mutex<Library>>`
- [X] Define service call routines based on `.proto` definition
- [ ] Implement logging channel that client can subscribe to
- [X] Implement error types for response messages: Use protobuf::Status
- [ ] (Optional) Implement reflection to make it more accessible to control the
  laser

> [!WARN]
>
> At the moment the stub connection is composed of much code duplication becasue
> proc_macro expands before basic macro_rules, such that placing a
> `shim_connection!();` macro inside the impl wrapped in `#[tonic::async_trait]`
> acts after the proc_macro, missing the TokenStream mutation to
> `Send + Sync + 'static`

## Misc

Another method to convert to/from sepia2::types and protobuf types than use
`macro_rules!` is to use a `proc_macro` as follows, which generates the types
with derive proc_macro annotations at prebuild step.
```rust
#[proc_macro_derive(ConvertBetween, attributes(other_type))]
pub fn derive_convert_between(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let other_type = input.attrs.iter()
        .find(|attr| attr.path.is_ident("other_type"))
        .and_then(|attr| attr.parse_args::<syn::Type>().ok())
        .expect("Missing #[other_type] attribute");

    let expanded = quote! {
        impl From<#other_type> for #name {
            fn from(other: #other_type) -> Self {
                unsafe { std::mem::transmute(other) }
            }
        }

        impl From<#name> for #other_type {
            fn from(this: #name) -> Self {
                unsafe { std::mem::transmute(this) }
            }
        }
    };

    TokenStream::from(expanded)
}
```

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
