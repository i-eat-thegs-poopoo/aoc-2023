use std::{cmp::Ordering, collections::HashMap};

mod one;
mod two;

fn main() {
    let (one, two) = utils::setup();
    one(|input| {
        let (workflows, parts) = parse(input);

        parts
            .into_iter()
            .filter(|part| one::apply_all_workflows(part, &workflows))
            .flat_map(|part| part.cats)
            .sum::<u64>()
    });
    two(|input| {
        let (workflows, _) = parse(input);
        two::find_combos(&workflows)
    });
}

#[derive(Clone, Copy)]
enum Category {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

struct Rule<'a> {
    cat: Category,
    cmp: Ordering,
    val: u64,
    to: &'a str,
}

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    fallback: &'a str,
}

struct Part {
    cats: [u64; 4],
}

fn parse<'a>(input: &'a str) -> (HashMap<&'a str, Workflow<'a>>, Vec<Part>) {
    let mut parser = utils::Parser::new(input);
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    parser.sep_by_until("\n", "\n\n", |parser| {
        let name = parser.ident();
        parser.expect("{");

        let mut rules = Vec::new();
        let mut fallback = "";
        parser.sep_by(",", |parser| {
            let cat = match parser.ident() {
                "x" => Category::X,
                "m" => Category::M,
                "a" => Category::A,
                "s" => Category::S,
                fb => {
                    fallback = fb;
                    return;
                }
            };

            let cmp = match parser.next().unwrap() {
                '>' => Ordering::Greater,
                '<' => Ordering::Less,
                _ => panic!(),
            };

            let val = parser.int();
            parser.expect(":");
            let to = parser.ident();

            rules.push(Rule { cat, cmp, val, to });
        });

        parser.expect("}");
        workflows.insert(name, Workflow { rules, fallback });
    });

    parser.sep_by("\n", |parser| {
        parser.expect("{");

        let mut cats = [0; 4];
        parser.sep_by(",", |parser| {
            let cat = match parser.ident() {
                "x" => Category::X,
                "m" => Category::M,
                "a" => Category::A,
                "s" => Category::S,
                _ => panic!(),
            };

            parser.expect("=");
            let val = parser.int();

            cats[cat as usize] = val;
        });

        parser.expect("}");
        parts.push(Part { cats });
    });

    (workflows, parts)
}
