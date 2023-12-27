use std::{collections::HashSet, iter};

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let bricks = parse(input);
        let brick_count = bricks.len();

        let grid = fall(bricks);

        brick_count - find_unremovable(&grid)
    });
    two(|input| {
        let bricks = parse(input);
        let grid = fall(bricks);
        find_disintegrated(&grid)
    });
}

// Assuming `coord.0` < `coord.1`
struct Brick {
    start: [u64; 3],
    axis: usize,
    len: u64,
}

struct Interaction {
    supports: HashSet<usize>,
    depends: HashSet<usize>,
}

struct Grid {
    // zs[ys[xs[id?]]]
    cubes: Vec<Vec<Vec<Option<usize>>>>,
    ints: Vec<Interaction>,
}

impl Grid {
    fn get(&self, pos: [u64; 3]) -> Option<usize> {
        let [x, y, z] = pos.map(|coord| coord as usize);
        self.cubes.get(z)?.get(y)?.get(x)?.clone()
    }

    // Panics if brick already exists
    fn insert(&mut self, pos: [u64; 3], id: usize) {
        let [x, y, z] = pos.map(|coord| coord as usize);

        fn get_or_extend<'a, T: Clone>(vec: &'a mut Vec<T>, idx: usize, fill: T) -> &'a mut T {
            if vec.len() <= idx {
                let ext = iter::repeat(fill).take(idx - vec.len() + 1);
                vec.extend(ext);
                return vec.get_mut(idx).unwrap();
            }

            vec.get_mut(idx).unwrap()
        }

        let ys = get_or_extend(&mut self.cubes, z, Vec::new());
        let xs = get_or_extend(ys, y, Vec::new());
        let cube = get_or_extend(xs, x, None);

        assert!(cube.is_none());
        *cube = Some(id);
    }

    fn new_dep(&mut self) -> usize {
        let id = self.ints.len();
        self.ints.push(Interaction {
            supports: HashSet::new(),
            depends: HashSet::new(),
        });

        id
    }
}

fn parse(input: &str) -> Vec<Brick> {
    let mut parser = utils::Parser::new(input);
    let mut bricks = Vec::new();

    fn coords(parser: &mut utils::Parser) -> [u64; 3] {
        let x = parser.int();
        parser.expect(",");
        let y = parser.int();
        parser.expect(",");
        let z = parser.int();

        [x, y, z]
    }

    parser.sep_by("\n", |parser| {
        let start = coords(parser);
        parser.expect("~");
        let end = coords(parser);

        let axis = start
            .into_iter()
            .zip(end)
            .position(|(a, b)| a != b)
            .unwrap_or(0);
        let len = end[axis] - start[axis] + 1;

        bricks.push(Brick { start, axis, len });
    });

    bricks
}

// Returns list of sets of ids of supporting bricks for each brick.
fn fall(mut bricks: Vec<Brick>) -> Grid {
    bricks.sort_by_key(|brick| brick.start[2]);

    let mut grid = Grid {
        cubes: Vec::new(),
        ints: Vec::new(),
    };

    for brick in bricks {
        let id = grid.new_dep();

        let cubes = (0..brick.len).map(|offset| {
            let mut coords = brick.start;
            coords[brick.axis] += offset;
            coords
        });

        let mut offset = 0;

        while brick.start[2] - offset > 1 {
            for mut pos in cubes.clone() {
                pos[2] -= offset + 1;

                if let Some(sup_id) = grid.get(pos) {
                    grid.ints[id].depends.insert(sup_id);
                    grid.ints[sup_id].supports.insert(id);
                }
            }

            if !grid.ints[id].depends.is_empty() {
                break;
            }

            offset += 1;
        }

        for mut pos in cubes {
            pos[2] -= offset;
            grid.insert(pos, id);
        }
    }

    grid
}

fn find_unremovable(grid: &Grid) -> usize {
    let mut unremovable = HashSet::new();

    for Interaction { depends, .. } in &grid.ints {
        if depends.len() == 1 {
            let sup_id = depends.iter().next().unwrap();
            unremovable.insert(*sup_id);
        }
    }

    unremovable.len()
}

fn find_disintegrated(grid: &Grid) -> usize {
    fn disintegrate(id: usize, grid: &Grid, disinted: &mut HashSet<usize>) -> usize {
        disinted.insert(id);

        let mut disint_count = 0;

        for &dep_id in &grid.ints[id].supports {
            if disinted.contains(&dep_id) {
                continue;
            }

            let disint = grid.ints[dep_id]
                .depends
                .iter()
                .all(|sup_id| disinted.contains(sup_id));

            if disint {
                disint_count += 1 + disintegrate(dep_id, grid, disinted);
            }
        }

        disint_count
    }

    let mut disint_count = 0;
    let mut disinted = HashSet::new();

    for id in 0..grid.ints.len() {
        disinted.clear();
        disint_count += disintegrate(id, grid, &mut disinted);
    }

    disint_count
}
