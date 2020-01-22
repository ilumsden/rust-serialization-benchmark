# About

This repo benchmarks various Rust serialization libraries based on `serde` (see list below). The actual test that each library undergoes was taken from https://github.com/thekvs/cpp-serializers. It essentially consists of three parts:
1. Create a record containing 1000 64-bit integers and 100 85-character strings (the actual data that is stored in the record can be found in `src/data.rs`).
2. Serialize and deserialize the record, and check if the deserialized version matches the original (ensures no loss or corruption of data).
3. Serialize and deserialize the record repeatedly (the number of iterations is user specified). This is the part of the test that is timed for performance.

The libraries tested are:
* `serde_json`
* `serde_cbor`
* `rmp-serde`
* `toml`
* `serde-pickle`
* `ron`
* `avro-rs`

Exact version information and other packages required to run this benchmark can be found in the `Cargo.toml` file.

# Build
Since this is a performance benchmark, it is highly recommended that you compile this code in release mode with
```Rust
cargo build --release
```
Once built, the code will be found in `target/release` if you ran in release mode (`/target/debug` otherwise).
