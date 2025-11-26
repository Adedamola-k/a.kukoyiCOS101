use std::fs;
use std::io::Write;

fn main() {
    // Create the file
    let mut file = fs::File::create("data.txt").expect("could not create file");

    // Write something simple into it
    file.write_all(b"hello").expect("could not write");

    println!("File created.");

    // Delete the file
    fs::remove_file("data.txt").expect("could not remove file");

    println!("File removed.");
}
