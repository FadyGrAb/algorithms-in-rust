use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Display;
use std::hash::Hash;

/// ## Edge struct
/// It contains the *Node* and "cost" for this node. The *Ord* trait is implemented for this struct
/// to return the Edge with the smallest cost for use in the *BinaryHeap*.
#[derive(PartialEq, Eq, Debug)]
pub struct Edge {
    pub node: Node,
    pub cost: u32,
}

impl Edge {
    pub fn new(node: Node, cost: u32) -> Self {
        Self {
            node: node.clone(),
            cost,
        }
    }
}
impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|__ ({})__> {}", self.cost, self.node)
    }
}
impl Ord for Edge {
    /// Reverse the Ordering. Other to self not self to other
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

/// ## Node struct
/// The graph Nodes. Only the node name is needed. Cost is added when creating Edges.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    pub name: String,
}
impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write! {f, "[{}]", self.name}
    }
}
impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.name.as_bytes())
    }
}

/// ## Graph struct
/// Contains the relations between the nodes.
/// ### Examples
/// ```
/// use dijkstra::{Graph, Node};
///
/// let a = Node::new("a");
/// let b = Node::new("b");
/// let c = Node::new("c");
///
/// let mut graph = Graph::new();
///
/// graph.add_edge(&a, &b, 10);
/// graph.add_edge(&a, &c, 5);
///
/// // Node a has 2 neighbors
/// assert_eq!(graph.get_neighbors(&a).unwrap().len(), 2);
///
/// // The Edge a -> b has cost of 10
/// assert_eq!(graph.get_neighbors(&a).unwrap()[0].cost, 10);
///
/// // The Edge a -> c has cost of 5
/// assert_eq!(graph.get_neighbors(&a).unwrap()[1].cost, 5);
///
/// println!("{graph}")
/// ```
pub struct Graph {
    pub neighbors: HashMap<Node, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            neighbors: HashMap::new(),
        }
    }

    pub fn add_edge<'b>(&mut self, from: &Node, to: &Node, cost: u32) {
        self.neighbors
            .entry(from.clone())
            .or_insert_with(Vec::new)
            .push(Edge::new(to.clone(), cost))
    }

    pub fn get_neighbors(&self, node: &Node) -> Option<&Vec<Edge>> {
        self.neighbors.get(node)
    }

    pub fn dijkstra(&self, start: &Node, end: &Node) -> Vec<Node> {
        let mut costs = HashMap::new();
        let mut parents: HashMap<&Node, Option<Node>> = HashMap::new();
        let mut processed: Vec<Node> = vec![];
        let mut buffer_heap = BinaryHeap::new();

        costs.insert(start.clone(), 0_u32);
        parents.insert(start, None);
        buffer_heap.push(Edge::new(start.clone(), 0));

        while let Some(Edge { node, cost }) = buffer_heap.pop() {
            if processed.contains(&node) {
                continue;
            }

            processed.push(node.clone());

            if let Some(neighbors) = self.get_neighbors(&node) {
                for neighbor in neighbors {
                    let new_cost = cost + neighbor.cost;

                    if new_cost < *costs.get(&neighbor.node).unwrap_or(&u32::MAX) {
                        costs.insert(node.clone(), new_cost);
                        parents.insert(&neighbor.node, Some(node.clone()));
                        buffer_heap.push(Edge::new(neighbor.node.clone(), new_cost))
                    }
                }
            }
        }

        Self::get_best_path(&parents, end)
    }

    fn get_best_path(parents: &HashMap<&Node, Option<Node>>, end: &Node) -> Vec<Node> {
        let mut path = vec![];
        let mut current_node = Some(end.clone());

        while let Some(ref n) = current_node {
            path.push(n.clone());
            current_node = parents.get(n).unwrap_or(&None).clone();
        }

        path.reverse();

        path
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for node in &self.neighbors {
            result.push_str(&format!("\nNode: {}", node.0));
            for edge in node.1 {
                result.push_str(&format!("\n\t{edge}"))
            }
        }
        write!(f, "{result}")
    }
}
