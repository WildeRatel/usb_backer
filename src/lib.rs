pub fn test_for_drive() -> std::result::Result<Vec<String>, std::io::Error> {
    let test_for_drive = std::process::Command::new("ls").output()?;
    let test_output = String::from_utf8(test_for_drive.stdout).unwrap();
    let test_output_vec: Vec<String> = test_output
        .split('\n')
        .map(|x| str::to_string(x).clone())
        .collect();

    Ok(test_output_vec)
}

pub fn load_configs() {
    let current_dir: String = std::env::current_exe()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    println!("{current_dir}")
}
