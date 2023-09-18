use crate::intcode::read_program;
use crate::intcode::IntCodeComputer;
use crate::intcode::Runnable;
use itertools::Itertools; // 0.8.2

fn evaluate_combination(content: String, phases: Vec<&i64>) -> i64 {
    let mut output = 0;
    for phase in phases.iter() {
        let mut m = read_program(content.clone());
        m.add_input(**phase);
        m.add_input(output);
        m.execute_until_stopped();
        output = m.get_last_output().unwrap();
    }

    output
}

fn find_best_combination(content: String) -> i64 {
    let items: Vec<i64> = vec![0, 1, 2, 3, 4];
    let mut current_best = 0;
    for perm in items.iter().permutations(items.len()) {
        let value = evaluate_combination(content.clone(), perm);
        if value > current_best {
            current_best = value;
        }
    }

    current_best
}

fn evalute_combination_recursively(content: String, phases: Vec<&i64>) -> i64 {
    let mut machines: Vec<IntCodeComputer> = Vec::new();
    for phase in phases.iter() {
        let mut m = read_program(content.clone());
        m.add_input(**phase);
        machines.push(m);
    }
    let mut current_value = 0;
    let mut still_running = true;
    while still_running {
        for m in machines.iter_mut() {
            m.add_input(current_value);
            m.execute_until_stopped();
            current_value = m.get_last_output().unwrap();
            still_running = still_running && !m.has_terminated();
        }
    }

    current_value
}

fn find_best_combination_recursively(content: String) -> i64 {
    let items: Vec<i64> = vec![5, 6, 7, 8, 9];
    let mut current_best = 0;
    for perm in items.iter().permutations(items.len()) {
        let value = evalute_combination_recursively(content.clone(), perm);
        if value > current_best {
            current_best = value;
        }
    }

    current_best
}

pub fn execute() {
    let content = crate::input_files::read_content(&String::from("data/day07.txt"));
    let best_combination = find_best_combination(content.clone());
    println!("Part 1: Best combination {}", best_combination);
    let best_recursive_combination = find_best_combination_recursively(content.clone());
    println!(
        "Part 2: Best recursive combination {}",
        best_recursive_combination
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn evalute_known_pattern() {
        let content =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .to_string();
        let value =
            crate::day07::evalute_combination_recursively(content, vec![&9, &8, &7, &6, &5]);
        assert_eq!(value, 139629729);
    }

    #[test]
    fn find_best_pattern() {
        let content =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .to_string();
        let value = crate::day07::find_best_combination_recursively(content);
        assert_eq!(value, 139629729);
    }
}
