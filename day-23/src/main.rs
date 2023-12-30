use std::collections::HashMap;
use utils::grid::*;

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let grid = parse(input);
        let graph = make_graph(grid, false);
        hike(&graph)
    });
    two(|input| {
        let grid = parse(input);
        let graph = make_graph(grid, true);
        hike(&graph)
    });
}

#[derive(Clone, Copy)]
enum Tile {
    Path,
    Forest,
    Dir((isize, isize)),
}

struct Graph {
    nodes: Vec<Node>,
    start: usize,
    dest: usize,
}

impl Graph {
    fn new_node(&mut self) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Node { to: Vec::new() });

        id
    }
}

struct Node {
    to: Vec<(usize, u64)>,
}

fn parse(input: &str) -> Grid<(Tile, Option<usize>)> {
    let mut parser = utils::Parser::new(input);

    parser.grid(|char| {
        let tile = match char {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '^' => Tile::Dir((-1, 0)),
            'v' => Tile::Dir((1, 0)),
            '<' => Tile::Dir((0, -1)),
            '>' => Tile::Dir((0, 1)),
            _ => panic!(),
        };

        (tile, None)
    })
}

fn make_graph(mut grid: Grid<(Tile, Option<usize>)>, nondirected: bool) -> Graph {
    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut graph = Graph {
        nodes: Vec::new(),
        start: 0,
        dest: 0,
    };

    let start = grid.pos(0, 1).unwrap();
    let dest = grid
        .pos(grid.tiles.len() - 1, grid.tiles[0].len() - 2)
        .unwrap();

    graph.start = graph.new_node();
    grid.get_mut(start).1 = Some(graph.start);

    let mut queue = vec![(start, (1, 0), graph.start)];
    let mut buffer = Vec::new();

    'outer: while let Some((mut pos, mut dir, id)) = queue.pop() {
        let mut weight = 0;
        let mut traversible_from_to = true;
        let mut traversible_to_from = true;

        let (from, to) = loop {
            let Some(new_pos) = grid.shift(pos, dir) else {
                continue 'outer;
            };

            pos = new_pos;
            weight += 1;
            buffer.clear();

            let (tile, seen) = grid.get_mut(pos);

            if let Tile::Dir(slope) = *tile {
                traversible_from_to &= dir == slope;
                traversible_to_from &= dir == (-slope.0, -slope.1);

                if !traversible_from_to && !traversible_to_from {
                    continue 'outer;
                }
            }

            if let Some(to) = *seen {
                break (id, to);
            }

            if pos == dest {
                graph.dest = graph.new_node();
                grid.get_mut(pos).1 = Some(graph.dest);

                break (id, graph.dest);
            }

            for adj in DIRS {
                if (-adj.0, -adj.1) == dir {
                    continue;
                }

                let Some(pos) = grid.shift(pos, adj) else {
                    continue;
                };

                match grid.get(pos).0 {
                    Tile::Path => buffer.push(adj),
                    Tile::Forest => continue,
                    Tile::Dir(_) => buffer.push(adj),
                }
            }

            match buffer.len() {
                0 => continue 'outer,
                1 => dir = buffer.pop().unwrap(),
                _ => {
                    let to = graph.new_node();
                    grid.get_mut(pos).1 = Some(to);

                    for dir in buffer.drain(..) {
                        queue.push((pos, dir, to));
                    }

                    break (id, to);
                }
            }
        };

        if nondirected || traversible_from_to {
            graph.nodes[from].to.push((to, weight));
        }

        if nondirected || traversible_to_from {
            graph.nodes[to].to.push((from, weight));
        }
    }

    graph
}

fn hike(graph: &Graph) -> u64 {
    // Bitflags assume <64 nodes
    assert!(graph.nodes.len() < 64);

    let seen = 1 << graph.start;
    let mut cache = HashMap::new();
    recurse(graph.start, seen, graph, &mut cache)
}

fn recurse(curr: usize, seen: u64, graph: &Graph, cache: &mut HashMap<(usize, u64), u64>) -> u64 {
    if curr == graph.dest {
        return 0;
    }

    if let Some(dist) = cache.get(&(curr, seen)) {
        return *dist;
    }

    let mut dist = 0;

    for &(to, weight) in &graph.nodes[curr].to {
        if (seen >> to) & 1 == 1 {
            continue;
        }

        let seen = seen | (1 << to);

        if reachable(to, seen, graph) {
            let branch = recurse(to, seen, graph, cache);
            dist = dist.max(branch + weight);
        }
    }

    cache.insert((curr, seen), dist);
    dist
}

fn reachable(curr: usize, seen: u64, graph: &Graph) -> bool {
    let mut queue = 1_u64 << curr;
    let mut reachable = seen;

    while queue != 0 {
        let curr = queue.trailing_zeros() as usize;
        queue ^= 1 << curr;

        if curr == graph.dest {
            return true;
        }

        for &(to, _) in &graph.nodes[curr].to {
            let flag = 1 << to;
            queue |= flag & !reachable;
            reachable |= flag;
        }
    }

    false
}
