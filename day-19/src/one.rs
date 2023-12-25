use super::*;

// `true` if accepted.
pub fn apply_all_workflows<'a>(part: &Part, workflows: &HashMap<&'a str, Workflow<'a>>) -> bool {
    let mut curr = "in";

    loop {
        let workflow = workflows.get(curr).unwrap();

        match apply_workflow(&part, workflow) {
            "A" => return true,
            "R" => return false,
            to => curr = to,
        }
    }
}

fn apply_workflow<'a>(part: &Part, workflow: &Workflow<'a>) -> &'a str {
    for rule in &workflow.rules {
        let val = part.cats[rule.cat as usize];
        let cmp = val.cmp(&rule.val);

        if cmp == rule.cmp {
            return rule.to;
        }
    }

    workflow.fallback
}
