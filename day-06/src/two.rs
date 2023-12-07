pub fn run(input: &str) {
    let mut parser = utils::Parser::new(&input);

    fn badly_kerned_int(input: impl Iterator<Item = (usize, char)>) -> u64 {
        input
            .map(|(_, c)| c)
            .filter(|c| *c != ' ')
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    }

    parser.expect("Time:");
    let time = badly_kerned_int(parser.inner.by_ref().take_while(|(_, c)| *c != '\n'));

    parser.expect("Distance:");
    let dist = badly_kerned_int(parser.inner);

    let (time, dist) = (time as f64, dist as f64);

    let discrim = (time * time - 4. * dist).sqrt();
    let (lower, upper) = ((time - discrim) / 2., (time + discrim) / 2.);

    let result = ((upper - 1.).ceil() - (lower + 1.).floor()) as u64 + 1;
    println!("Two: {result}");
}
