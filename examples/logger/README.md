## Usage

```shell
cargo build
make -C hello
DYLD_INSERT_LIBRARIES=./target/debug/liblogger.dylib RUST_LOG=info ./hello/hello
```
or on Linux:
```shell
LD_PRELOAD=./target/debug/liblogger.so RUST_LOG=info ./hello/hello
```

The default log location is `\/tmp/shimmer.log`, it can be changed by modifying
`src/log4rs.yml`. For more details about formatting see: https://docs.rs/log4rs/latest/log4rs/index.html
