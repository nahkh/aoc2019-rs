use crate::input_files::read_content;
use crate::intcode::read_program;
use crate::intcode::Runnable;

pub fn execute() {
    let content = read_content(&String::from("data/day02.txt"));
    let c = content.matches(",").count() + 1;
    for i in 1..c {
        for j in 1..c {
            let mut m = read_program(content.clone());
            m.set_value(1, i as i32);
            m.set_value(2, j as i32);
            m.execute_until_stopped();
            if m.get_value(0) == 19690720 {
                println!("Noun {}, verb {}, output: {}", i, j, 100 * i + j);
                return;
            }
        }
    }
}
