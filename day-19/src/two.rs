use super::*;

#[derive(Clone)]
struct Part {
    cats: [(u64, u64); 4],
}

pub fn find_combos(workflows: &HashMap<&str, Workflow>) -> u64 {
    let part = Part {
        cats: [(1, 4000); 4],
    };

    apply_workflow(part, "in", &workflows)
}

fn apply_workflow<'a>(
    mut part: Part,
    workflow: &'a str,
    workflows: &HashMap<&'a str, Workflow<'a>>,
) -> u64 {
    let workflow = match workflow {
        "A" => {
            return part
                .cats
                .iter()
                .map(|(min, max)| max - min + 1)
                .product::<u64>();
        }
        "R" => return 0,
        name => workflows.get(name).unwrap(),
    };

    let mut combos = 0;

    for rule in &workflow.rules {
        let mut curr_part = part.clone();
        let (min, max) = curr_part.cats[rule.cat as usize];

        let cat = match rule.cmp {
            Ordering::Greater if min > rule.val => (min, max),
            Ordering::Greater if max > rule.val => {
                part.cats[rule.cat as usize] = (min, rule.val);
                (rule.val + 1, max)
            }
            Ordering::Less if max < rule.val => (min, max),
            Ordering::Less if min < rule.val => {
                part.cats[rule.cat as usize] = (rule.val, max);
                (min, rule.val - 1)
            }
            _ => continue,
        };

        curr_part.cats[rule.cat as usize] = cat;
        combos += apply_workflow(curr_part, rule.to, workflows);
    }

    combos + apply_workflow(part, workflow.fallback, workflows)
}
