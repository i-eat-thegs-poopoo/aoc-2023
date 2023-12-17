use utils::grid::*;

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let mut grid = parse(input);
        launch_beam(grid.pos(0, 0).unwrap(), (0, 1), &mut grid, 1)
    });
    two(|input| {
        let mut grid = parse(input);
        let mut gen = 0;
        let mut max = 0;

        let rows = grid.tiles.len();
        let cols = grid.tiles[0].len();

        for row in 0..rows {
            max = max
                .max(launch_beam(
                    grid.pos(row, 0).unwrap(),
                    (0, 1),
                    &mut grid,
                    gen + 1,
                ))
                .max(launch_beam(
                    grid.pos(row, cols - 1).unwrap(),
                    (0, -1),
                    &mut grid,
                    gen + 2,
                ));

            gen += 2;
        }

        for col in 0..cols {
            max = max
                .max(launch_beam(
                    grid.pos(0, col).unwrap(),
                    (1, 0),
                    &mut grid,
                    gen + 1,
                ))
                .max(launch_beam(
                    grid.pos(rows - 1, col).unwrap(),
                    (-1, 0),
                    &mut grid,
                    gen + 2,
                ));

            gen += 2;
        }

        max
    });
}

#[derive(Clone, Copy)]
enum Mirror {
    Positive, // ie, a positive horiz becomes a positive vert
    Negative,
    Horizontal,
    Vertical,
    Empty,
}

fn parse(input: &str) -> Grid<(Mirror, u8, u64)> {
    let mut parser = utils::Parser::new(input);

    parser.grid(|char| {
        let mirror = match char {
            '\\' => Mirror::Positive,
            '/' => Mirror::Negative,
            '-' => Mirror::Horizontal,
            '|' => Mirror::Vertical,
            '.' => Mirror::Empty,
            _ => panic!(),
        };

        (mirror, 0, 0) // Direction bitflags and generation
    })
}

fn launch_beam(
    mut pos: Pos,
    mut dir: (isize, isize),
    grid: &mut Grid<(Mirror, u8, u64)>,
    gen: u64,
) -> u64 {
    let mut total = 0;

    loop {
        let (mirror, energized, mirror_gen) = grid.get_mut(pos);

        match reflect(dir, *mirror) {
            Reflection::One(new_dir) => dir = new_dir,
            Reflection::Two(dir_a, dir_b) => {
                let a = launch_beam(pos, dir_a, grid, gen);
                let b = launch_beam(pos, dir_b, grid, gen);
                total += a + b;
                break;
            }
        }

        if *mirror_gen != gen {
            *energized = 0;
            *mirror_gen = gen;
        }

        let flag = dir_to_flags(dir);

        if *energized & flag != 0 {
            break;
        } else if *energized == 0 {
            total += 1;
        }

        *energized |= flag;

        pos = if let Some(new_pos) = grid.shift(pos, dir) {
            new_pos
        } else {
            break;
        };
    }

    total
}

enum Reflection {
    One((isize, isize)),
    Two((isize, isize), (isize, isize)),
}

fn reflect(dir: (isize, isize), mirror: Mirror) -> Reflection {
    let dir = match mirror {
        Mirror::Positive => match dir {
            (step, 0) => (0, step),
            (0, step) => (step, 0),
            _ => panic!(),
        },
        Mirror::Negative => match dir {
            (step, 0) => (0, -step),
            (0, step) => (-step, 0),
            _ => panic!(),
        },
        Mirror::Horizontal => match dir {
            (_, 0) => return Reflection::Two((0, 1), (0, -1)),
            (0, _) => dir,
            _ => panic!(),
        },
        Mirror::Vertical => match dir {
            (0, _) => return Reflection::Two((1, 0), (-1, 0)),
            (_, 0) => dir,
            _ => panic!(),
        },
        Mirror::Empty => dir,
    };

    Reflection::One(dir)
}

fn dir_to_flags(dir: (isize, isize)) -> u8 {
    match dir {
        (1, 0) => 0b0001,
        (-1, 0) => 0b0010,
        (0, 1) => 0b0100,
        (0, -1) => 0b1000,
        _ => panic!(),
    }
}
