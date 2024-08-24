use dijkstra::*;

fn main() {
    let mut graph = Graph::new();

    let book = Node::new("book");
    let lp = Node::new("LP");
    let poster = Node::new("Poster");
    let guitar = Node::new("Guitar");
    let drums = Node::new("Drums");
    let piano = Node::new("Piano");

    graph.add_edge(&book, &lp, 5);
    graph.add_edge(&book, &poster, 0);
    graph.add_edge(&lp, &guitar, 15);
    graph.add_edge(&lp, &drums, 20);
    graph.add_edge(&poster, &guitar, 30);
    graph.add_edge(&poster, &drums, 35);
    graph.add_edge(&drums, &piano, 10);
    graph.add_edge(&guitar, &piano, 20);

    println!("{:?}", graph.dijkstra(&book, &piano))
}
