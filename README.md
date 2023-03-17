# Stream EOF min. reproducer

Quick'n'dirty reproduction of [Rustls #1188][1188].

## Bug/Fix Repro

* Delete the `[patch]` section of the workspace `Cargo.toml`
* `cargo build`
* Run the reproducer that uses `Stream`:
  * `cargo run --bin mrp-notokio | grep "EOF"`
  * Note that no unexpected EOF is produced.
```bash
[nix-shell:~/Code/Rust/stream-eof-mrp]$ cargo run --bin mrp-notokio | grep "EOF"
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/mrp-notokio`
```
* Run the reproducer that uses `tokio-rustls`:
  * `cargo run --bin mrp-tokio | grep "EOF"`
  * Note that an unexpected EOF **does** occur: 
```bash
[nix-shell:~/Code/Rust/stream-eof-mrp]$ cargo run --bin mrp-tokio | grep "EOF"
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/mrp-tokio`
thread 'main' panicked at 'IO error: unexpected end of file', mrp-tokio/src/main.rs:21:29
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

* Restore the `patch` in the workspace `Cargo.toml` to use the patched Rustls dependency.
* `cargo build`
* Run the reproducer that uses `Stream`:
  * `cargo run --bin mrp-tokio | grep "EOF"`
  * Note that an unexpected EOF **does** occur:
```bash
[nix-shell:~/Code/Rust/stream-eof-mrp]$ cargo run --bin mrp-notokio | grep "EOF"
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/mrp-notokio`
thread 'main' panicked at 'IO error: unexpected end of file', mrp-notokio/src/main.rs:18:23
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Credits

Credit to [@vilgotf](https://github.com/vilgotf) for providing the original
reproducer code in [tokio-rs/tls#128][128].

[1188]: https://github.com/rustls/rustls/issues/1188
[128]: https://github.com/tokio-rs/tls/pull/128
