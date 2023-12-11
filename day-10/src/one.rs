use std::collections::VecDeque;

#[derive(Clone, Copy)]
enum Tile {
    Vert,
    Horiz,
    Ne,
    Nw,
    Sw,
    Se,
    Ground,
}

pub fn run(input: &str) {
    let mut pipes = Vec::new();
    let mut start = (0, 0);

    for (r, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (c, tile) in line.chars().enumerate() {
            let tile = match tile {
                '|' => Tile::Vert,
                '-' => Tile::Horiz,
                'L' => Tile::Ne,
                'J' => Tile::Nw,
                '7' => Tile::Sw,
                'F' => Tile::Se,
                '.' => Tile::Ground,
                'S' => {
                    start = (r, c);
                    Tile::Ground
                }
                _ => panic!(),
            };

            row.push((tile, None));
        }

        pipes.push(row);
    }

    pipes[start.0][start.1] = {
        const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let openings = DIRS
            .into_iter()
            .map(|(dr, dc)| {
                let (r, c) = start;
                let nr = r.checked_add_signed(dr);
                let nc = c.checked_add_signed(dc);

                if let Some((nr, nc)) = nr.zip(nc) {
                    get_openings(pipes[nr][nc].0)
                        .filter(|o| o.contains(&(-dr, -dc)))
                        .is_some()
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        let tile = match openings.as_slice() {
            [true, true, false, false] => Tile::Vert,
            [false, false, true, true] => Tile::Horiz,
            [true, false, false, true] => Tile::Ne,
            [true, false, true, false] => Tile::Nw,
            [false, true, true, false] => Tile::Sw,
            [false, true, false, true] => Tile::Se,
            _ => panic!(),
        };

        (tile, Some(0))
    };

    let mut farthest = 0;
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some((r, c)) = queue.pop_front() {
        let (tile, Some(dist)) = pipes[r][c] else {
            panic!();
        };

        if dist > farthest {
            farthest = dist;
        }

        for (dr, dc) in get_openings(tile).into_iter().flatten() {
            let nr = r.checked_add_signed(dr);
            let nc = c.checked_add_signed(dc);
            let (nr, nc) = nr.zip(nc).unwrap();

            if pipes[nr][nc].1.is_none() {
                pipes[nr][nc].1 = Some(dist + 1);
                queue.push_back((nr, nc));
            }
        }
    }

    println!("One: {farthest}");
}

fn get_openings(tile: Tile) -> Option<[(isize, isize); 2]> {
    let openings = match tile {
        Tile::Vert => [(1, 0), (-1, 0)],
        Tile::Horiz => [(0, 1), (0, -1)],
        Tile::Ne => [(-1, 0), (0, 1)],
        Tile::Nw => [(-1, 0), (0, -1)],
        Tile::Sw => [(1, 0), (0, -1)],
        Tile::Se => [(1, 0), (0, 1)],
        Tile::Ground => return None,
    };

    Some(openings)
}
