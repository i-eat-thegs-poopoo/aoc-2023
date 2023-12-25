use std::collections::{HashMap, HashSet};
use utils::grid::*;

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let grid = parse(input);
        let start = grid.pos(0, 1).unwrap();
        let dest = grid.pos(grid.tiles.len() - 1, grid.tiles[0].len() - 2).unwrap();

        let mut ctx = Traversal {
            grid,
            dest,
            seen: HashMap::new(),
            branch_ids: vec![0],
            next_branch_id: 1,
        };

        hike(start, &mut ctx)
    });
    two(|input| 123);
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Path,
    Forest,
    Dir((isize, isize)),
}

struct Traversal {
    grid: Grid<Tile>,
    dest: Pos,
    seen: HashMap<Pos, HashSet<u64>>,
    branch_ids: Vec<u64>,
    next_branch_id: u64,
}

fn parse(input: &str) -> Grid<Tile> {
    let mut parser = utils::Parser::new(input);

    parser.grid(|char| match char {
        '.' => Tile::Path,
        '#' => Tile::Forest,
        '^' => Tile::Dir((-1, 0)),
        'v' => Tile::Dir((1, 0)),
        '<' => Tile::Dir((0, -1)),
        '>' => Tile::Dir((0, 1)),
        _ => panic!(),
    })
}

// HashMap of Vecs of ids of branches that have seen tile
fn hike(pos: Pos, ctx: &mut Traversal) -> u64 {
    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    if pos == ctx.dest {
        return 0;
    }

    let seen = ctx.seen.entry(pos).or_insert_with(|| HashSet::new());

    if ctx.branch_ids.iter().any(|id| seen.contains(id)) {
        return 0;
    }

    seen.insert(*ctx.branch_ids.last().unwrap());

    ctx.branch_ids.push(ctx.next_branch_id);
    ctx.next_branch_id += 1;

    let mut dist = 0;

    for dir in DIRS {
        if let Some(pos) = ctx.grid.shift(pos, dir) {
            match ctx.grid.get(pos) {
                Tile::Path => {}
                Tile::Forest => continue,
                Tile::Dir(slope) => {
                    if dir != *slope {
                        continue;
                    }
                }
            }

            dist = hike(pos, ctx).max(dist);
        }
    }

    ctx.branch_ids.pop();
    dist + 1
}
