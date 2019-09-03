# Fuzzcheck-rs example

This repo contains a very basic graph library. It also contains a `fuzz` folder
that was generated with `cargo fuzzcheck init`. That folder contains all the 
fuzz tests as well as additional code needed by the fuzz tester. For example,
the file `fuzz/src/lib.rs` defines a `GraphInputGenerator` type that defines 
how to fuzz inputs of type `Graph`. It also contains a fuzz test at 
`fuzz/fuzz_targets/target1.rs`.

To run the fuzz test, first install the `cargo fuzzcheck` tool by running:
```
cargo +nightly install --git https://github.com/loiclec/fuzzcheck-rs
```

Then set your version of rust to nightly and initialize the `fuzz` folder:
```
rustup override set nightly
cargo fuzzcheck init
```

And finally run the fuzz test called `target1` with:
```
cargo fuzzcheck run target1 fuzz
```
