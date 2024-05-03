pub fn load_configs(
    config_string: String,
) -> std::result::Result<(Vec<String>, Vec<String>), std::io::Error> {
    let mut combiner = std::path::PathBuf::new();
    let get_current_dir = std::env::current_exe().unwrap().to_path_buf();
    let mut config_lines: Vec<String> = Vec::new();
    let mut file_transfer: Vec<String> = Vec::new();

    combiner.push(get_current_dir);
    combiner.pop();
    combiner.push(std::path::PathBuf::from(config_string));

    for (x, i) in std::fs::read_to_string(combiner)
        .unwrap()
        .lines()
        .enumerate()
    {
        if x <= 1 {
            config_lines.push(i.to_string());
        } else {
            file_transfer.push(i.to_string());
        }
    }
    Ok((config_lines, file_transfer))
}
pub fn copy_over(dir_from: String, dir_to: String) -> std::result::Result<(), std::io::Error> {
    if cfg!(unix) {
        let mut com = std::process::Command::new("cp");

        println!("Starting copy, please be patient, as this might take a while depending on the amount of files you are trying to copy");

        com.current_dir("/");
        com.arg("-r")
            .arg(&dir_from)
            .arg(&dir_to)
            .output()
            .expect("Fail!");

        println!("Done copying.");
    }
    Ok(())
}
