mod one;
mod two;

fn main() {
    let input = utils::read_input();
    one::run(&input);
    two::run(&input)
}
