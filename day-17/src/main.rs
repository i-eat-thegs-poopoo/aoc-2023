use std::{cmp::Ordering, collections::BinaryHeap};
use utils::grid::*;

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let mut grid = parse(input);
        traverse(&mut grid, 1, 3)
    });
    two(|input| {
        let mut grid = parse(input);
        traverse(&mut grid, 4, 10)
    });
}

fn parse(input: &str) -> Grid<Tile> {
    let mut parser = utils::Parser::new(input);
    parser.grid(|c| Tile {
        heat_loss: c.to_digit(10).unwrap() as u64,
        seen: 0,
    })
}

struct Tile {
    heat_loss: u64,
    seen: u64,
}

#[derive(PartialEq, Eq)]
struct State {
    pos: Pos,
    dir: (isize, isize),
    nth_in_dir: u64,
    cost: u64,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost).reverse())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

struct Traversal<'a> {
    queue: BinaryHeap<State>,
    grid: &'a mut Grid<Tile>,
}

impl<'a> Traversal<'a> {
    fn push_to_queue(&mut self, state: &State, dir: (isize, isize), nth_in_dir: u64) {
        if let Some(pos) = self.grid.shift(state.pos, dir) {
            self.queue.push(State {
                pos,
                dir,
                nth_in_dir,
                cost: state.cost,
            });
        }
    }
}

fn dir_to_flag(dir: (isize, isize), nth_in_dir: u64, max_step: u64) -> u64 {
    let dir = match dir {
        (1, 0) => 0,
        (-1, 0) => 1,
        (0, 1) => 2,
        (0, -1) => 3,
        _ => panic!(),
    };

    1 << (dir * max_step + nth_in_dir - 1)
}

fn traverse(grid: &mut Grid<Tile>, min_step: u64, max_step: u64) -> u64 {
    let start = grid.pos(0, 0).unwrap();
    let find = grid
        .pos(grid.tiles.len() - 1, grid.tiles[0].len() - 1)
        .unwrap();

    let mut traversal = Traversal {
        queue: BinaryHeap::from_iter([(1, 0), (0, 1)].into_iter().map(|dir| State {
            pos: grid.shift(start, dir).unwrap(),
            dir,
            nth_in_dir: 1,
            cost: 0,
        })),
        grid,
    };

    while let Some(mut state) = traversal.queue.pop() {
        let Tile { heat_loss, seen } = traversal.grid.get_mut(state.pos);
        let flag = dir_to_flag(state.dir, state.nth_in_dir, max_step);

        if *seen & flag != 0 {
            continue;
        }

        *seen |= flag;
        state.cost = state.cost + *heat_loss;

        if state.pos == find && state.nth_in_dir >= min_step {
            return state.cost;
        }

        if state.nth_in_dir >= min_step {
            // Not necessarily left and right in that order, but eh
            let (left, right) = match state.dir {
                (1 | -1, 0) => ((0, 1), (0, -1)),
                (0, 1 | -1) => ((1, 0), (-1, 0)),
                _ => panic!(),
            };

            traversal.push_to_queue(&state, left, 1);
            traversal.push_to_queue(&state, right, 1);
        }

        if state.nth_in_dir < max_step {
            traversal.push_to_queue(&state, state.dir, state.nth_in_dir + 1);
        }
    }

    panic!()
}
