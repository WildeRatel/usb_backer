pub fn load_configs(config_string: String) -> std::result::Result<Vec<String>, std::io::Error> {
    let mut combiner = std::path::PathBuf::new();
    let get_current_dir = std::env::current_exe().unwrap().to_path_buf();
    let mut config_lines: Vec<String> = Vec::new();

    combiner.push(get_current_dir);
    combiner.pop();
    combiner.push(std::path::PathBuf::from(config_string));

    for i in std::fs::read_to_string(combiner).unwrap().lines() {
        config_lines.push(i.to_string());
    }
    Ok(config_lines)
}
