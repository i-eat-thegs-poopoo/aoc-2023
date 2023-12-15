use std::iter;

mod one;
mod two;

fn main() {
    let (one, two) = utils::setup();
    one(|input| find_symmetry(input, one::find_axis_symmetry));
    two(|input| find_symmetry(input, two::find_axis_fixable));
}

type Terrain = Vec<Vec<Tile>>;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tile {
    Ash,
    Rock,
}

fn parse(input: &str) -> Vec<Terrain> {
    let mut lines = input.lines();
    let mut terrains = Vec::new();

    loop {
        let mut terrain = Vec::new();

        loop {
            let line = match lines.next() {
                Some("") => {
                    terrains.push(terrain);
                    break;
                }
                Some(line) => line,
                None => {
                    terrains.push(terrain);
                    return terrains;
                }
            };

            let row = line
                .chars()
                .map(|c| match c {
                    '.' => Tile::Ash,
                    '#' => Tile::Rock,
                    _ => panic!(),
                })
                .collect();

            terrain.push(row);
        }
    }
}

struct AxisParams<'a> {
    lines: usize,
    max: usize,
    get_tile: fn(usize, &Terrain, usize) -> Option<Tile>,
    terrain: &'a Terrain,
}

fn find_symmetry(input: &str, find_terrain_symmetry: fn(AxisParams) -> Option<usize>) -> usize {
    let terrains = parse(input);
    let mut sum = 0;

    for terrain in terrains {
        let horiz = find_terrain_symmetry(AxisParams {
            lines: terrain.len(),
            max: terrain[0].len(),
            get_tile: |curr, terrain, row| terrain[row].get(curr).copied(),
            terrain: &terrain,
        });

        if let Some(pt) = horiz {
            sum += pt;
            continue;
        }

        let vert = find_terrain_symmetry(AxisParams {
            lines: terrain[0].len(),
            max: terrain.len(),
            get_tile: |curr, terrain, col| terrain.get(curr).map(|row| row[col]),
            terrain: &terrain,
        });

        if let Some(pt) = vert {
            sum += pt * 100;
            continue;
        }

        panic!()
    }

    sum
}

fn find_line_symmetry(
    get_tile: fn(usize, &Terrain, usize) -> Option<Tile>,
    terrain: &Terrain,
    parent: usize,
) -> Vec<usize> {
    let mut seen = Vec::new();
    let mut symmetry_pts = Vec::new();

    let mut curr = 0;

    while let Some(tile) = get_tile(curr, terrain, parent) {
        seen.push(tile);

        let is_symmetric = seen
            .iter()
            .rev()
            .zip({
                let mut curr = curr;
                iter::from_fn(move || {
                    curr += 1;
                    get_tile(curr, terrain, parent)
                })
            })
            .all(|(a, b)| *a == b);

        curr += 1;

        if is_symmetric {
            symmetry_pts.push(curr);
        }
    }

    symmetry_pts
}
