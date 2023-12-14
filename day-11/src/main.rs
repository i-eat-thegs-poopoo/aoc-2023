fn main() {
    let (one, two) = utils::setup();
    one(|input| run(input, 2));
    two(|input| run(input, 1_000_000));
}

fn run(input: &str, expand: u64) -> u64 {
    let (galaxies, empty_rows, empty_cols) = parse(input);
    let mut sum = 0;

    for i in 0..galaxies.len() - 1 {
        let curr = galaxies[i];

        for &(ax, ay) in &galaxies[i + 1..] {
            let (bx, by) = curr;

            let mut calc_axis = |a, b, empty_axis: &[_]| {
                let range = if a < b { a..b } else { b..a };
                for n in range {
                    sum += if empty_axis.contains(&n) { expand } else { 1 };
                }
            };

            calc_axis(ax, bx, &empty_cols);
            calc_axis(ay, by, &empty_rows);
        }
    }

    sum
}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
    let mut galaxies = Vec::new();
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut row_is_empty = true;

        for (x, char) in line.chars().enumerate() {
            if empty_cols.len() <= x {
                empty_cols.push(true);
            }

            match char {
                '#' => {
                    galaxies.push((x, y));
                    row_is_empty = false;
                    empty_cols[x] = false;
                }
                '.' => continue,
                _ => panic!(),
            }
        }

        if row_is_empty {
            empty_rows.push(y);
        }
    }

    let empty_cols = empty_cols
        .into_iter()
        .enumerate()
        .filter(|(_, is_empty)| *is_empty)
        .map(|(x, _)| x)
        .collect::<Vec<_>>();

    (galaxies, empty_rows, empty_cols)
}
