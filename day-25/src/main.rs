use std::collections::{HashMap, VecDeque};

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let graph = parse(input);
        find_answer(graph)
    });
    two(|_| "Big Red Button has been pushed!");
}

struct Node {
    edges: Vec<EdgeIdx>,
    gen: u64,
}

#[derive(Clone, Copy)]
struct EdgeIdx {
    idx: usize,
    endpt: usize,
}

#[derive(PartialEq, Debug)]
struct Edge {
    endpts: [usize; 2],
    used: bool,
}

struct Graph<'a> {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    gen: u64,
    names: HashMap<&'a str, usize>,
}

impl<'a> Graph<'a> {
    fn new_node(&mut self) -> usize {
        let id = self.nodes.len();

        self.nodes.push(Node {
            edges: Vec::new(),
            gen: 1,
        });

        id
    }

    fn new_edge(&mut self, from: usize, to: usize) {
        let [from, to] = if from < to { [from, to] } else { [to, from] };
        let edge = Edge {
            endpts: [from, to],
            used: false,
        };

        if self.edges.contains(&edge) {
            return;
        }

        let idx = self.edges.len();
        self.edges.push(edge);
        self.nodes[from].edges.push(EdgeIdx { idx, endpt: 0 });
        self.nodes[to].edges.push(EdgeIdx { idx, endpt: 1 });
    }

    fn traverse_edge(&mut self, edge: EdgeIdx) -> Option<usize> {
        let to = self.edges[edge.idx].endpts[1 - edge.endpt];

        if self.nodes[to].gen == self.gen {
            return None;
        }

        self.nodes[to].gen = self.gen;
        Some(to)
    }

    fn resolve(&mut self, name: &'a str) -> usize {
        if let Some(id) = self.names.get(name) {
            *id
        } else {
            let id = self.new_node();
            self.names.insert(name, id);

            id
        }
    }
}

fn parse(input: &str) -> Graph {
    let mut parser = utils::Parser::new(input);
    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
        gen: 0,
        names: HashMap::new(),
    };

    parser.sep_by("\n", |parser| {
        let from = graph.resolve(parser.ident());
        parser.expect(": ");

        parser.sep_by(" ", |parser| {
            let to = graph.resolve(parser.ident());
            graph.new_edge(from, to);
        });
    });

    graph
}

fn find_answer(mut graph: Graph) -> usize {
    let from = farthest_from(0, &mut graph);
    let find = farthest_from(from, &mut graph);

    for _ in 0..3 {
        let count = find_region_count(from, find, &mut graph);
        assert_eq!(count, None);
    }

    let a = find_region_count(from, find, &mut graph).unwrap();
    let b = graph.nodes.len() - a;

    a * b
}

fn farthest_from(from: usize, graph: &mut Graph) -> usize {
    let mut queue = VecDeque::from([from]);
    let mut farthest = 0;

    graph.gen += 1;
    graph.nodes[from].gen = graph.gen;

    while let Some(node) = queue.pop_front() {
        farthest = node;

        for edge in 0..graph.nodes[node].edges.len() {
            let edge = graph.nodes[node].edges[edge];
            let Some(to) = graph.traverse_edge(edge) else {
                continue;
            };

            queue.push_back(to);
        }
    }

    farthest
}

fn find_region_count(from: usize, find: usize, graph: &mut Graph) -> Option<usize> {
    let mut queue = VecDeque::from([(from, None)]);
    let mut paths = Vec::new();
    let mut count = 0;

    graph.gen += 1;
    graph.nodes[from].gen = graph.gen;

    while let Some((node, path)) = queue.pop_front() {
        if node == find {
            let mut path = path;

            while let Some(curr) = path {
                let (edge, rest): (usize, _) = paths[curr];
                graph.edges[edge].used = true;
                path = rest;
            }

            return None;
        }

        count += 1;

        for edge in 0..graph.nodes[node].edges.len() {
            let edge = graph.nodes[node].edges[edge];

            if graph.edges[edge.idx].used {
                continue;
            }

            let Some(to) = graph.traverse_edge(edge) else {
                continue;
            };

            let new_path = paths.len();
            paths.push((edge.idx, path));
            queue.push_back((to, Some(new_path)));
        }
    }

    Some(count)
}
