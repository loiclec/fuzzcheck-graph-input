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

## Content of fuzz test 

```rust
extern crate graph;
use graph::Graph;

extern crate graph_fuzz;
use graph_fuzz::GraphGenerator;

extern crate fuzzcheck;
use fuzzcheck::fuzzer;

extern crate fuzzcheck_input;
use fuzzcheck_input::integer::IntegerGenerator;
use fuzzcheck_input::vector::VectorGenerator;

fn test(graph: &Graph<i8>) -> bool {
    if 
    graph.nodes.len() >= 8 && 
    graph.nodes[0].data == 63 && 
    graph.nodes[1].data == 3 &&
    graph.nodes[2].data == -56 &&
    graph.nodes[3].data == 100 &&
    graph.nodes[4].data == -100 &&
    graph.nodes[5].data == -78 &&
    graph.nodes[6].data == 46 &&
    graph.nodes[7].data == 120 &&
    
    graph.nodes[0].edges.len() == 2 && 
    graph.nodes[0].edges[0] == 1 && 
    graph.nodes[0].edges[1] == 2 && 
    graph.nodes[1].edges.len() == 2 && 
    graph.nodes[1].edges[0] == 3 && 
    graph.nodes[1].edges[1] == 4 && 
    graph.nodes[2].edges.len() == 2 && 
    graph.nodes[2].edges[0] == 5 && 
    graph.nodes[2].edges[1] == 6 && 
    graph.nodes[3].edges.len() == 1 && 
    graph.nodes[3].edges[0] == 7 && 
    graph.nodes[4].edges.len() == 0 && 
    graph.nodes[5].edges.len() == 0 && 
    graph.nodes[6].edges.len() == 0 && 
    graph.nodes[7].edges.len() == 0 {
        return false
    }
    true
}

fn main() {
    let i8_gen = IntegerGenerator::<i8>::default();
    let graph_gen = GraphGenerator::new(i8_gen);

    let _ = fuzzer::launch(test, graph_gen);
}
```