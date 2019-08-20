
extern crate fuzzcheck;

use fuzzcheck::input::*;

use rand::Rng;
use rand::rngs::ThreadRng;
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;

use serde::{Serialize, Deserialize};

use std::hash::{Hash, Hasher};

use crate::graph::*;

#[derive(Clone, Copy, Debug)]
pub enum GraphMutator {
    AddNode,
    RemoveNode,
    MutateNodeData,
    AddEdge,
    RemoveEdge,
    MoveEdge,
    AddFriend,
    MoveNode,
}

static MUTATORS: &[GraphMutator] = &[
    GraphMutator::AddNode,
    GraphMutator::RemoveNode,
    GraphMutator::MutateNodeData,
    GraphMutator::AddEdge,
    GraphMutator::RemoveEdge,
    GraphMutator::MoveEdge,
    GraphMutator::AddFriend,
    GraphMutator::MoveNode,
];
static WEIGHTS: &[usize] = &[5, 5, 25, 5, 5, 10, 10, 5];

pub struct GraphGenerator <G> 
    where G: InputGenerator, G::Input: Hash
{
    g: G,
    rng: ThreadRng,
    weighted_index: WeightedIndex<usize>,
}


impl<G> GraphGenerator<G>
    where G: InputGenerator, G::Input: Hash
{
   pub fn new(g: G) -> Self {
        Self {
            g,
            rng: rand::thread_rng(),
            weighted_index: WeightedIndex::new(WEIGHTS).unwrap(),
        }
    }

    fn mutate_with(
        &mut self,
        mutator: GraphMutator,
        input: &mut Graph<G::Input>,
        spare_cplx: f64
    ) -> bool {
        match mutator {
            GraphMutator::AddNode => {
                let data = self.g.new_input(spare_cplx);
                input.add_node(data);
                true
            },
            GraphMutator::RemoveNode => {
                let len = input.nodes.len();
                if len > 0 {
                    let pick = self.rng.gen_range(0, len);
                    let _ = input.remove_node(pick);
                    true
                } else {
                    false
                }
            },
            GraphMutator::MutateNodeData => {
                let len = input.nodes.len();
                if len > 0 {
                    let pick = self.rng.gen_range(0, len);
                    let node = input.get(pick);
                    self.g.mutate(&mut node.data, spare_cplx)
                } else {
                    false
                }
            },
            GraphMutator::AddEdge => {
                let len = input.nodes.len();
                if len > 0 {
                    let pick1 = self.rng.gen_range(0, len);
                    let pick2 = self.rng.gen_range(0, len);
                    let _ = input.add_edge(pick1, pick2);
                    true
                } else {
                    false
                }
            },
            GraphMutator::RemoveEdge => {
                let len = input.nodes.len();
                if len > 0 {
                    let pick = self.rng.gen_range(0, len);
                    let node = input.get(pick);
                    let len = node.edges.len();
                    if len > 0 {
                        let pick = self.rng.gen_range(0, len);
                        node.edges.remove(pick);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            GraphMutator::MoveEdge => {
                let nodes_len = input.nodes.len();
                if nodes_len > 0 {
                    let pick = self.rng.gen_range(0, nodes_len);
                    let node = input.get(pick);
                    let edges_len = node.edges.len();
                    if edges_len > 0 {
                        let pick1 = self.rng.gen_range(0, edges_len);
                        let pick2 = self.rng.gen_range(0, nodes_len);
                        node.edges[pick1] = pick2;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            GraphMutator::AddFriend => {
                let len = input.nodes.len();
                if len > 0 {
                    let pick = self.rng.gen_range(0, len);
                    let data = self.g.new_input(spare_cplx);
                    input.add_node(data);
                    input.add_edge(pick, input.nodes.len()-1);
                    true
                } else {
                    false
                }
            },
            GraphMutator::MoveNode => {
                let nodes_len = input.nodes.len();
                if nodes_len > 1 {
                    let pick1 = self.rng.gen_range(0, nodes_len);
                    let pick2 = self.rng.gen_range(0, nodes_len);
                    input.nodes.swap(pick1, pick2);
                    true
                } else {
                    false
                }
            },
        }
    }
}

impl<G> InputGenerator for GraphGenerator<G>
    where 
        G: InputGenerator, G::Input: Hash,
        G::Input: Serialize + for<'de> Deserialize<'de>
{
    type Input = Graph<G::Input>;

    fn hash<H>(input: &Self::Input, state: &mut H) where H: Hasher {
        input.hash(state);
    }

    fn complexity(input: &Self::Input) -> f64 {
        input.nodes.iter().fold(0.0, |c, n| c + 1.0 + (n.edges.len() as f64))
    }

    fn base_input() -> Self::Input {
        Graph::new()
    }

    fn new_input(&mut self, max_cplx: f64) -> Self::Input {
        let mut g = Graph::new();
        let target_cplx = self.rng.gen_range(0.0, max_cplx);
        let mut current_cplx = Self::complexity(&g);
        while current_cplx < target_cplx {
            let mutator = if self.rng.gen_bool(0.5) { GraphMutator::AddNode } else { GraphMutator::AddEdge };
            self.mutate_with(mutator, &mut g, target_cplx - current_cplx);
            current_cplx = Self::complexity(&g);
        }
        while current_cplx > target_cplx {
            let mutator = if self.rng.gen_bool(0.5) { GraphMutator::RemoveNode } else { GraphMutator::RemoveEdge };
            self.mutate_with(mutator, &mut g, target_cplx - current_cplx);
            current_cplx = Self::complexity(&g);
        }
        g
    }

    fn mutate(&mut self, input: &mut Self::Input, spare_cplx: f64) -> bool {
        for _ in 0..MUTATORS.len() {
            let pick = self.weighted_index.sample(&mut self.rng);
            if self.mutate_with(MUTATORS[pick], input, spare_cplx) {
                return true;
            }
        }
        false
    }
    fn from_data(data: &Vec<u8>) -> Option<Self::Input> {
        serde_json::from_slice(data).ok()
    }
    fn to_data(input: &Self::Input) -> Vec<u8> {
        serde_json::to_vec_pretty(input).unwrap()
    }
}
