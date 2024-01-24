# fast-aug - rust library

All commands relative to this directory with rust library (`fast_aug/`)


Run all rust tests
```shell
cargo test
```

Run rust benchmarks
```shell
cargo bench
# or, for specific benchmark
cargo bench --bench text
```

Run profiler against main binary  
Note: requires sudo to run
```shell
cargo install flamegraph
sudo cargo flamegraph --dev
```

Format code
```shell
cargo fmt
``` 


To measure compile time
```shell
cargo build --timings --profile=dev  # optimized and cached
cargo build --timings --profile=release
```
TBA: More commands from https://endler.dev/2020/rust-compile-times/
