use std::fs::File;
use std::io::{Read, Write};

fn main() {
    // create file and write into it
    let mut file = File::create("Welcome_message.txt").unwrap();
    file.write_all(b"Hello from the file!").unwrap();

    // now read it
    let mut contents = String::new();
    let mut file = File::open("Welcome_message.txt").unwrap();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);
}
