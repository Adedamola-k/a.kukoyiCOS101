//rust program that takes an input the experience and age of an employee

use std::io;

fn main() 
{
	println!("Annual incentive calculator");
	
	println!("Experience level? (if experienced write 'experienced', if not experienced write 'not experienced')");
	let mut experience = String::new();
	io::stdin().read_line(&mut experience).expect("not a valid String");
	let experience = experience.trim().to_lowercase();

	println!("what is your age");
	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("not a valid string");
	let age:u32 = input2.trim().parse().expect("not a valid number");

	if experience == "experienced" && age >= 40
	{
		println!("your incentive is 1,560,000 per month");
	}

	else if experience == "experienced" && age >= 30 && age < 40
	{
		println!("your incentive is 1,480,00 per month");
	} 

	else if experience == "experienced" && age < 28 
	{
		println!("your incentive is 1,300,000 per month");
	}

	else if experience == "not experienced" 
	{
		println!("your incentive is 100,000");
	}
	else {
		println!("input does not match the incentive rules");
	}
}