use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Node<T> {
    pub data: T,
    pub edges: Vec<usize>
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            edges: Vec::new()
        }
    }
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new()
        }
    }

    pub fn get(&mut self, idx: usize) -> &mut Node<T> {
        &mut self.nodes[idx]
    }

    pub fn add_node(&mut self, data: T) {
        self.nodes.push(Node::new(data));
    }

    pub fn remove_node(&mut self, idx: usize) -> bool {
        if idx < self.nodes.len() {
            self.nodes.remove(idx);
            for n in self.nodes.iter_mut() {
                if let Some(position) = n.edges.iter().position(|&x| x == idx) {
                    n.edges.remove(position);
                }
            }
            true
        } else {
            false
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> bool {
        if from < self.nodes.len() && to < self.nodes.len() {
            self.nodes[from].edges.push(to);
            true
        } else {
            false
        }
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) -> bool {
        if from < self.nodes.len() && to < self.nodes.len() {
            let from_node = self.get(from);
            if let Some(to_node_index) = from_node.edges.iter().position(|&x| x == to) {
                let to_node = from_node.edges[to_node_index];
                from_node.edges.remove(to_node_index);
                self.remove_node(to_node);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

