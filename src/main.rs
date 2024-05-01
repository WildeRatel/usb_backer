fn main() {
    if let Ok(config_lines) = usb_backer::load_configs(String::from("config.txt")) {
        println!("{}", config_lines[0].clone());
    }
}
