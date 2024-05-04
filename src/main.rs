fn main() {
    if let Ok((config_lines, file_transfer)) = usb_backer::load_configs(String::from("config.txt"))
    {
        println!("Starting copy, please be patient, as this might take a while depending on the amount of files you are trying to copy");

        for (i, x) in file_transfer.iter().enumerate() {
            println!("{}", x);

            if i == file_transfer.len() - 1 {
                break;
            }

            let mut dir_from = String::new();
            dir_from.push_str(&config_lines[0]);
            dir_from.push_str("");
            dir_from.push_str(&config_lines[1]);

            let mut dir_to = String::new();
            dir_to.push_str(&config_lines[i]);

            if cfg!(unix) {
                usb_backer::copy_over(dir_from, dir_to).expect("Failed to copy!");
            }
        }
        println!("Done copying.");
    }
}
