use std::{cmp::Eq, collections::HashMap, hash::Hash};
use utils::grid::*;

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let (grid, start) = parse(input);

        walk(
            start,
            |pos| *grid.get(pos),
            |pos, dir| grid.shift(pos, dir),
            64,
        )
    });
    two(|input| {
        let (grid, start) = parse(input);
        let steps = 26501365;
        let len = grid.tiles.len() as i64;

        let start = (start, (0, 0));
        let get = |(pos, _)| *grid.get(pos);
        let shift = |(pos, (cx, cy)), (dx, dy)| {
            let chunk = (cx + dx, cy + dy);
            Some((grid.shift_wraparound(pos, (dx, dy)), chunk))
        };

        let x1 = steps % len;
        let x2 = x1 + len;
        let x3 = x2 + len;

        let y1 = walk(start, get, shift, x1);
        let y2 = walk(start, get, shift, x2);
        let y3 = walk(start, get, shift, x3);

        let (sx1, sx2, sx3) = (steps - x1, steps - x2, steps - x3);
        let term = |x1, y1, sx2, sx3, x2, x3| sx2 * sx3 / (x1 - x2) / (x1 - x3) * y1;

        let t1 = term(x1, y1, sx2, sx3, x2, x3);
        let t2 = term(x2, y2, sx1, sx3, x1, x3);
        let t3 = term(x3, y3, sx1, sx2, x1, x2);

        t1 + t2 + t3
    });
}

#[derive(Clone, Copy)]
enum Tile {
    Plot,
    Rock,
}

fn parse(input: &str) -> (Grid<Tile>, Pos) {
    let mut parser = utils::Parser::new(input);
    let mut start = (0, 0);

    let grid = parser.grid_with_pos(|char, pos| match char {
        'S' => {
            start = pos;
            Tile::Plot
        }
        '.' => Tile::Plot,
        '#' => Tile::Rock,
        _ => panic!(),
    });

    let start = grid.pos(start.0, start.1).unwrap();
    (grid, start)
}

fn walk<P: Copy + Hash + Eq>(
    start: P,
    get: impl Fn(P) -> Tile,
    shift: impl Fn(P, (isize, isize)) -> Option<P>,
    steps: i64,
) -> i64 {
    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut queue = vec![start];
    let mut buffer = Vec::new();

    let mut parity = 0;
    let mut seen = HashMap::new();

    for _ in 0..steps {
        buffer.clear();
        parity = 1 - parity;

        for pos in queue.drain(..) {
            for dir in DIRS {
                let Some(pos) = shift(pos, dir) else {
                    continue;
                };

                match get(pos) {
                    Tile::Plot => {
                        let parity = &mut seen.entry(pos).or_insert([false, false])[parity];

                        if !*parity {
                            *parity = true;
                            buffer.push(pos);
                        }
                    }
                    Tile::Rock => continue,
                }
            }
        }

        queue.extend(buffer.drain(..));
    }

    let mut sum = 0;

    for tile in seen.into_values() {
        if tile[parity] {
            sum += 1;
        }
    }

    sum
}
