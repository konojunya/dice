use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("1.png").expect("can not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("can not read string");

    println!("{}", &contents);
}
