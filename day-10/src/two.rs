use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
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

            row.push((tile, false));
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

        (tile, true)
    };

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some((r, c)) = queue.pop_front() {
        let (tile, true) = pipes[r][c] else {
            panic!();
        };

        for (dr, dc) in get_openings(tile).into_iter().flatten() {
            let nr = r.checked_add_signed(dr);
            let nc = c.checked_add_signed(dc);
            let (nr, nc) = nr.zip(nc).unwrap();

            if !pipes[nr][nc].1 {
                pipes[nr][nc].1 = true;
                queue.push_back((nr, nc));
            }
        }
    }

    let mut enclosed = 0;

    for row in pipes {
        let mut tiles = row.into_iter();
        let mut inside = false;

        while let Some((tile, is_loop)) = tiles.next() {
            if is_loop {
                let open_dir = match tile {
                    Tile::Ne | Tile::Nw => -1,
                    Tile::Sw | Tile::Se => 1,
                    Tile::Vert => {
                        inside = !inside;
                        continue;
                    }
                    _ => panic!(),
                };

                while let Some((tile, _)) = tiles.next() {
                    let close_dir = match tile {
                        Tile::Horiz => continue,
                        Tile::Ne | Tile::Nw => -1,
                        Tile::Sw | Tile::Se => 1,
                        _ => panic!(),
                    };

                    if open_dir != close_dir {
                        inside = !inside;
                    }

                    break;
                }
            } else if inside {
                enclosed += 1;
            }
        }
    }

    println!("Two: {enclosed}");
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
