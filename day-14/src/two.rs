use super::*;
use std::collections::HashMap;

pub fn calc_load(dish: &mut Dish) -> usize {
    do_cycles(dish);

    let mut total_load = 0;
    let mut curr_load = dish.len();

    for row in dish.iter() {
        for tile in row.iter() {
            if let Tile::Round = tile {
                total_load += curr_load;
            }
        }

        curr_load -= 1;
    }

    total_load
}

fn do_cycles(dish: &mut Dish) {
    const CYCLES: usize = 1_000_000_000;

    let mut curr_cycle = 0;
    let mut cache = HashMap::new();

    while curr_cycle < CYCLES {
        let rounds = dish_to_rounds(dish);

        if let Some(cycle_cycle) = cache.insert(rounds, curr_cycle) {
            let period = curr_cycle - cycle_cycle;
            curr_cycle += (CYCLES - curr_cycle) / period * period;

            if curr_cycle == CYCLES {
                break;
            }
        }

        cycle_dish(dish);
        curr_cycle += 1;
    }
}

fn dish_to_rounds(dish: &Dish) -> Vec<u32> {
    let mut rounds = Vec::new();
    let mut pos = 0;

    for row in dish.iter() {
        for tile in row.iter() {
            if let Tile::Round = tile {
                rounds.push(pos);
            }

            pos += 1;
        }
    }

    rounds
}

fn cycle_dish(dish: &mut Dish) {
    let (rows, cols) = (dish.len(), dish[0].len());

    // Tilt north
    for col in 0..cols {
        tilt_line(dish, (0, col), (1, 0));
    }

    // Tilt west
    for row in 0..rows {
        tilt_line(dish, (row, 0), (0, 1));
    }

    // Tilt south
    for col in 0..cols {
        tilt_line(dish, (rows - 1, col), (-1, 0));
    }

    // Tilt east
    for row in 0..rows {
        tilt_line(dish, (row, cols - 1), (0, -1));
    }
}

fn tilt_line(dish: &mut Dish, start: (usize, usize), step: (isize, isize)) {
    let mut curr = start;

    loop {
        let range = curr;
        let mut rounds = 0;
        let mut nones = 0;

        loop {
            match index(dish, curr) {
                Some(Tile::Round) => rounds += 1,
                Some(Tile::Square) => {
                    tilt_range(dish, range, step, rounds, nones);

                    if let Some(next) = apply_step(curr, step) {
                        curr = next;
                        break;
                    } else {
                        return;
                    };
                }
                Some(Tile::None) => nones += 1,
                None => {
                    tilt_range(dish, range, step, rounds, nones);
                    return;
                }
            }

            let Some(next) = apply_step(curr, step) else {
                tilt_range(dish, range, step, rounds, nones);
                return;
            };

            curr = next;
        }
    }
}

fn tilt_range(
    dish: &mut Dish,
    start: (usize, usize),
    step: (isize, isize),
    rounds: usize,
    nones: usize,
) {
    let mut curr = start;

    for _ in 0..rounds {
        *index(dish, curr).unwrap() = Tile::Round;

        if let Some(next) = apply_step(curr, step) {
            curr = next;
        }
    }

    for _ in 0..nones {
        *index(dish, curr).unwrap() = Tile::None;

        if let Some(next) = apply_step(curr, step) {
            curr = next;
        }
    }
}

fn apply_step(pos: (usize, usize), step: (isize, isize)) -> Option<(usize, usize)> {
    let row = pos.0.checked_add_signed(step.0);
    let col = pos.1.checked_add_signed(step.1);

    row.zip(col)
}

fn index<'a>(dish: &'a mut Dish, pos: (usize, usize)) -> Option<&'a mut Tile> {
    dish.get_mut(pos.0).and_then(|row| row.get_mut(pos.1))
}
