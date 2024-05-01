pub fn test_for_drive() {
    let mut test_for_drive = std::process::Command::new("ls");
    test_for_drive.output();
}
