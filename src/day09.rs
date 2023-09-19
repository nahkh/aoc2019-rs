pub fn execute() {
    let content = crate::input_files::read_content(&String::from("data/day09.txt"));
    let mut m1 = crate::intcode::read_program_with_input(content.clone(), 1);
    m1.execute_until_stopped();
    println!(
        "Part 1: BOOST keycode: {}",
        m1.get_last_output().unwrap()
    );
    let mut m2 = crate::intcode::read_program_with_input(content.clone(), 2);
    m2.execute_until_stopped();
    println!(
        "Part 2: Distress signal: {}",
        m2.get_last_output().unwrap()
    );
}
