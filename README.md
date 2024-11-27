# cargo_bench_training

# nightly 버젼으로 하는 bench
- https://github.com/chhetripradeep/cargo-bench-example


# cargo benches폴더로 하는 예시
- https://github.com/BurntSushi/rust-snappy
- https://bencher.dev/learn/benchmarking/rust/custom-harness/

# Criterion.rs Documentation
- https://bheisler.github.io/criterion.rs/book/getting_started.html

# cargo test --release
- https://stackoverflow.com/questions/29818084/can-tests-be-built-in-release-mode-using-cargo

- `Cargo.toml` 에 추가

```toml
[profile.test]
inherits = "release"
```

- `cargo test --release` exists, but it is slightly different than just enabling optimizations. For example, `debug assertions` become disabled.
  - You can also set opt-level in the `[profile.test]` section of your `Cargo.toml`, as Viktor Dahl suggests.

