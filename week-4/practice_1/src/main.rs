//rust program to output name and age

use std::io;

fn main() {
	println!("student information management system");


	//input name
	println!("please enter your name");
	let mut name = String::new();
		io::stdin()
		.read_line(&mut name)
		.expect("failed to read input");
	println!("your name is: {}", name);

	//input age
	println!("enter your age.");
	let mut age = String::new();
		io::stdin().read_line(&mut age).expect("failed to read input");
	let age:i32 = age.trim().parse().expect("input not an integer");
	println!("your age is: {}", age);
	}
