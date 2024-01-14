use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut file: File = File::open("example.json").expect("Json file can not be opened");
    let mut json_str: String = String::new();

    file.read_to_string(&mut json_str).expect("Can not load the file into string");

    return json_str;
}

fn main() {
    println!("{}", read_file());
}
