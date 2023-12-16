use super::*;

pub fn dish_load(dish: &Dish) -> usize {
    (0..dish[0].len()).map(|col| col_load(col, dish)).sum()
}

fn col_load(col: usize, dish: &Dish) -> usize {
    let mut tiles = dish
        .iter()
        .enumerate()
        .map(|(idx, row)| (row[col], dish.len() - idx));

    let mut total_load = 0;
    let mut curr_load = dish.len();

    loop {
        let mut rounds = 0;

        loop {
            match tiles.next() {
                Some((Tile::Round, _)) => rounds += 1,
                Some((Tile::Square, load)) => {
                    total_load += calc_load(curr_load, rounds);
                    curr_load = load - 1;
                    break;
                }
                Some((Tile::None, _)) => (),
                None => {
                    total_load += calc_load(curr_load, rounds);
                    return total_load;
                }
            }
        }
    }
}

fn calc_load(high: usize, count: usize) -> usize {
    if count > 0 {
        let low = high - count + 1;
        count * (low + high) / 2
    } else {
        0
    }
}
