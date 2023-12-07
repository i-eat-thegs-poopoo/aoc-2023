/*
let T, D
t(T - t) > D
-t^2 + Tt - D > 0
*/
pub fn run(input: &str) {
    let mut parser = utils::Parser::new(&input);

    let mut times = Vec::new();
    let mut dists = Vec::new();

    parser.expect("Time:");

    loop {
        if parser.str_while(|c| *c == ' ').len() == 0 {
            break;
        }

        let time = parser.int();
        times.push(time);
    }

    parser.expect("\nDistance:");

    loop {
        if parser.str_while(|c| *c == ' ').len() == 0 {
            break;
        }

        let dist = parser.int();
        dists.push(dist);
    }

    let mut product = 1;

    for (time, dist) in times.iter().zip(dists.iter()) {
        let (time, dist) = (*time as f64, *dist as f64);

        let discrim = (time * time - 4. * dist).sqrt();
        let (lower, upper) = ((time - discrim) / 2., (time + discrim) / 2.);

        product *= ((upper - 1.).ceil() - (lower + 1.).floor()) as u64 + 1;
    }

    println!("One: {product}");
}
