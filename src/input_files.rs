pub fn read_content(filename: &String) -> String {
    std::fs::read_to_string(filename).expect("Should have been able to read file")
}
