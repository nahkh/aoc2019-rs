use itertools::Itertools; // 0.8.2
use crate::intcode::Runnable;
use crate::intcode::read_program;

fn evaluate_combination(content: String, phases: Vec<&i32>) -> i32 {
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

fn find_best_combination(content: String) -> i32 {
    let items: Vec<i32> = vec![0, 1, 2, 3, 4];
    let mut current_best = 0;
    for perm in items.iter().permutations(items.len()) {
        let value = evaluate_combination(content.clone(), perm);
        if value > current_best {
            current_best = value;
        }
    }

    current_best
}

pub fn execute() {
    let content = crate::input_files::read_content(&String::from("data/day07.txt"));
    let best_combination = find_best_combination(content);
    println!("Part 1: Best combination {}", best_combination);
}