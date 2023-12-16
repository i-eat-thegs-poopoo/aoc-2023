mod one;
mod two;

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let dish = parse(input);
        one::dish_load(&dish)
    });
    two(|input| {
        let mut dish = parse(input);
        two::calc_load(&mut dish)
    });
}

type Dish = Vec<Vec<Tile>>;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tile {
    Round,
    Square,
    None,
}

fn parse(input: &str) -> Dish {
    let mut dish = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();

        for char in line.chars() {
            let tile = match char {
                'O' => Tile::Round,
                '#' => Tile::Square,
                '.' => Tile::None,
                _ => panic!(),
            };

            row.push(tile);
        }

        dish.push(row);
    }

    dish
}
