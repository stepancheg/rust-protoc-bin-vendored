<!-- cargo-sync-readme start -->

`protoc` binary downloaded and stored inside the crate.

Can be used to avoid downloading and installing `protoc` binary.

# Example

```rust
protoc_bin_vendored::protoc_bin_path().unwrap()
```

returns a path to a `protoc` binary packaged into the crate.

Crate also packs `.proto` files distributed with protobuf:

```rust
protoc_bin_vendored::include_path().unwrap()
```

<!-- cargo-sync-readme end -->
