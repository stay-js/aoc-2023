pub fn get_input() -> (String, String) {
    let demo_input = std::fs::read_to_string(format!(
        "inputs/{}/demo-input.txt",
        std::env::args().next().unwrap().split("/").last().unwrap(),
    ))
    .unwrap();

    let input = std::fs::read_to_string(format!(
        "inputs/{}/input.txt",
        std::env::args().next().unwrap().split("/").last().unwrap(),
    ))
    .unwrap();

    return (demo_input, input);
}

pub fn get_input_by_name(file_name: &str) -> String {
    return std::fs::read_to_string(format!(
        "inputs/{}/{}",
        std::env::args().next().unwrap().split("/").last().unwrap(),
        file_name,
    ))
    .unwrap();
}
