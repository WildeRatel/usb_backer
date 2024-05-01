fn main() {
    if let Ok(config_lines) = usb_backer::load_configs(String::from("config.txt")) {
        let mut dir_from = String::new();
        dir_from.push_str("");
        dir_from.push('"');
        dir_from.push_str("");
        dir_from.push_str(&config_lines[0]);
        dir_from.push_str("");
        dir_from.push_str(&config_lines[1]);
        dir_from.push('"');

        let mut dir_to = String::new();
        dir_to.push_str("");
        dir_to.push('"');
        dir_to.push_str("");
        dir_to.push_str(&config_lines[2]);
        dir_to.push_str("");
        dir_to.push('"');

        usb_backer::copy_over(dir_from, dir_to).expect("Failed to copy!");
    }
}
