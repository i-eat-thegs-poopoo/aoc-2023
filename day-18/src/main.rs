fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let instrs = parse_normal(input);
        let mut grid = Grid::new();

        dig(&instrs, &mut grid);
        area(&mut grid)
    });
    two(|input| {
        let instrs = parse_color(input);
        let mut grid = Grid::new();

        dig(&instrs, &mut grid);
        area(&mut grid)
    });
}

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Instr {
    dir: Dir,
    count: i64,
}

#[derive(Clone, Copy)]
enum Trench {
    // Never an endpoint
    Vertical(i64),

    // Endpoint inclusive
    Protrusion { start: i64, count: i64 },
    Offset { start: i64, count: i64 },
}

struct Grid {
    rows: Vec<(i64, i64, Vec<Trench>)>,
    min: i64,
    max: i64,
}

impl Grid {
    fn new() -> Self {
        Self {
            rows: Vec::new(),
            min: 0,
            max: 0,
        }
    }

    fn find_y_idx(&self, y: i64) -> Option<usize> {
        self.rows
            .iter()
            .position(|(start, end, _)| y >= *start && y <= *end)
    }

    fn split_upward(&mut self, y: i64) -> Option<usize> {
        let idx = self.find_y_idx(y)?;
        let (start, _, ref trenches) = self.rows[idx];

        if y == start {
            return Some(idx);
        }

        self.rows.insert(idx, (start, y - 1, trenches.clone()));
        self.rows[idx + 1].0 = y;

        Some(idx + 1)
    }

    fn split_downward(&mut self, y: i64) -> Option<usize> {
        let idx = self.find_y_idx(y)?;
        let (start, end, ref trenches) = self.rows[idx];

        if y == end {
            return Some(idx);
        }

        self.rows.insert(idx, (start, y, trenches.clone()));
        self.rows[idx + 1].0 = y + 1;

        Some(idx)
    }

    fn insert(&mut self, ymin: i64, ymax: i64, mut callback: impl FnMut(&mut Vec<Trench>)) {
        let (min, max) = if self.rows.is_empty() {
            self.rows.push((ymin, ymax, Vec::new()));
            (0, 0)
        } else if ymax < self.min {
            self.rows.insert(0, (ymin, ymax, Vec::new()));
            (0, 0)
        } else if ymin > self.max {
            self.rows.push((ymin, ymax, Vec::new()));
            let idx = self.rows.len() - 1;
            (idx, idx)
        } else {
            let min = if ymin < self.min {
                self.rows.insert(0, (ymin, self.rows[0].0 - 1, Vec::new()));
                0
            } else {
                self.split_upward(ymin).unwrap()
            };

            let max = if ymax > self.max {
                let last = self.rows.last().unwrap();
                self.rows.push((last.1 + 1, ymax, Vec::new()));
                self.rows.len() - 1
            } else {
                self.split_downward(ymax).unwrap()
            };

            (min, max)
        };

        self.min = self.min.min(ymin);
        self.max = self.max.max(ymax);

        for idx in min..=max {
            callback(&mut self.rows[idx].2)
        }
    }
}

fn parse_normal(input: &str) -> Vec<Instr> {
    let mut parser = utils::Parser::new(input);
    let mut instrs = Vec::new();

    parser.sep_by("\n", |parser| {
        let dir = match parser.ident() {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => panic!(),
        };

        parser.expect(" ");
        let count = parser.int() as i64;
        parser.expect(" (#");
        parser.alphanumeric();
        parser.expect(")");

        instrs.push(Instr { dir, count })
    });

    instrs
}

fn parse_color(input: &str) -> Vec<Instr> {
    let mut parser = utils::Parser::new(input);
    let mut instrs = Vec::new();

    parser.sep_by("\n", |parser| {
        parser.ident();
        parser.expect(" ");
        parser.int();
        parser.expect(" (#");
        let color = parser.alphanumeric();
        parser.expect(")");

        let mut digits = color.chars();

        let count = digits
            .by_ref()
            .take(5)
            .flat_map(|digit| digit.to_digit(16))
            .enumerate()
            .map(|(place, digit)| digit as i64 * 16_i64.pow(4 - place as u32))
            .sum();

        let dir = match digits.next().unwrap() {
            '0' => Dir::Right,
            '1' => Dir::Down,
            '2' => Dir::Left,
            '3' => Dir::Up,
            _ => panic!(),
        };

        instrs.push(Instr { dir, count })
    });

    instrs
}

// Assuming no consecutive verticals or horizontals
fn dig(instrs: &[Instr], grid: &mut Grid) {
    let instrs_len = instrs.len();
    let mut x = 0;
    let mut y = 0;

    for idx in 0..instrs_len {
        let instr = &instrs[idx];

        match instr.dir {
            Dir::Up | Dir::Down => {
                let (min, max) = match instr.dir {
                    Dir::Up => {
                        let min = y + 1;
                        y += instr.count;

                        (min, y - 1)
                    }
                    Dir::Down => {
                        let max = y - 1;
                        y -= instr.count;

                        (y + 1, max)
                    }
                    _ => panic!(),
                };

                grid.insert(min, max, |trenches| trenches.push(Trench::Vertical(x)));
            }
            Dir::Left | Dir::Right => {
                let wraparound = |offset| {
                    idx.checked_add_signed(offset)
                        .map(|idx| idx % instrs_len)
                        .unwrap_or(instrs_len - 1)
                };
                let (prev, next) = (instrs[wraparound(-1)].dir, instrs[wraparound(1)].dir);

                let start = match instr.dir {
                    Dir::Left => {
                        x -= instr.count;
                        x
                    }
                    Dir::Right => {
                        let start = x;
                        x += instr.count;
                        start
                    }
                    _ => panic!(),
                };

                let trench = if prev == next {
                    Trench::Offset {
                        start,
                        count: instr.count + 1,
                    }
                } else {
                    Trench::Protrusion {
                        start,
                        count: instr.count + 1,
                    }
                };

                grid.insert(y, y, |trenches| {
                    trenches.push(trench);
                });
            }
        }
    }
}

fn area(grid: &mut Grid) -> i64 {
    let mut total = 0;

    for (start, end, trenches) in grid.rows.iter_mut() {
        let mut area = 0;

        trenches.sort_by_key(|trench| match *trench {
            Trench::Vertical(x) => x,
            Trench::Protrusion { start, .. } => start,
            Trench::Offset { start, .. } => start,
        });

        // Some(_) if inside
        let mut prev = None;

        for trench in trenches {
            match trench {
                Trench::Vertical(x) => match prev.take() {
                    Some(prev) => area += *x - prev,
                    None => {
                        area += 1;
                        prev = Some(*x);
                    }
                },
                Trench::Protrusion { start, count } => {
                    area += *count;

                    if let Some(ref mut prev) = prev {
                        area += *start - *prev - 1;
                        *prev = *start + *count - 1;
                    }
                }
                Trench::Offset { start, count } => {
                    area += *count;

                    match prev.take() {
                        Some(prev) => area += *start - prev - 1,
                        None => prev = Some(*start + *count - 1),
                    }
                }
            }
        }

        total += area * (*end - *start + 1)
    }

    total
}
