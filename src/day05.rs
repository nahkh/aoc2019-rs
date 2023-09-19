pub fn execute() {
    let content = crate::input_files::read_content(&String::from("data/day05.txt"));
    let mut m1 = crate::intcode::read_program_with_input(content.clone(), 1);
    m1.execute_until_stopped();
    println!(
        "Part 1: Diagnostic output: {}",
        m1.get_last_output().unwrap()
    );
    let mut m2 = crate::intcode::read_program_with_input(content.clone(), 5);
    m2.execute_until_stopped();
    println!(
        "Part 2: Diagnostic output: {}",
        m2.get_last_output().unwrap()
    );
}
