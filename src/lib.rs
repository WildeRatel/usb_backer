pub fn test_for_drive() -> std::result::Result<Vec<String>, std::io::Error> {
    let test_for_drive = std::process::Command::new("ls").output()?;
    let test_output = String::from_utf8(test_for_drive.stdout).unwrap();
    let test_output_vec: Vec<String> = test_output
        .split('\n')
        .map(|x| str::to_string(x).clone())
        .collect();

    Ok(test_output_vec)
}

pub fn load_configs() -> Vec<String> {
    let mut combiner = std::path::PathBuf::new();
    let get_current_dir = std::env::current_exe().unwrap().to_path_buf();
    let mut config_lines: Vec<String> = Vec::new();

    combiner.push(get_current_dir);
    combiner.push(std::path::PathBuf::from("config.txt"));

    for i in std::fs::read_to_string(combiner).unwrap().lines() {
        config_lines.push(i.to_string());
    }
    config_lines
}
