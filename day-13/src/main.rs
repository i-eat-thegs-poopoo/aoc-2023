use std::iter;

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let terrains = parse(input);
        let mut sum = 0;

        for terrain in terrains {
            sum += find_sym(&terrain);
        }

        sum
    });
    two(|input| 123);
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

fn find_sym(terrain: &Terrain) -> usize {
    let horiz = find_axis_symmetry(
        0..terrain.len(),
        |curr, terrain, row| terrain[row].get(curr).copied(),
        terrain,
        terrain[0].len() - 1,
    );

    if let Some(pt) = horiz {
        return pt;
    }

    let vert = find_axis_symmetry(
        0..terrain[0].len(),
        |curr, terrain, col| terrain.get(curr).map(|row| row[col]),
        terrain,
        terrain.len() - 1,
    );

    if let Some(pt) = vert {
        return pt * 100;
    }

    panic!()
}

fn find_axis_symmetry(
    lines: impl Iterator<Item = usize>,
    get_tile: fn(usize, &Terrain, usize) -> Option<Tile>,
    terrain: &Terrain,
    max: usize,
) -> Option<usize> {
    let mut symmetry_pts = None::<Vec<usize>>;

    for line in lines {
        let pts = find_line_symmetry(get_tile, terrain, line);

        if let Some(ref mut symmetry_pts) = symmetry_pts {
            symmetry_pts.retain(|pt| pts.contains(pt) && *pt < max);
        } else {
            symmetry_pts = Some(pts);
        }
    }

    symmetry_pts.unwrap().get(0).map(|pt| pt + 1)
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

        if is_symmetric {
            symmetry_pts.push(curr);
        }

        curr += 1;
    }

    symmetry_pts
}
