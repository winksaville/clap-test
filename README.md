# clap-test

Minimal code to show name bug

In `-h` the name is `clap-test` but in `-V` it is `clap-test-name`:
```bash
$ cargo run -- -h
   Compiling clap-test v0.1.0 (/home/wink/data/prgs/clap/clap-test)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/clap-test -h`
Usage: clap-test

Options:
  -h, --help     Print help
  -V, --version  Print version
$ cargo run -- -V
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/clap-test -V`
clap-test-name 0.1.0
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
