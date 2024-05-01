fn main() {
    usb_backer::load_configs();

    if let Err(e) = usb_backer::test_for_drive() {
        panic!("Couldn't find drive: {e}")
    } else {
        let output_vec = usb_backer::test_for_drive().unwrap();

        for i in output_vec.iter() {
            print!("{}", i);
        }
    }
}
