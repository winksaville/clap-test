# clap-test

Test using the latest version of `clap` in a project, in particular
you now should use `command` and `arg` instead of `clap` in the attributes.

Also, about is sourced from Cargo.toml description or a `/// Doc comment` above the `struct Args`

Example usage:

Note Bug: in `cargo run` and `cargo run -- -V` the name is clap-test-name from the command attribute,
but in `cargo run -- -h` the name is clap-test from Cargo.toml,

```bash
wink@3900x 25-08-10T05:01:19.332Z:~/data/prgs/clap/clap-test (main)
$ cargo run
   Compiling clap-test v0.1.0 (/home/wink/data/prgs/clap/clap-test)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/clap-test`
name = "clap-test-name"
about = "about in command attribute, highest priority"
version = 0.0.0-xxx
length = 1.00
wink@3900x 25-08-10T05:02:59.044Z:~/data/prgs/clap/clap-test (main)
$ cargo run -- -V
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/clap-test -V`
clap-test-name 0.0.0-xxx
wink@3900x 25-08-10T05:03:12.085Z:~/data/prgs/clap/clap-test (main)
$ cargo run -- -h
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/clap-test -h`
about in command attribute, highest priority

Usage: clap-test [OPTIONS]

Options:
  -l, --length <LENGTH>  [default: 1]
  -h, --help             Print help
  -V, --version          Print version
wink@3900x 25-08-10T05:03:15.204Z:~/data/prgs/clap/clap-test (main)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
