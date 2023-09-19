use crate::input_files::read_content;
use crate::intcode::IntCodeComputer;

fn part1() {
    let content = read_content(&String::from("data/day02.txt"));
    let mut m = IntCodeComputer::read_program(&content);
    m.set_value(1, 12);
    m.set_value(2, 2);
    m.execute_until_stopped();
    println!("Part 1: Value {}", m.get_value(0));
}

fn part2() {
    let content = read_content(&String::from("data/day02.txt"));
    let c = content.matches(",").count() + 1;
    for i in 1..c {
        for j in 1..c {
            let mut m = IntCodeComputer::read_program(&content);
            m.set_value(1, i as i64);
            m.set_value(2, j as i64);
            m.execute_until_stopped();
            if m.get_value(0) == 19690720 {
                println!("Part 2: Noun {}, verb {}, output: {}", i, j, 100 * i + j);
                return;
            }
        }
    }
}

pub fn execute() {
    part1();
    part2();
}
