
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
cargo install cargo-flamegraph
sudo cargo flamegraph --dev
```


TBA: More commands from https://endler.dev/2020/rust-compile-times/