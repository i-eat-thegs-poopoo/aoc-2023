use std::{env, fmt::Display, fs, rc::Rc, time};

mod parse;

pub use parse::*;

pub fn read_input() -> String {
    let path = env::args().nth(1).expect("Path not supplied");
    fs::read_to_string(path).expect("File not found")
}

pub fn run<Out: Display>(part: &str, func: impl FnOnce(&str) -> Out, input: &str) {
    let now = time::Instant::now();
    let out = func(input);
    let dur = now.elapsed();

    println!("Part {part}: {out}\n Elapsed: {dur:?}");
}

pub fn setup<
    OneOut: Display,
    TwoOut: Display,
    One: FnOnce(&str) -> OneOut,
    Two: FnOnce(&str) -> TwoOut,
>() -> (impl FnOnce(One), impl FnOnce(Two)) {
    let one_input = Rc::new(read_input());
    let two_input = one_input.clone();

    let one = move |func| run("one", func, one_input.as_str());
    let two = move |func| run("two", func, two_input.as_str());

    (one, two)
}
