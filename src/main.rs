fn main() {
    let test_output = String::from_utf8(test_for_drive.stdout).unwrap();
    let test_output_vec: Vec<String> = test_output
        .split('\n')
        .map(|x| str::to_string(x).clone())
        .collect();

    for i in test_output_vec.iter() {
        print!("{}", i);
    }
}
