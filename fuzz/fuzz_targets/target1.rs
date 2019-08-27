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
    graph.nodes.len() == 8 && 
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